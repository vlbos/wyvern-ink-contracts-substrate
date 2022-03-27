//!dev Contract module which provides a basic access control mechanism, where
//! there is an account (an owner) that can be granted exclusive access to
//! specific functions.
//!
//! By default, the owner account will be the one that deploys the contract. This
//! can later be changed with {transfer_ownership}.
//!
//! This module is used through inheritance. It will make available the modifier
//! `onlyOwner`, which can be applied to your functions to restrict their use to
//! the owner.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// #[ink::contract]
// mod ownable {

//     #[ink(event)]
//     pub struct OwnershipTransferred {
//         #[ink(topic)]
//         previous_owner: AccountId,
//         #[ink(topic)]
//         new_owner: AccountId,
//     }

//     /// Defines the storage of your contract.
//     /// Add new fields to the below struct in order
//     /// to add new static storage fields to your contract.
//     #[ink(storage)]
//     pub struct Ownable {
//         _owner: AccountId,
//     }

    #[ink::trait_definition]
pub trait Ownable {
        // ///dev Initializes the contract setting the deployer as the initial owner.
        // #[ink(constructor)]
        // pub fn new() -> Self {
        //     ink_lang::utils::initialize_contract(|contract: &mut Self| {
        //         contract._owner = self.env().caller();
        //     })
        // }

        ///dev Returns the of :AccountId the current owner.
        #[ink(message)]
        pub fn owner() -> AccountId;

        ///dev Throws if called by any account other than the owner.
        fn only_owner();

        ///dev Leaves the contract without owner. It will not be possible to call
        /// `onlyOwner` functions anymore. Can only be called by the current owner.
        ///
        /// NOTE: Renouncing ownership will leave the contract without an owner,
        /// thereby removing any functionality that is only available to the owner.
        #[ink(message)]
        fn renounce_ownership() ;
        ///dev Transfers ownership of the contract to a new account (`new_owner`).
        /// Can only be called by the current owner.
        #[ink(message)]
        fn transfer_ownership(new_owner: AccountId);

        fn _set_owner(new_owner: AccountId);
    }

