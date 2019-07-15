use support::{decl_storage, decl_module, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ContractModule {
        // Declare storage and getter functions here
        pub NextDepositIndex get(next_deposit_index): Option<u64>;
        // pub LastDepositIndexOf get(last_deposit_index_of): map (T::AccountId, Symbol) => Option<u32>;
        // map address to u8
        pub PoolOperators: map T::AccountId => u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // must include if your module has event.
        fn deposit_event<T>() = default;
        // Declare public functions here
        pub fn set_next_deposit_index(origin, value: u64) -> Result {
            let _sender = ensure_signed(origin,)?;
            <NextDepositIndex<T>>::put(value);
            Ok(())
        }

//        pub fn set_pool_operator(origin, key:: AccountId, value: u32) -> Result {
//            let _sender = ensure_signed(origin)?;
//            <PoolOperators<T>>::insert(key, value);
//            OperatorsPriviedgeSet(key, value);
//            Ok(())
//        }
    }
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		OperatorsPriviedgeSet(AccountId, u32),
	}
);
