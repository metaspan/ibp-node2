//! IBP Service Pallet

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{ Decode, Encode, MaxEncodedLen };
use scale_info::{self, TypeInfo};
use frame_support::{
    traits::ConstU32,
    BoundedVec,
};

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

pub type ServiceId = BoundedVec<u8, ConstU32<32>>;
pub type ChainId = BoundedVec<u8, ConstU32<32>>;

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum ServiceType {
    None = 0,
    RPC = 1,
    BOOT = 2,
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum ServiceStatus {
    None = 0,
    Locked = 1,
    Active = 2,
    Chilled = 3,
    Deleted = 9,
}

// #[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
// pub enum ServiceMembershipLevel {
//     None = 0,
//     One = 1,
//     Two = 2,
//     Three = 3,
//     Four = 4,
//     Five = 5,
//     Six = 6,
// }

impl Default for ServiceType {
    fn default() -> Self {
        ServiceType::RPC
    }
}

impl Default for ServiceStatus {
    fn default() -> Self {
        ServiceStatus::Active
    }
}

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    // use pallet_ibp_member::Pallet as MembersPallet; // Import the Members pallet
    use pallet_ibp_member::Pallet as MemberPallet;
    use pallet_ibp_member::MembershipLevel;

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
    pub trait Config: frame_system::Config + pallet_ibp_member::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
    }

    #[derive(Debug, Encode, Decode, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct ServiceData<T: Config> {
        // service Id: unique identifier for the service, string
        // example: polkadot_rpc
        pub id: ServiceId,
        // chainId: unique identifier for the chain, string
        // example: polkadot
        pub chain_id: ChainId,
        // service type; enum ['RPC', 'BOOT']
        pub service_type: ServiceType,
        // level of membership where service applies
        pub level: MembershipLevel,
        // service status
        pub status: ServiceStatus,
        // PhantomData - not stored in storage
        #[codec(skip)]
        pub _marker: PhantomData<T>,
    }

    impl<T:Config> Default for ServiceData<T> {
        fn default() -> Self {
            Self {
                id: ServiceId::default(),
                chain_id: ChainId::default(),
                service_type: ServiceType::None,
                status: ServiceStatus::None,
                level: MembershipLevel::None,
                _marker: PhantomData,
            }
        }
    }
    
    impl<T: Config> ServiceData<T> {
        pub fn ok_or(self, err: Error<T>) -> Result<Self, Error<T>> {
            if self.id == ServiceId::default() {
                Err(err)
            } else {
                Ok(self)
            }
        }
    }
        
    /// storage for this pallet.
    #[pallet::storage]
    #[pallet::getter(fn services)]
    pub type Services<T: Config> = StorageMap<_, Blake2_128Concat, ServiceId, ServiceData<T>, ValueQuery>;

    // read the curators from the members pallet
    // #[pallet::storage]
    // pub type Curators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

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
        ServiceRegistered(ServiceId, MembershipLevel),
        // ServiceCreated(ServiceId, ServiceMembershipLevel),
        ServiceStatusUpdated(ServiceId, ServiceStatus),
        ServiceMembershipLevelUpdated(ServiceId, MembershipLevel),
        ServiceLocked(ServiceId),
        ServiceUnlocked(ServiceId),
        ServiceChilled(ServiceId),
        ServiceUnChilled(ServiceId),
        ServiceDeleted(ServiceId),
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
        // /// The value retrieved was `None` as no value was previously set.
        // NoneValue,
        // /// There was an attempt to increment the value in storage over `u32::MAX`.
        // StorageOverflow,
        ServiceAlreadyExists,
        ServiceNotFound,
        InvalidStatusTransition,
        NotACurator,
        CuratorAlreadyExists,
        CuratorLimitReached,
        CannotRemoveLastCurator,
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
        pub fn register_service( origin: OriginFor<T>, 
            id: ServiceId,
            chain_id: ChainId,
            service_type: ServiceType,
            level: MembershipLevel,
            status: ServiceStatus
        ) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(!Services::<T>::contains_key(&id), Error::<T>::ServiceAlreadyExists);
            // only curators can register services
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            let service_id = id.clone();
            // let level = ServiceMembershipLevel::Zero;
            // let status = ServiceStatus::Locked;
            // let service_id = id.clone();
            let service_data = ServiceData::<T> {
                id: service_id.clone(),
                chain_id,
                service_type,
                level: level.clone(),
                status,
                _marker: PhantomData,
            };
            Services::<T>::insert(service_id, service_data);
            Self::deposit_event(Event::ServiceRegistered(id, level));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn lock_service(origin: OriginFor<T>, service_id: ServiceId) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            // only curators can lock services
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
            Services::<T>::try_mutate(&service_id, |data_option| -> DispatchResult {
                // check if the service exists
                let data = data_option; // .as_mut(); //.ok_or(Error::<T, I>::ServiceNotFound)?;
                ensure!(data.status != ServiceStatus::Deleted, Error::<T>::InvalidStatusTransition);
                data.status = ServiceStatus::Locked;
                Self::deposit_event(Event::ServiceStatusUpdated(service_id.clone(), ServiceStatus::Locked));
                Ok(())
            })?;
            Ok(())
        }
        
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn unlock_service(origin: OriginFor<T>, service_id: ServiceId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // only curators can unlock services
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
            Services::<T>::try_mutate(&service_id, |data_opt| -> DispatchResult {
                //ensure!(status != None, Error::<T>::ServiceNotFound);
                let data = data_opt; // .as_mut(); //.ok_or(Error::<T>::ServiceNotFound)?;
                ensure!(data.status == ServiceStatus::Locked, Error::<T>::InvalidStatusTransition);
                data.status = ServiceStatus::Active;
                Self::deposit_event(Event::ServiceStatusUpdated(service_id.clone(), ServiceStatus::Active));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn delete_service(origin: OriginFor<T>, service_id: ServiceId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // only curators can delete services
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
            Services::<T>::try_mutate(&service_id, |data_opt| -> DispatchResult {
                let data = data_opt; // .as_mut(); // .ok_or(Error::<T>::ServiceNotFound)?;
                data.status = ServiceStatus::Deleted;
                Self::deposit_event(Event::ServiceDeleted(service_id.clone()));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn undelete_service(origin: OriginFor<T>, service_id: ServiceId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // only curators can undelete services
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
            Services::<T>::try_mutate(&service_id, |data_opt| -> DispatchResult {            
                let data = data_opt; // .as_mut(); // .ok_or(Error::<T>::ServiceNotFound)?;
                ensure!(data.status == ServiceStatus::Deleted, Error::<T>::InvalidStatusTransition);
                data.status = ServiceStatus::Deleted;
                Self::deposit_event(Event::ServiceStatusUpdated(service_id.clone(), ServiceStatus::Deleted));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn update_service_level(origin: OriginFor<T>, service_id: ServiceId, level: MembershipLevel) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // only curators can change service levels
            ensure!(MemberPallet::<T>::curators(&sender), Error::<T>::NotACurator);
            ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
            Services::<T>::try_mutate(&service_id, |data_opt| -> DispatchResult {
                let data = data_opt; // .as_mut(); // .ok_or(Error::<T>::ServiceNotFound)?;
                data.level = level.clone();
                Self::deposit_event(Event::ServiceMembershipLevelUpdated(service_id.clone(), level.clone()));
                Ok(())
            })?;
            Ok(())
        }

        // /// Each service has a required membershipLevel.
        // /// Depending on the member.level return the list of services that the member should provide.
        // #[pallet::call_index(6)]
        // #[pallet::weight(10_000)]
        // pub fn service_members(origin: OriginFor<T>, service_id: ServiceId) -> DispatchResult {
        //     let _sender = ensure_signed(origin)?;
        //     ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
        //     let service: ServiceData = Services::<T>::get(&service_id); // .ok_or(Error::<T>::ServiceNotFound)?;
        //     // empty vec to hold the list of providers
        //     let providers: Vec<(T::AccountId, ServiceMembershipLevel)> = Vec::new();
        //     let members: Vec<(T::AccountId, ServiceMembershipLevel)> = MemberPallet::<T>::members().iter().collect();

        //     for (service_id, status, level) in services {
        //         // This is just for demonstration; in real use case, you would return this data.
        //         // sp_runtime::print::info!("{:?}: {:?}", service_id, status);
        //     }
        //     Ok(())
        // }

        // #[pallet::call_index(7)]
        // #[pallet::weight(10_000)]
        // pub fn get_service_details(origin: OriginFor<T>, service_id: ServiceId) -> DispatchResult {
        //     let _sender = ensure_signed(origin)?;
        //     let service_data = Services::<T>::get(&service_id);
        //     if service_data.is_none() {
        //       return Err(Error::<T>::ServiceNotFound.into());
        //     }
        //     // let (status, level) = Services::get(&service_id).ok_or(Error::<T>::ServiceNotFound)?;
        //     // This is just for demonstration; in real use case, you would return this data.
        //     // sp_runtime::print::info!("{:?}: {:?}", service_id, status, level);
        //     Ok(())
        // }

    }
}

pub use pallet::*;
