//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// temporary feature flag for AccountId Alias

use codec::{ Decode, Encode, MaxEncodedLen };
use scale_info::{self, TypeInfo};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// printing TODO remove me
// use sp_runtime::*;

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

#[derive(Clone, Copy, Debug, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum MemberStatus {
    None = 0,
    Locked = 1,
    Active = 2,
    Chilled = 3,
    Deleted = 9,
}

#[derive(Debug, Encode, Decode, Clone, Copy, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum MembershipLevel {
    None = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Default for MemberStatus {
    fn default() -> Self {
        MemberStatus::Active
    }
}

impl Default for MembershipLevel {
    fn default() -> Self {
        MembershipLevel::None
    }
}

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    // import AccountId from frame_system
    // use frame_system::Config as SystemConfig;

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
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        // type MemberId: Parameter + Member + MaybeSerializeDeserialize + Default + MaxEncodedLen + TypeInfo;
    }

    pub type MemberIdOf<T> = <T as frame_system::Config>::AccountId;

    // pub type MemberData = (MemberStatus, MembershipLevel);
    #[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct MemberData<T: Config> {
        pub id: Option<T::AccountId>,
        pub status: MemberStatus,
        pub level: MembershipLevel,
    }

    impl<T: Config> Default for MemberData<T> {
        fn default() -> Self {
            Self {
                id: None,
                status: Default::default(), // MemberStatus::Active,
                level: Default::default(), // MembershipLevel::None,
            }
        }
        // fn is_none (&self) -> bool {
        //     self.id.is_none()
        // }
        // fn as_mut(&mut self) -> &mut MemberData<T> {
        //     self
        // }
    }

    impl<T: Config> MemberData<T> {
        pub fn ok_or(self, err: Error<T>) -> Result<Self, Error<T>> {
            if self.id.is_none() {
                Err(err)
            } else {
                Ok(self)
            }
        }
    }

    /// A storage item for this pallet.
    ///
    /// In this template, we are declaring a storage item called `Something` that stores a single
    /// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub type Members<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, MemberData<T>, ValueQuery>;

    /// Curators are allowed to update the status and level of a member and service
    #[pallet::storage]
    #[pallet::getter(fn curators)] // getter function for the storage item
    pub(super) type Curators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    /// Monitors are allowed to submit HealthChecks
    #[pallet::storage]
    #[pallet::getter(fn monitors)] // getter function for the storage item
    pub(super) type Monitors<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    #[pallet::storage]
    pub type CuratorCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    // /// GenesisConfig
    // #[pallet::genesis_config]
    // pub struct GenesisConfig {
    //     pub members: Vec<(T::AccountId, MemberData)>,
    //     pub curators: Vec<T::AccountId>,
    // }

    // #[pallet::genesis_build]
    // impl<T: Config> GenesisBuild<T> for GenesisConfig {
    //     fn build(&self) {
    //         // add ALICE as the first curator
    //         Curators::<T>::insert(&self.curators[0], true);
    //     }
    // }

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
        MemberRegistered(T::AccountId),
        MemberStatusUpdated(T::AccountId, MemberStatus, MemberStatus),
        MembershipLevelUpdated(T::AccountId, MembershipLevel, MembershipLevel),
        MemberCreated(T::AccountId),
        // MemberLocked(T::AccountId),
        // MemberUnlocked(T::AccountId),
        MemberChilled(T::AccountId),
        MemberUnChilled(T::AccountId),
        MemberDeleted(T::AccountId),
        // events for curator
        CuratorAssigned(T::AccountId),
        CuratorRemoved(T::AccountId),
        // events for monitor
        MonitorAssigned(T::AccountId),
        MonitorRemoved(T::AccountId),
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
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
        MemberAlreadyExists,
        MemberNotFound,
        InvalidStatusTransition,
        NotACurator,
        CuratorAlreadyExists,
        CuratorLimitReached,
        CannotRemoveLastCurator,
        NotAMonitor,
        MonitorAlreadyExists,
        MonitorLimitReached,
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

        /// Register a new member
        /// The member will be level 0 and status locked
        /// Only curators can update the level or status of a member
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_member( origin: OriginFor<T> ) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(!Members::<T>::contains_key(&sender), Error::<T>::MemberAlreadyExists);
            let member_id = sender.clone();
            let level = MembershipLevel::None;
            let status = MemberStatus::Locked;
            let member_data = MemberData { id: Some(member_id), status, level };
            Members::<T>::insert(&sender, member_data);
            Self::deposit_event(Event::MemberRegistered(sender));
            Ok(())
        }

        /// voluntary action by the member
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn chill_member(origin: OriginFor<T>) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(Members::<T>::contains_key(&sender), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&sender, |data_option| -> DispatchResult {
                // check if the member exists
                let data = data_option; // .as_mut(); //.ok_or(Error::<T, I>::MemberNotFound)?;
                ensure!(data.status != MemberStatus::Deleted, Error::<T>::InvalidStatusTransition);
                ensure!(data.status != MemberStatus::Locked, Error::<T>::InvalidStatusTransition);
                let prev_status = data.status;
                data.status = MemberStatus::Chilled;
                Self::deposit_event(Event::MemberStatusUpdated(sender.clone(), prev_status, MemberStatus::Chilled));
                Ok(())
            })?;
            Ok(())
        }
        
        /// voluntary action by the member
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn unchill_member(origin: OriginFor<T>) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(Members::<T>::contains_key(&sender), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&sender, |data_option| -> DispatchResult {
                // check if the member exists
                let data = data_option; // .as_mut(); //.ok_or(Error::<T, I>::MemberNotFound)?;
                ensure!(data.status == MemberStatus::Chilled, Error::<T>::InvalidStatusTransition);
                let prev_status = data.status;
                data.status = MemberStatus::Active;
                Self::deposit_event(Event::MemberStatusUpdated(sender.clone(), prev_status, MemberStatus::Active));
                Ok(())
            })?;
            Ok(())
        }
        
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn lock_member(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            ensure!(Members::<T>::contains_key(&account_id), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&account_id, |data_option| -> DispatchResult {
                // check if the member exists
                let data = data_option; // .as_mut(); //.ok_or(Error::<T, I>::MemberNotFound)?;
                let prev_status = data.status;
                ensure!(data.status != MemberStatus::Deleted, Error::<T>::InvalidStatusTransition);
                data.status = MemberStatus::Locked;
                Self::deposit_event(Event::MemberStatusUpdated(account_id.clone(), prev_status, MemberStatus::Locked));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(20)]
        #[pallet::weight(10_000)]
        pub fn unlock_member(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            ensure!(Members::<T>::contains_key(&account_id), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&account_id, |data_opt| -> DispatchResult {
                //ensure!(status != None, Error::<T>::MemberNotFound);
                let data = data_opt; // .as_mut(); //.ok_or(Error::<T>::MemberNotFound)?;
                ensure!(data.status == MemberStatus::Locked, Error::<T>::InvalidStatusTransition);
                let prev_status = data.status;
                data.status = MemberStatus::Active;
                Self::deposit_event(Event::MemberStatusUpdated(account_id.clone(), prev_status, MemberStatus::Active));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(30)]
        #[pallet::weight(10_000)]
        pub fn delete_member(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            ensure!(Members::<T>::contains_key(&account_id), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&account_id, |data_opt| -> DispatchResult {
                let data = data_opt; // .as_mut(); // .ok_or(Error::<T>::MemberNotFound)?;
                let prev_status = data.status;
                data.status = MemberStatus::Deleted;
                Self::deposit_event(Event::MemberStatusUpdated(account_id.clone(), prev_status, MemberStatus::Deleted));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(40)]
        #[pallet::weight(10_000)]
        pub fn undelete_member(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            ensure!(Members::<T>::contains_key(&account_id), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&account_id, |data_opt| -> DispatchResult {            
                let data = data_opt; // .as_mut(); // .ok_or(Error::<T>::MemberNotFound)?;
                ensure!(data.status == MemberStatus::Deleted, Error::<T>::InvalidStatusTransition);
                let prev_status = data.status;
                data.status = MemberStatus::Deleted;
                Self::deposit_event(Event::MemberStatusUpdated(account_id.clone(), prev_status, MemberStatus::Deleted));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(50)]
        #[pallet::weight(10_000)]
        pub fn update_member_level(origin: OriginFor<T>, account_id: T::AccountId, level: MembershipLevel) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            ensure!(Members::<T>::contains_key(&account_id), Error::<T>::MemberNotFound);
            Members::<T>::try_mutate(&account_id, |data_opt| -> DispatchResult {
                let data = data_opt; // .as_mut(); // .ok_or(Error::<T>::MemberNotFound)?;
                let prev_level = data.level.clone();
                data.level = level.clone();
                Self::deposit_event(Event::MembershipLevelUpdated(account_id.clone(), prev_level.clone(), level.clone()));
                Ok(())
            })?;
            Ok(())
        }

        #[pallet::call_index(60)]
        #[pallet::weight(10_000)]
        pub fn assign_monitor(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // only members can be monitors
            ensure!(Members::<T>::contains_key(&account_id), Error::<T>::MemberNotFound);
            // only curators can assign monitors
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            Monitors::<T>::insert(&account_id, true);
            Self::deposit_event(Event::MonitorAssigned(account_id));
            Ok(())
        }

        #[pallet::call_index(70)]
        #[pallet::weight(10_000)]
        pub fn remove_monitor(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Curators::<T>::contains_key(&sender), Error::<T>::NotACurator);
            ensure!(Monitors::<T>::contains_key(&account_id), Error::<T>::NotAMonitor);
            Monitors::<T>::remove(&account_id);
            Self::deposit_event(Event::MonitorRemoved(account_id));
            Ok(())
        }

        #[pallet::call_index(80)]
        #[pallet::weight(10_000)]
        pub fn assign_curator(_origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            //ensure_root(origin)?;
            // ensure the sender is ALICE
            ensure!(!Curators::<T>::contains_key(&account_id), Error::<T>::CuratorAlreadyExists);
            ensure!(CuratorCount::<T>::get() < 5, Error::<T>::CuratorLimitReached);
            Curators::<T>::insert(&account_id, true);
            // CuratorCount::put(CuratorCount::get() + 1);
            Self::deposit_event(Event::CuratorAssigned(account_id));
            Ok(())
        }

        #[pallet::call_index(90)]
        #[pallet::weight(10_000)]
        pub fn remove_curator(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(Curators::<T>::contains_key(&account_id), Error::<T>::NotACurator);
            ensure!(CuratorCount::<T>::get() > 1, Error::<T>::CannotRemoveLastCurator);

            Curators::<T>::remove(&account_id);
            // CuratorCount::put(CuratorCount::get() - 1);
            Self::deposit_event(Event::CuratorRemoved(account_id));
            Ok(())
        }

    }
}
