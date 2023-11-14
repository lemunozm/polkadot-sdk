//! DO NOT GET SCARED about this file.
//! It's expected that almost every line of this file can be automatically generated
//! once mock-builder support procedural macros to build the mocked pallets.

/// With procedural support, it will be autogenerated as follows:
///
/// ```ignore
/// #[mock_builder::pallet]
/// mod mock_pallet_time {
///     trait Time {
///         type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen;
///         fn now() -> Self::Moment;
///     }
/// }
/// ```
///
/// It's expected that if you own the trait, you can simply put one procedural attribute on
/// top of it to autogerate the mocked pallet:
///
/// ```ignore
/// #[mock_builder::pallet]
/// trait MyOwnTrait { .. }
/// ```
#[frame_support::pallet(dev_mode)]
pub mod mock_pallet_time {
	use frame_support::{pallet_prelude::*, traits::Time};
	use mock_builder::{execute_call, register_call};
	use sp_runtime::traits::AtLeast32Bit;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	type CallIds<T: Config> = StorageMap<_, _, String, mock_builder::CallId>;

	impl<T: Config> Pallet<T> {
		pub fn mock_now(f: impl Fn() -> T::Moment + 'static) {
			register_call!(move |()| f());
		}
	}

	impl<T: Config> Time for Pallet<T> {
		type Moment = T::Moment;

		fn now() -> Self::Moment {
			execute_call!(())
		}
	}
}

/// With procedural support, it will be autogenerated as follows,
/// implementing all mock_* functions.
///
/// Be aware that this shows a very complex use case with several traits with a bunch of functions
///
/// ```ignore
/// #[mock_builder::pallet]
/// mod mock_pallet_time {
///     trait Currency<AccountId> {
///         type Balance: Balance + MaybeSerializeDeserialize;
///         type PositiveImbalance: Imbalance<Self::Balance, Opposite = Self::NegativeImbalance>;
///         type NegativeImbalance: Imbalance<Self::Balance, Opposite = Self::PositiveImbalance>;
///         fn total_balance(a: &AccountId) -> Self::Balance;
///         fn can_slash(a: &T::AccountId, b: Self::Balance);
///         fn total_issuance() -> Self::Balance;
///         fn minimum_balance() -> Self::Balance;
///         fn burn(a: Self::Balance) -> Self::PositiveImbalance;
///         fn issue(a: Self::Balance) -> Self::NegativeImbalance;
///         fn free_balance(a: &T::AccountId) -> Self::Balance;
///         fn ensure_can_withdraw(
///             a: &T::AccountId,
///             b: Self::Balance,
///             c: WithdrawReasons,
///             d: Self::Balance,
///         ) -> DispatchResult;
///         fn transfer(
///             a: &T::AccountId,
///             b: &T::AccountId,
///             c: Self::Balance,
///             d: ExistenceRequirement,
///         ) -> DispatchResult;
///         fn slash(a: &T::AccountId, b: Self::Balance) -> (Self::NegativeImbalance, Self::Balance);
///         fn deposit_into_existing(
///             a: &T::AccountId,
///             b: Self::Balance,
///         ) -> Result<Self::PositiveImbalance, DispatchError>;
///         fn deposit_creating(a: &T::AccountId, b: Self::Balance) -> Self::PositiveImbalance;
///         fn withdraw(
///             a: &T::AccountId,
///             b: Self::Balance,
///             c: WithdrawReasons,
///             d: ExistenceRequirement,
///         ) -> Result<Self::NegativeImbalance, DispatchError>;
///         fn make_free_balance_be(
///             a: &T::AccountId,
///             b: Self::Balance,
///         ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance>
///     }
///     trait ReservableCurrency<AccountId> {
///         fn can_reserve(a: &T::AccountId, b: Self::Balance) -> bool;
///         fn slash_reserved(
///             a: &T::AccountId,
///             b: Self::Balance,
///         ) -> (Self::NegativeImbalance, Self::Balance);
///         fn reserved_balance(a: &T::AccountId) -> Self::Balance;
///         fn reserve(a: &T::AccountId, b: Self::Balance) -> DispatchResult;
///         fn unreserve(a: &T::AccountId, b: Self::Balance) -> Self::Balance;
///         fn repatriate_reserved(
///             a: &T::AccountId,
///             b: &T::AccountId,
///             c: Self::Balance,
///             d: BalanceStatus,
///         ) -> Result<Self::Balance, DispatchError>;
///     }
/// }
/// ```
///
/// NOTE: It's expected that if you own the trait, you can simply put ONE procedural attribute on
/// top of it to autogerate the mocked pallet.
///
/// ```ignore
/// #[mock_builder::pallet] // <- this
/// trait MyCustomTrait {
///     // A lot of methods
/// }
/// ```
#[frame_support::pallet(dev_mode)]
pub mod mock_pallet_currency {
	use frame_support::{
		pallet_prelude::*,
		traits::{
			tokens::{
				imbalance::{Imbalance, SignedImbalance},
				Balance, BalanceStatus, ExistenceRequirement, WithdrawReasons,
			},
			Currency, ReservableCurrency,
		},
	};
	use mock_builder::{execute_call, register_call};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Balance: Balance + MaybeSerializeDeserialize;
		type PositiveImbalance: Imbalance<Self::Balance, Opposite = Self::NegativeImbalance>;
		type NegativeImbalance: Imbalance<Self::Balance, Opposite = Self::PositiveImbalance>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	type CallIds<T: Config> = StorageMap<_, _, String, mock_builder::CallId>;

