#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]

    pub type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRemoved(T::AccountId, Vec<u8>),
        ClaimOwnerChanged(T::AccountId, T::AccountId, Vec<u8>),
    }


    #[pallet::error]
    pub enum Error<T> {
        NoneValue,

        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,

        NotTransferOwner,
        ErrorForTransferClaimUnauthorized,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

   
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn create_claim(
            origin: OriginFor<T>, 
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            
            let creator = ensure_signed(origin)?;

            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExist
            );

            Proofs::<T>::insert(
                &claim,
                (creator.clone(), frame_system::Pallet::<T>::block_number()),
            );
            
            Self::deposit_event(Event::ClaimCreated(creator, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn remove_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            
            let sender = ensure_signed(origin)?;

            let (owner, _) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(sender == owner, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRemoved(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            newUser: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            
            let sender = ensure_signed(origin)?;

            let (owner, block_Num) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(
                sender == owner,
                Error::<T>::NotTransferOwner
            );
            Proofs::<T>::insert(&claim, (&newUser, block_Num));
            Self::deposit_event(Event::ClaimOwnerChanged(owner, newUser, claim));
            Ok(().into())
        }
    }
}