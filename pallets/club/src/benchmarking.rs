#![cfg(feature = "runtime-benchmarks")]


use crate::*;
use frame_benchmarking::{account,benchmarks, whitelisted_caller,Vec,impl_benchmark_test_suite};
use frame_system::{RawOrigin,Origin,EnsureSignedBy};
use super::{Pallet as ClubPallet, *};
use frame_support::{assert_ok, traits::EnsureOrigin};
const SEED: u32 = 0;


fn club_create<T:Config>(clubid:u32) {
    let caller: T::AccountId = whitelisted_caller();
    let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller));

	<ClubPallet<T>>::create_club(  caller_origin, 1);

}

benchmarks! {
 
    create_club {

        let clubid =  1; 
        let caller: T::AccountId = whitelisted_caller();
    let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller));
    }: _(RawOrigin::Root,clubid)
    verify {

       assert_eq!(Pallet::<T>::club(), 1);

    }


    add_member{
        let m in 1 .. (6 );
 	
    let new_member = account::<T::AccountId>("add", m, SEED);
       let clubid = 1;
        let caller: T::AccountId = whitelisted_caller();
      crate::Pallet::<T>::create_club(RawOrigin::Signed(caller.clone()).into(),clubid);
    }: _(RawOrigin::Root,new_member,clubid)
   

    remove_member{
        let m in 1 .. (6);
        
	
      let clubid = 3;
        let caller: T::AccountId = whitelisted_caller();
        let new_member = account::<T::AccountId>("add", m, SEED);
   
        let caller: T::AccountId = whitelisted_caller();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
    crate::Pallet::<T>::create_club(RawOrigin::Signed(caller.clone()).into(),clubid);

    crate::Pallet::<T>::add_member(RawOrigin::Signed(caller.clone()).into(),new_member,clubid);
    let new_member1 = account::<T::AccountId>("add", m, SEED);

    }: {
        assert_ok!(<ClubPallet<T>>::remove_member(RawOrigin::Signed(caller.clone()).into(), new_member1,clubid));
    }


  }

 impl_benchmark_test_suite!(ClubPallet, crate::mock::new_test_ext(), crate::mock::Test);
