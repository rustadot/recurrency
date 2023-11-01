//! Types for the Capacity Pallet
use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use log::warn;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{CheckedAdd, CheckedSub, Saturating, Zero},
	RuntimeDebug,
};

#[cfg(any(feature = "runtime-benchmarks", test))]
use sp_std::vec::Vec;

/// The type used for storing information about staking details.
#[derive(
	TypeInfo, RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, Clone, Decode, Encode, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct StakingAccountDetails<T: Config> {
	/// The amount a Staker has staked, minus the sum of all tokens in `unlocking`.
	pub active: BalanceOf<T>,
	/// The total amount of tokens in `active` and `unlocking`
	pub total: BalanceOf<T>,
	/// Unstaked balances that are thawing or awaiting withdrawal.
	pub unlocking: BoundedVec<UnlockChunk<BalanceOf<T>, T::EpochNumber>, T::MaxUnlockingChunks>,
}
/// The type that is used to record a single request for a number of tokens to be unlocked.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UnlockChunk<Balance, EpochNumber> {
	/// Amount to be unlocked.
	pub value: Balance,
	/// Block number at which point funds are unlocked.
	pub thaw_at: EpochNumber,
}

impl<T: Config> StakingAccountDetails<T> {
	/// Increases total and active balances by an amount.
	pub fn deposit(&mut self, amount: BalanceOf<T>) -> Option<()> {
		self.total = amount.checked_add(&self.total)?;
		self.active = amount.checked_add(&self.active)?;

		Some(())
	}

	/// Calculates a stakable amount from a proposed amount.
	pub fn get_stakable_amount_for(
		&self,
		staker: &T::AccountId,
		proposed_amount: BalanceOf<T>,
	) -> BalanceOf<T> {
		let account_balance = T::Currency::free_balance(&staker);
		let available_staking_balance = account_balance.saturating_sub(self.total);
		available_staking_balance
			.saturating_sub(T::MinimumTokenBalance::get())
			.min(proposed_amount)
	}

	#[cfg(any(feature = "runtime-benchmarks", test))]
	#[allow(clippy::unwrap_used)]
	///  tmp fn for testing only
	/// set unlock chunks with (balance, thaw_at).  does not check that the unlock chunks
	/// don't exceed total.
	/// returns true on success, false on failure (?)
	pub fn set_unlock_chunks(&mut self, chunks: &Vec<(u32, u32)>) -> bool {
		let result: Vec<UnlockChunk<BalanceOf<T>, <T>::EpochNumber>> = chunks
			.into_iter()
			.map(|chunk| UnlockChunk { value: chunk.0.into(), thaw_at: chunk.1.into() })
			.collect();
		self.unlocking = BoundedVec::try_from(result).unwrap();
		self.unlocking.len() == chunks.len()
	}

	/// deletes thawed chunks, updates `total`, Caller is responsible for updating free/locked
	/// balance on the token account.
	/// Returns: the total amount reaped from `unlocking`
	pub fn reap_thawed(&mut self, current_epoch: <T>::EpochNumber) -> BalanceOf<T> {
		let mut total_reaped: BalanceOf<T> = 0u32.into();
		self.unlocking.retain(|chunk| {
			if current_epoch.ge(&chunk.thaw_at) {
				total_reaped = total_reaped.saturating_add(chunk.value);
				match self.total.checked_sub(&chunk.value) {
					Some(new_total) => self.total = new_total,
					None => warn!(
						"Underflow when subtracting {:?} from staking total {:?}",
						chunk.value, self.total
					),
				}
				false
			} else {
				true
			}
		});
		total_reaped
	}

	/// Decrease the amount of active stake by an amount and create an UnlockChunk.
	pub fn withdraw(
		&mut self,
		amount: BalanceOf<T>,
		thaw_at: T::EpochNumber,
	) -> Result<BalanceOf<T>, DispatchError> {
		// let's check for an early exit before doing all these calcs
		ensure!(
			self.unlocking.len() < T::MaxUnlockingChunks::get() as usize,
			Error::<T>::MaxUnlockingChunksExceeded
		);
		let mut active = self.active.saturating_sub(amount);
		let mut actual_unstaked: BalanceOf<T> = amount;

		if active.le(&T::MinimumStakingAmount::get()) {
			actual_unstaked = amount.saturating_add(active);
			active = Zero::zero();
		}
		let unlock_chunk = UnlockChunk { value: actual_unstaked, thaw_at };

		// we've already done the check but it's fine, we need to handle possible errors.
		self.unlocking
			.try_push(unlock_chunk)
			.map_err(|_| Error::<T>::MaxUnlockingChunksExceeded)?;

		self.active = active;
		Ok(actual_unstaked)
	}
}

