#![cfg_attr(not(feature = "std"), no_std)]

//dependecies used
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{sp_runtime::traits::{Hash, Zero}, 
                        inherent::Vec,
                        dispatch::{DispatchResultWithPostInfo, DispatchResult},
                        traits::{Currency, ExistenceRequirement, Randomness},
                        pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
    use scale_info::TypeInfo;

    #[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

    //declaration of pallet types
    type AccountOf<T> = <T as frame_system::Config>::AccountId;


    //Structs
    //struct to hold individual drug
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct DrugPackages<T: Config> {
        pub lot_number: [u8; 16],
        pub expiry_date: Vec<u8>,
        pub verifiable_identifier: AccountOf<T>,
        pub gtin: [u8; 16],
        pub manufacturer_name: Vec<u8>,
        pub manufacturing_country: Vec<u8>,
        pub manufacturing_facility: Vec<u8>,
        pub lot_id: [u8; 16],
    }

    //Enum and implementations


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    //#[pallet::generate_storage_info]
    pub struct Pallet<T>(_);

    //runtime configuration traits. parameters, types and constants it deppends on
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the flexinft pallet.
        type Currency: Currency<Self::AccountId>;

        // custom types for flexinft pallet.
        //maximum number of cnfts a single account can own
        #[pallet::constant]
		type MaxCnftOwned: Get<u32>;

    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // specifying errors here
    }

    // declaring storage items

    //number of Cnfts created
    #[pallet::storage]
    #[pallet::getter(fn Cnft_cnt)]
    /// Keeps track of the number of composable nfts in existence.
    pub(super) type CnftCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

    //other storage items
    //create a storage instance for Cnft
    #[pallet::storage]
    #[pallet::getter(fn Flexinfts)]
    pub(super) type Flexinfts<T: Config> = StorageMap<
      _,
      Twox64Concat,
      T::Hash,
      DrugPackages<T>
      >; 
    
    //storage instance to keep track of what account owns what Cnft
    #[pallet::storage]
    #[pallet::getter(fn Cnft_owned)]
    /// Keeps track of what accounts own what Cnft.
    pub(super) type CnftOwned<T: Config> = StorageMap<
      _,
      Twox64Concat,
      T::AccountId,
      BoundedVec<T::Hash, T::MaxCnftOwned>,
      ValueQuery
      >;
    

    //pallets genesis configuration

    //runtime events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // write here
    }

    //hooks
    /*#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {} */

    //Extrinsics : functions calleable from outside the runtime
    #[pallet::call]
    impl<T:Config> Pallet<T> { 

     }

    impl<T:Config> Pallet<T> { 

     }


}