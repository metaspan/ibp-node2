
use super::*;
use sp_staking::{Agent, DelegationInterface, DelegationMigrator, Delegator, OnStakingUpdate};

impl<T: Config> DelegationInterface for Pallet<T> {
	// type Balance = BalanceOf<T>;
	type AccountId = T::AccountId;

}