	impl<T: Config> Pallet<T> {
		pub fn mock_reserved_balance(f: impl Fn(&T::AccountId) -> T::Balance + 'static) {
			register_call!(f)
		}

		pub fn mock_reserve(f: impl Fn(&T::AccountId, T::Balance) -> DispatchResult + 'static) {
			register_call!(move |(a, b)| f(a, b))
		}

		// Not mandatory to mock all functions from the traits
		// Any `mock_<name>` function can be used to mock the a <name> function.
		//
		// With procedural macros, every mock method will be generated
	}

	impl<T: Config> Currency<T::AccountId> for Pallet<T> {
		type Balance = T::Balance;
		type PositiveImbalance = T::PositiveImbalance;
		type NegativeImbalance = T::NegativeImbalance;

		fn total_balance(a: &T::AccountId) -> Self::Balance {
			execute_call!(a)
		}

		fn can_slash(a: &T::AccountId, b: Self::Balance) -> bool {
			execute_call!((a, b))
		}

		fn total_issuance() -> Self::Balance {
			execute_call!(())
		}

		fn minimum_balance() -> Self::Balance {
			execute_call!(())
		}

		fn burn(a: Self::Balance) -> Self::PositiveImbalance {
			execute_call!(a)
		}

		fn issue(a: Self::Balance) -> Self::NegativeImbalance {
			execute_call!(a)
		}

		fn free_balance(a: &T::AccountId) -> Self::Balance {
			execute_call!(a)
		}

		fn ensure_can_withdraw(
			a: &T::AccountId,
			b: Self::Balance,
			c: WithdrawReasons,
			d: Self::Balance,
		) -> DispatchResult {
			execute_call!((a, b, c, d))
		}

		fn transfer(
			a: &T::AccountId,
			b: &T::AccountId,
			c: Self::Balance,
			d: ExistenceRequirement,
		) -> DispatchResult {
			execute_call!((a, b, c, d))
		}

		fn slash(a: &T::AccountId, b: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
			execute_call!((a, b))
		}

		fn deposit_into_existing(
			a: &T::AccountId,
			b: Self::Balance,
		) -> Result<Self::PositiveImbalance, DispatchError> {
			execute_call!((a, b))
		}

		fn deposit_creating(a: &T::AccountId, b: Self::Balance) -> Self::PositiveImbalance {
			execute_call!((a, b))
		}

		fn withdraw(
			a: &T::AccountId,
			b: Self::Balance,
			c: WithdrawReasons,
			d: ExistenceRequirement,
		) -> Result<Self::NegativeImbalance, DispatchError> {
			execute_call!((a, b, c, d))
		}

