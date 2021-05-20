#![cfg_attr(not(feature = "std"), no_std)]

pub use without_instance::*; // reexport in crate namespace for `construct_runtime!`

#[frame_support::pallet]
// NOTE: The name of the pallet is provided by `construct_runtime` and is used as
// the unique identifier for the pallet's storage. It is not defined in the pallet itself.
pub mod without_instance {
    use frame_support::pallet_prelude::*; // Import various types used in the pallet definition
    use frame_support::sp_runtime::print;
    use frame_system::pallet_prelude::*; // Import some system helper types.

    type BalanceOf<T> = <T as Config>::Balance;

    // Define the generic parameter of the pallet
    // The macro parses `#[pallet::constant]` attributes and uses them to generate metadata
    // for the pallet's constants.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant] // put the constant in metadata
        type MyGetParam: Get<u32>;
        type Balance: Parameter + From<u8>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Define the pallet struct placeholder, various pallet function are implemented on it.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Define some additional constant to put into the constant metadata.
    #[pallet::extra_constants]
    impl<T: Config> Pallet<T> {
        /// some description
        fn extra_constant_name() -> u128 {
            4u128
        }
    }

    // Implement the pallet hooks.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_: BlockNumberFor<T>) -> Weight {
            print("hook fired: on_initialize");
            10
        }

        // can implement also: on_finalize, on_runtime_upgrade, offchain_worker, ...
        // see `Hooks` trait
    }

    // Declare Call struct and implement dispatchables.
    //
    // WARNING: Each parameter used in functions must implement: Clone, Debug, Eq, PartialEq,
    // Codec.
    //
    // The macro parses `#[pallet::compact]` attributes on function arguments and implements
    // the `Call` encoding/decoding accordingly.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Doc comment put in metadata
        #[pallet::weight(0)] // Defines weight for call (function parameters are in scope)
        fn toto(origin: OriginFor<T>, #[pallet::compact] foo: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            print("call toto with params");
            ensure!(foo < 10, Error::<T>::InsufficientProposersBalance);
            print("do some work here...");
            Self::deposit_event(Event::Something(foo));
            Ok(())
        }
    }

    // Declare the pallet `Error` enum (this is optional).
    // The macro generates error metadata using the doc comment on each variant.
    #[pallet::error]
    pub enum Error<T> {
        /// doc comment put into metadata
        InsufficientProposersBalance,
    }

    // Declare pallet Event enum (this is optional).
    //
    // WARNING: Each type used in variants must implement: Clone, Debug, Eq, PartialEq, Codec.
    //
    // The macro generates event metadata, and derive Clone, Debug, Eq, PartialEq and Codec
    #[pallet::event]
    // Additional argument to specify the metadata to use for given type.
    #[pallet::metadata(BalanceOf<T> = "Balance", u32 = "Other")]
    // Generate a function on Pallet to deposit an event.
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// doc comment put in metadata
        // `<T as frame_system::Config>::AccountId` is not defined in metadata list, the last
        // Thus the metadata is `<T as frame_system::Config>::AccountId`.
        Proposed(<T as frame_system::Config>::AccountId),
        /// doc
        // here metadata will be `Balance` as define in metadata list
        Spending(BalanceOf<T>),
        // here metadata will be `Other` as define in metadata list
        Something(u32),
    }

    // Define a struct which implements `frame_support::traits::Get<T::Balance>` (optional).
    #[pallet::type_value]
    pub(super) fn MyDefault<T: Config>() -> T::Balance {
        3.into()
    }

    // Declare a storage item. Any amount of storage items can be declared (optional).
    //
    // Is expected either `StorageValue`, `StorageMap` or `StorageDoubleMap`.
    // The macro generates the prefix type and replaces the first generic `_`.
    //
    // The macro expands the metadata for the storage item with the type used:
    // * for a storage value the type of the value is copied into the metadata
    // * for a storage map the type of the values and the type of the key is copied into the metadata
    // * for a storage double map the types of the values and keys are copied into the
    //   metadata.
    //
    // NOTE: The generic `Hasher` must implement the `StorageHasher` trait (or the type is not
    // usable at all). We use [`StorageHasher::METADATA`] for the metadata of the hasher of the
    // storage item. Thus generic hasher is supported.
    #[pallet::storage]
    pub(super) type MyStorageValue<T: Config> =
        StorageValue<_, T::Balance, ValueQuery, MyDefault<T>>;

    // Another storage declaration
    #[pallet::storage]
    #[pallet::getter(fn my_storage)]
    pub(super) type MyStorage<T> = StorageMap<_, Blake2_128Concat, u32, u32>;

    // Declare the genesis config (optional).
    //
    // The macro accepts either a struct or an enum; it checks that generics are consistent.
    //
    // Type must implement the `Default` trait.
    // #[pallet::genesis_config]
    // #[derive(Default)]
    // pub struct GenesisConfig {
    //     _my_field: u32,
    // }

    // Declare genesis builder. (This is need only if GenesisConfig is declared)
    // #[pallet::genesis_build]
    // impl<T: Config> GenesisBuild<T> for GenesisConfig {
    //     fn build(&self) {}
    // }

    // Declare a pallet origin (this is optional).
    //
    // The macro accept type alias or struct or enum, it checks generics are consistent.
    // #[pallet::origin]
    // pub struct Origin<T>(PhantomData<T>);
}
