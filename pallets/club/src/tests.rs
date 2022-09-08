use crate::{mock::{*, self}, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::{RawOrigin};

use sp_runtime::{
    testing::Header,
    traits::{BadOrigin, BlakeTwo256, IdentityLookup},
};

#[test]
fn test_create_club() {
    new_test_ext().execute_with(|| {
     let clubid =1; 
    
     assert_eq!(ClubModule::club(), 0);
     assert_ok!(ClubModule::create_club(mock::Origin::root(), clubid));
     assert_eq!(ClubModule::club(), 1);
    
    });

}

#[test]
fn test_add_member(){

    new_test_ext().execute_with(||{

        let clubid = 1; 
        
        assert_ok!(ClubModule::create_club(mock::Origin::root(), clubid));
        assert_eq!(ClubModule::club(), 1);
        assert_noop!(ClubModule::add_member(mock::Origin::signed(15),1,clubid), BadOrigin);
       assert_ok!(ClubModule::add_member(mock::Origin::root(),1,clubid));
       })
}

#[test]
fn test_remove_member(){

    new_test_ext().execute_with(||{

        let clubid = 1; 
        // let member = RawOrigin::Root;
        assert_ok!(ClubModule::create_club(mock::Origin::root(), clubid));
        assert_eq!(ClubModule::club(), 1);
     assert_noop!(ClubModule::remove_member(mock::Origin::signed(15),1,clubid), BadOrigin);
       assert_ok!(ClubModule::add_member(mock::Origin::root(),1,clubid));
     let res = ClubModule::remove_member(mock::Origin::root(),1,clubid);
     assert_ok!(res);
   })
}