		fn make_free_balance_be(
			a: &T::AccountId,
			b: Self::Balance,
		) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
			execute_call!((a, b))
		}
	}

	impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T> {
		fn can_reserve(a: &T::AccountId, b: Self::Balance) -> bool {
			execute_call!((a, b))
		}

		fn slash_reserved(
			a: &T::AccountId,
			b: Self::Balance,
		) -> (Self::NegativeImbalance, Self::Balance) {
			execute_call!((a, b))
		}

		fn reserved_balance(a: &T::AccountId) -> Self::Balance {
			execute_call!(a)
		}

		fn reserve(a: &T::AccountId, b: Self::Balance) -> DispatchResult {
			execute_call!((a, b))
		}

		fn unreserve(a: &T::AccountId, b: Self::Balance) -> Self::Balance {
			execute_call!((a, b))
		}

		fn repatriate_reserved(
			a: &T::AccountId,
			b: &T::AccountId,
			c: Self::Balance,
			d: BalanceStatus,
		) -> Result<Self::Balance, DispatchError> {
			execute_call!((a, b, c, d))
		}
	}
}

/// With procedural support, it will be autogenerated as follows,
/// implementing all mock_* functions.
///
/// NOTE: It's expected that if you own the trait, you can simply put one procedural attribute on
/// top of it to autogerate the mocked pallet.
/// ```ignore
/// #[mock_builder::pallet]
/// mod mock_pallet_time {
///     trait Auctioneer<AccountId> {
/// 		type AccountId = AccountId; /// Initialized with a value
/// 		type LeasePeriod: T::LeasePeriod;
/// 		type Currency: T::Currency;
///         fn new_auction(a: BlockNumberFor<T>, b: Self::LeasePeriod) -> DispatchResult;
///         fn auction_status(a: BlockNumberFor<T>) -> AuctionStatus<BlockNumberFor<T>>;
///         fn place_bid(
///             a: Self::AccountId,
///             b: Id,
///             c: Self::LeasePeriod,
///             d: Self::LeasePeriod,
///             e: <Self::Currency as Currency<Self::AccountId>>::Balance,
///         ) -> DispatchResult;
///         #[cfg(feature = "runtime-benchmarks")]
///         fn lease_period_length() -> (BlockNumberFor<T>, BlockNumberFor<T>);
///         fn lease_period_index(a: BlockNumberFor<T>) -> Option<(Self::LeasePeriod, bool)>;
///         fn has_won_an_auction(a: Id, b: &Self::AccountId) -> bool;
///     }
/// }
/// ```
#[frame_support::pallet(dev_mode)]
pub mod mock_pallet_auctioneer {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::BlockNumberFor;
	use mock_builder::{execute_call, register_call};
	use polkadot_primitives::Id;
	use polkadot_runtime_common::traits::{AuctionStatus, Auctioneer};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type LeasePeriod;
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	type CallIds<T: Config> = StorageMap<_, _, String, mock_builder::CallId>;

	impl<T: Config> Pallet<T> {
		pub fn mock_new_auction(
			f: impl Fn(BlockNumberFor<T>, T::LeasePeriod) -> DispatchResult + 'static,
		) {
			register_call!(move |(a, b)| f(a, b))
		}

		// Not mandatory to mock all functions from the traits
		// Any `mock_<name>` function can be used to mock the a <name> function.
		//
		// With procedural macros, every mock method will be generated
	}

	impl<T: Config> Auctioneer<BlockNumberFor<T>> for Pallet<T> {
		type AccountId = T::AccountId;
		type LeasePeriod = T::LeasePeriod;
		type Currency = T::Currency;

		fn new_auction(a: BlockNumberFor<T>, b: Self::LeasePeriod) -> DispatchResult {
			execute_call!((a, b))
		}

		fn auction_status(a: BlockNumberFor<T>) -> AuctionStatus<BlockNumberFor<T>> {
			execute_call!(a)
		}

		fn place_bid(
			a: Self::AccountId,
			b: Id,
			c: Self::LeasePeriod,
			d: Self::LeasePeriod,
			e: <Self::Currency as Currency<Self::AccountId>>::Balance,
		) -> DispatchResult {
			execute_call!((a, b, c, d, e))
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn lease_period_length() -> (BlockNumberFor<T>, BlockNumberFor<T>) {
			execute_call!(())
		}

		fn lease_period_index(a: BlockNumberFor<T>) -> Option<(Self::LeasePeriod, bool)> {
			execute_call!(a)
		}

		fn has_won_an_auction(a: Id, b: &Self::AccountId) -> bool {
			execute_call!((a, b))
		}
	}
}