impl<T: Config> Default for StakingAccountDetails<T> {
	fn default() -> Self {
		Self { active: Zero::zero(), total: Zero::zero(), unlocking: Default::default() }
	}
}

/// Details about the total token amount targeted to an MSA.
/// The Capacity that the target will receive.
#[derive(PartialEq, Eq, Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct StakingTargetDetails<Balance> {
	/// The total amount of tokens that have been targeted to the MSA.
	pub amount: Balance,
	/// The total Capacity that an MSA received.
	pub capacity: Balance,
}

impl<Balance: Saturating + Copy + CheckedAdd> StakingTargetDetails<Balance> {
	/// Increase an MSA target Staking total and Capacity amount.
	pub fn deposit(&mut self, amount: Balance, capacity: Balance) -> Option<()> {
		self.amount = amount.checked_add(&self.amount)?;
		self.capacity = capacity.checked_add(&self.capacity)?;

		Some(())
	}

	/// Decrease an MSA target Staking total and Capacity amount.
	pub fn withdraw(&mut self, amount: Balance, capacity: Balance) {
		self.amount = self.amount.saturating_sub(amount);
		self.capacity = self.capacity.saturating_sub(capacity);
	}
}

/// The type for storing Registered Provider Capacity balance:
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CapacityDetails<Balance, EpochNumber> {
	/// The Capacity remaining for the `last_replenished_epoch`.
	pub remaining_capacity: Balance,
	/// The amount of tokens staked to an MSA.
	pub total_tokens_staked: Balance,
	/// The total Capacity issued to an MSA.
	pub total_capacity_issued: Balance,
	/// The last Epoch that an MSA was replenished with Capacity.
	pub last_replenished_epoch: EpochNumber,
}

impl<Balance, EpochNumber> CapacityDetails<Balance, EpochNumber>
where
	Balance: Saturating + Copy + CheckedAdd + CheckedSub,
	EpochNumber: Clone + PartialOrd + PartialEq,
{
	/// Increase a targets total Tokens staked and Capacity total issuance by an amount.
	/// To be called on a stake
	pub fn deposit(&mut self, amount: &Balance, capacity: &Balance) -> Option<()> {
		self.total_tokens_staked = amount.checked_add(&self.total_tokens_staked)?;
		self.remaining_capacity = capacity.checked_add(&self.remaining_capacity)?;
		self.total_capacity_issued = capacity.checked_add(&self.total_capacity_issued)?;

		// We do not touch last_replenished epoch here, because it would create a DoS vulnerability.
		// Since capacity is lazily replenished, an attacker could stake
		// a minimum amount, then at the very beginning of each epoch, stake a tiny additional amount,
		// thus preventing replenishment when "last_replenished_at" is checked on the next provider's
		// message.
		Some(())
	}

	/// Return whether capacity can be replenished, given the current epoch.
	pub fn can_replenish(&self, current_epoch: EpochNumber) -> bool {
		self.last_replenished_epoch.lt(&current_epoch)
	}

	/// Completely refill all available capacity.
	/// To be called lazily when a Capacity message is sent in a new epoch.
	pub fn replenish_all(&mut self, current_epoch: &EpochNumber) {
		self.remaining_capacity = self.total_capacity_issued;
		self.last_replenished_epoch = current_epoch.clone();
	}

	/// Replenish remaining capacity by the provided amount and
	/// touch last_replenished_epoch with the current epoch.
	pub fn replenish_by_amount(&mut self, amount: Balance, current_epoch: &EpochNumber) {
		self.remaining_capacity = amount.saturating_add(self.remaining_capacity);
		self.last_replenished_epoch = current_epoch.clone();
	}

	/// Deduct the given amount from the remaining capacity that can be used to pay for messages.
	/// To be called when a message is paid for with capacity.
	pub fn deduct_capacity_by_amount(&mut self, amount: Balance) -> Result<(), ArithmeticError> {
		let new_remaining =
			self.remaining_capacity.checked_sub(&amount).ok_or(ArithmeticError::Underflow)?;
		self.remaining_capacity = new_remaining;
		Ok(())
	}

	/// Decrease a target's total available capacity.
	/// To be called on an unstake.
	pub fn withdraw(&mut self, capacity_deduction: Balance, tokens_staked_deduction: Balance) {
		self.total_tokens_staked = self.total_tokens_staked.saturating_sub(tokens_staked_deduction);
		self.total_capacity_issued = self.total_capacity_issued.saturating_sub(capacity_deduction);
		self.remaining_capacity = self.remaining_capacity.saturating_sub(capacity_deduction);
	}
}

/// The type for storing details about an epoch.
/// May evolve to store other needed data such as epoch_end.
#[derive(
	PartialEq, Eq, Clone, Default, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen,
)]
pub struct EpochInfo<BlockNumber> {
	/// The block number when this epoch started.
	pub epoch_start: BlockNumber,
}
