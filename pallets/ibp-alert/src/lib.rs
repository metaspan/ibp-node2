//! IBP HealthCheck Pallet

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{ Decode, Encode, MaxEncodedLen };
use scale_info::TypeInfo;
use frame_support::{
    traits::ConstU32,
    BoundedVec,
};

/// implement jobs at the end of each session
use pallet_session::{SessionManager, ShouldEndSession};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use pallet_ibp_member::Pallet as MemberPallet;
use pallet_ibp_service::Pallet as ServicePallet;

pub type DomainId = BoundedVec<u8, ConstU32<32>>;
pub type AlertType = BoundedVec<u8, ConstU32<32>>;

// #[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
// pub enum AlertType {
//     HostDown,
//     ServiceDown,
//     InstanceDown,
//     BlackboxProbeFailed,
//     BlackboxProbeHttpFailure,
//     NoIncomingConnection,
//     BlockFinalizationLaggingBehind,
//     BlackboxSslCertificateWillExpireSoon,
//     SubstrateServiceTooManyRestarts,
//     Other,
// }

// impl Default for AlertType {
//     fn default() -> Self {
//       AlertType::Other
//     }
// }


// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use pallet_ibp_service::{ServiceId, ServiceStatus};
    use pallet_ibp_service::Error::ServiceNotFound;
    use pallet_ibp_member::MemberStatus;
    use pallet_ibp_member::Error::MemberNotFound;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_ibp_member::Config + pallet_ibp_service::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        // type WeightInfo: WeightInfo;
        type AlertKey: Parameter + Member + TypeInfo + MaxEncodedLen;
    }

    #[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct AlertKey<T: Config> {
        monitor_id: T::AccountId,
        alert_id: u64
    }

    #[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct AlertData<T: Config> {
        // monitor Id: unique identifier for the monitor, string
        pub monitor_id: T::AccountId,
        // external alert Id: unique identifier for the alert, string(maxlen = 64)
        pub alert_id: u64,
        // member Id: unique identifier for the member, string
        pub member_id: T::AccountId,
        // domain Id: unique identifier for the domain, string
        pub domain_id: DomainId,
        // service Id: unique identifier for the service, string
        pub service_id: ServiceId,
        // service type; enum ['RPC', 'BOOT']
        pub alert_type: AlertType
    }

    /// storage for this pallet. 
    #[pallet::storage]
    #[pallet::getter(fn alerts)]
    pub type Alerts<T: Config> = StorageMap<_, Blake2_128Concat, AlertKey<T>, AlertData<T>, OptionQuery>;

    /// for fast access - check if alert exists
    #[pallet::storage]
    #[pallet::getter(fn alert_index)]
    pub type AlertIndex<T: Config> = StorageMap<_, Blake2_128Concat, (T::AccountId, ServiceId, AlertType), u64, OptionQuery>;

    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AlertRegistered(T::AccountId, ServiceId, DomainId, AlertType),
        AlertCleared(T::AccountId, ServiceId, DomainId, AlertType),
        AlertIndexGenerated(),
    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        AlertNotFound,
        MemberNotFound,
        MemberNotActive,
        ServiceNotFound,
        ServiceNotActive,
        // Must be monitor to register alerts
        NotAMonitor,
        // Must be curator to clear alerts
        NotACurator,
        // Must be original monitor or curator to clear alerts
        BadOriginOrNotACurator,
        // check for service membership level
        ServiceMembershipLevelMismatch,
        MemberServiceAlertExists,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_alert(
            origin: OriginFor<T>, 
            alert_id: u64,
            member_id: T::AccountId,
            service_id: ServiceId,
            domain_id: DomainId,
            alert_type: AlertType
        ) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            // only monitors can register health checks
            ensure!(MemberPallet::<T>::monitors(&sender), Error::<T>::NotAMonitor);
            // ensure member exists
            let member = MemberPallet::<T>::members(&member_id).ok_or(MemberNotFound)?;
            if member.status != MemberStatus::Active {
                return Err(Error::<T>::MemberNotActive.into());
            }
            // ensure service exists
            let service = ServicePallet::<T>::services(&service_id).ok_or(ServiceNotFound)?;
            if service.status != ServiceStatus::Active {
                return Err(Error::<T>::ServiceNotActive.into());
            }
            let alert_index = (member_id.clone(), service_id.clone(), alert_type.clone());
            ensure!(!AlertIndex::<T>::contains_key(alert_index.clone()), Error::<T>::MemberServiceAlertExists);
            // check member.level GE service.membershipLevel
            // ensure!(member.level >= service.level, Error::<T>::ServiceMembershipLevelMismatch);
            // let level = ServiceMembershipLevel::Zero;
            // let status = ServiceStatus::Locked;
            // let service_id = id.clone();
            let alert_data: AlertData<T> = AlertData {
                alert_id,
                monitor_id: sender.clone(),
                member_id: member_id.clone(),
                service_id: service_id.clone(),
                domain_id: domain_id.clone(),
                alert_type: alert_type.clone()
            };
            let key: AlertKey<T> = AlertKey { monitor_id: sender.clone(), alert_id };
            // let key = ( sender.clone(), alert_id );
            // store the alert
            Alerts::<T>::insert(key, alert_data);
            // update alert index
            AlertIndex::<T>::insert(alert_index, 1);
            Self::deposit_event(Event::AlertRegistered(member_id, service_id, domain_id, alert_type));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn clear_alert(
            origin: OriginFor<T>, 
            alert_id: u64
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // get the alert by (monitor, Id)
            let key: AlertKey<T> = AlertKey { monitor_id: sender.clone(), alert_id };
            // let key = ( sender.clone(), alert_id );
            let alert: AlertData<T> = Alerts::<T>::get(key.clone()).ok_or(Error::<T>::AlertNotFound)?;
            // only original monitor or curator can clear alerts
            ensure!(
                alert.monitor_id == sender || MemberPallet::<T>::curators(&sender), 
                Error::<T>::BadOriginOrNotACurator
            );
            // delete the alert
            Alerts::<T>::remove(key);
            // delete alert index
            let alert_index = (sender.clone(), alert.service_id.clone(), alert.alert_type.clone());
            AlertIndex::<T>::remove(alert_index);
            Self::deposit_event(Event::AlertCleared(sender, alert.service_id, alert.domain_id, alert.alert_type));
            Ok(())
        }

        #[pallet::call_index(20)]
        #[pallet::weight(10_000)]
        pub fn clear_monitor_alert(
            origin: OriginFor<T>, 
            monitor_id: T::AccountId,
            alert_id: u64
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // only curators can force clear alerts
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            // get the alert by (monitor, Id)
            let key: AlertKey<T> = AlertKey { monitor_id: monitor_id.clone(), alert_id };
            // let key = ( monitor_id, alert_id );
            let alert: AlertData<T> = Alerts::<T>::get(key.clone()).ok_or(Error::<T>::AlertNotFound)?;
            // delete the alert
            Alerts::<T>::remove(key);
            // delete alert index
            let alert_index = (monitor_id.clone(), alert.service_id.clone(), alert.alert_type.clone());
            AlertIndex::<T>::remove(alert_index);
            Self::deposit_event(Event::AlertCleared(sender, alert.service_id, alert.domain_id, alert.alert_type));
            Ok(())
        }

    } // impl<T: Config> Pallet<T>

    use pallet_session::SessionManager;
    extern crate alloc;
    use log::info;
    use alloc::vec::Vec;
    use frame_support::pallet_prelude::PhantomData;

    /// Session Manager
    pub struct IbpSessionManager<T>(PhantomData<T>);

    impl<T: Config> SessionManager<T::AccountId> for IbpSessionManager<T> {

        fn new_session(_new_index: u32) -> Option<Vec<T::AccountId>> {
            None
        }

        fn end_session(_end_index: u32) {
            // Insert your logic here
        }

        fn start_session(_start_index: u32) {
            info!("New session, recalculating AlertIndex");
            // clear the AlertIndex
            AlertIndex::<T>::remove_all(None);
            // recalculate the AlertIndex (to be sure...)
            for (_, alert) in Alerts::<T>::iter() {
                let key = (alert.member_id.clone(), alert.service_id.clone(), alert.alert_type);
                AlertIndex::<T>::insert(key, 1);
            };
            Pallet::<T>::deposit_event(Event::AlertIndexGenerated());
        }
    }

}
