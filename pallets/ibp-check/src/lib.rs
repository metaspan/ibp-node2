//! IBP HealthCheck Pallet

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{ Decode, Encode, MaxEncodedLen };
use scale_info::{self, TypeInfo};
use frame_support::{
    traits::ConstU32,
    BoundedVec,
};

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

// pub type ServiceId = BoundedVec<u8, ConstU32<32>>;
pub type DomainId = BoundedVec<u8, ConstU32<32>>;

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum HealthCheckType {
    RPC = 0,
    BOOT = 1,
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum HealthCheckStatus {
    Error = 0,
    Active = 1,
    Chilled = 2,
}

impl Default for HealthCheckType {
  fn default() -> Self {
    HealthCheckType::RPC
  }
}

impl Default for HealthCheckStatus {
  fn default() -> Self {
    HealthCheckStatus::Active
  }
}

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

    // use AccountId;
    // use pallet_ibp_member::MemberId;
    // use pallet_ibp_member::MemberId;
    use pallet_ibp_member::Error::{MemberNotFound};
    use pallet_ibp_member::{MembershipLevel};
    
    use pallet_ibp_service::{ServiceId, ServiceType, ServiceStatus};
    use pallet_ibp_service::Error::{ServiceNotFound};

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
		type WeightInfo: WeightInfo;
	}

    #[derive(Default, Debug, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct HealthCheckData<T: Config> {
        // member Id: unique identifier for the member, string
        pub member_id: T::AccountId,
        // domain Id: unique identifier for the domain, string
        pub domain_id: DomainId,
        // service Id: unique identifier for the service, string
        pub service_id: ServiceId,
        // service type; enum ['RPC', 'BOOT']
        pub check_type: HealthCheckType,
        // level of membership where service applies
        pub level: MembershipLevel,
        // service status
        pub status: HealthCheckStatus,
    }

    // impl HealthCheckData {
    //     fn as_mut(&mut self) -> &mut HealthCheckData {
    //         self
    //     }
    // }

    // /// storage for this pallet.
    // #[pallet::storage]
    // pub type Checks<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, HealthCheckData<T>, ValueQuery>;

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
        HealthCheckRegistered(T::AccountId, ServiceId, HealthCheckType, HealthCheckStatus),
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
        NotAMonitor,
        MemberNotFound,
        ServiceNotFound,
        NotACurator,
        // check for service membership level
        ServiceMembershipLevelMismatch,
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
        pub fn register_health_check(
            origin: OriginFor<T>, 
            member_id: T::AccountId,
            service_id: ServiceId,
            domain_id: DomainId,
            check_type: HealthCheckType,
            // level: ServiceMembershipLevel,
            status: HealthCheckStatus
        ) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            // only monitors can register health checks
            ensure!(MemberPallet::<T>::monitors(&sender), Error::<T>::NotAMonitor);
            // ensure member exists
            // ensure!(MemberPallet::<T>::members(&member_id), Error::<T>::MemberNotFound);
            let member = MemberPallet::<T>::members(&member_id).ok_or(MemberNotFound)?;
            // ensure service exists
            // ensure!(ServicePallet::<T>::services(&service_id), ServiceNotFound);
            let service = ServicePallet::<T>::services(&service_id).ok_or(ServiceNotFound)?;
            // check member.level GE service.membershipLevel
            // ensure!(member.level >= service.level, Error::<T>::ServiceMembershipLevelMismatch);
            // let level = ServiceMembershipLevel::Zero;
            // let status = ServiceStatus::Locked;
            // let service_id = id.clone();
            let _check_data: HealthCheckData<T> = HealthCheckData {
                member_id: member_id.clone(),
                service_id: service_id.clone(),
                domain_id,
                check_type: check_type.clone(),
                level: service.level,
                status: status.clone(),
            };
            // Services::<T>::insert(id.clone(), service_data);
            Self::deposit_event(Event::HealthCheckRegistered(member_id, service_id, check_type, status));
            Ok(())
        }

	}
}
