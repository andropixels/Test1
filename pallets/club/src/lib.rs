#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;




#[frame_support::pallet]
pub mod pallet {
	// use crate::Error::{MemberLimitExceeded, MemberNotFound};
	use codec::{self};
    use super::*; 
	use frame_support::pallet_prelude::ValueQuery;
use frame_support::{Blake2_128Concat, BoundedVec};
// use frame_support::storage::bounded_vec::BoundedVec;
	use frame_support::{
		dispatch::{DispatchResult, PartialEq},
		pallet_prelude::*,
	};
    
    use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
    
	/// Configure the pallet by specifying the parameters and types on which it depends.

    #[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);



	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MAXMembers: Get<u32>;
        type WeightInfo: WeightInfo;
        
	}

	

	#[pallet::storage]
	#[pallet::getter(fn get_club_member)]
	pub type ClubMember<T: Config> =
		StorageValue<_, BoundedVec<ClubMembers<T>, T::MAXMembers>, ValueQuery>;

	
    #[pallet::storage]
    #[pallet::getter(fn club)]
   pub (super) type Clubs<T:Config> = StorageValue<_, u32, ValueQuery>;
   
	#[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct ClubMembers<T: Config> {
		AccountId: T::AccountId,
		RegisteredTime: T::BlockNumber,
        clubid:u32
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		WaitListMemberAdded(T::AccountId),
		ClubMemberAdded(T::AccountId),
		ClubMemberRemoved(T::AccountId),
		MemberRemovedfromWaitList(T::AccountId),
        ClubCreated(u32)
	}

	#[pallet::error]
	pub enum Error<T> {
		MemberAlreadyExist,
		MemberNotFound,
		TooManyMembers,
         NotAMember,
        ClubDoesNotExist
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

 
        #[pallet::weight(<T as Config>::WeightInfo::create_club())]
        pub fn create_club(origin: OriginFor<T>,clubid:u32) -> DispatchResult {
            ensure_root(origin)?;


            Clubs::<T>::put(clubid.clone() );


       Self::deposit_event(Event::ClubCreated(clubid));
			Ok(())

        }
		
		#[pallet::weight(<T as Config>::WeightInfo::add_member(10))]
		pub fn add_member(
			origin: OriginFor<T>,
			member: T::AccountId,
            clubid:u32
		) -> DispatchResult {
          ensure!( <Clubs<T>>::exists(), Error::<T>::ClubDoesNotExist);
           if clubid == 1 || clubid ==2 {
                ensure_root(origin)?;
            }else {
                ensure_signed(origin)?;
            }
			let time = <frame_system::Pallet<T>>::block_number();
			let mem = ClubMembers { AccountId: member.clone(), RegisteredTime: time,clubid };

            <ClubMember<T>>::try_mutate(|b_vec| b_vec.try_push(mem))
            .map_err(|_| <Error<T>>::TooManyMembers)?;

			
			Self::deposit_event(Event::ClubMemberAdded(member.clone()));
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::remove_member(1))]
		pub fn remove_member(origin: OriginFor<T>, member: T::AccountId,clubid:u32) -> DispatchResult {
             ensure!( <Clubs<T>>::exists(), Error::<T>::ClubDoesNotExist);
            if clubid == 1 || clubid ==2 {
                ensure_root(origin)?;
            }else {
                ensure_signed(origin)?;
            }
			<ClubMember<T>>::try_mutate(|b_vec| {
				if let Some(index) = b_vec.iter().position(|mem| mem.AccountId == member) {
					b_vec.remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| Error::<T>::MemberNotFound)?;
			Self::deposit_event(Event::ClubMemberRemoved(member.clone()));
			Ok(())
		}
	}
}

