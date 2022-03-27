//   Proxy registry; keeps a mapping of AuthenticatedProxy contracts and mapping of contracts authorized to access them.
//   Abstracted away from the Exchange (a) to reduce Exchange attack surface and (b) so that the Exchange contract can be upgraded without users needing to transfer assets to new proxies.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// #[ink::contract]
// mod proxy_registry {

// use ink_env::call::{build_call, Call, ExecutionInput};
// use ink_prelude::vec::Vec;
// use ink_storage::{
//     traits::{PackedLayout, SpreadAllocate, SpreadLayout},
//     Mapping,
// };
// use scale::Output;

//     //  Delay period for adding an authenticated contract.
//     //    This mitigates a particular class of potential attack on the Wyvern DAO (which owns this registry) - if at any point the value of assets held by proxy contracts exceeded the value of half the WYV supply (votes in the DAO),
//     //    a malicious but rational attacker could buy half the Wyvern and grant themselves access to all the proxy contracts. A delay period renders this attack nonthreatening - given two weeks, if that happened, users would have
//     //    plenty of time to notice and transfer their assets.
//     const DELAY_PERIOD: u32 = 2;

//    #[ink(event)]
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
//     #[derive(SpreadAllocate)]
//     pub struct ProxyRegistry {
//         _owner: AccountId,

//         /// DelegateProxy implementation contract. Must be initialized.
//         delegateProxyImplementation: AccountId,

//         /// Authenticated proxies by user.
//         proxies: Mapping<AccountId, AccountId>,

//         /// Contracts pending access.
//         pending: Mapping<AccountId, u32>,

//         /// Contracts allowed to call those proxies.
//         contracts: Mapping<AccountId, bool>,

//     }

#[ink::trait_definition]
pub trait ProxyRegistry {
    // /// Constructor that initializes the `bool` value to the given `init_value`.
    // #[ink(constructor)]
    // pub fn new() -> Self {
    //     Self {}
    // }

    /// Start the process to enable access for specified contract. Subject to delay period.
    ///dev ProxyRegistry owner only
    ///param addr to :AccountId which to grant permissions
    #[ink(message)]
    pub fn start_grant_authentication(addr: AccountId);
    /// End the process to nable access for specified contract after delay period has passed.
    ///dev ProxyRegistry owner only
    ///param addr to :AccountId which to grant permissions
    #[ink(message)]
    pub fn end_grant_authentication(addr: AccountId);

    /// Revoke access for specified contract. Can be done instantly.
    ///dev ProxyRegistry owner only
    ///param addr of :AccountId which to revoke permissions
    #[ink(message)]
    pub fn revoke_authentication(addr: AccountId);

    /// Register a proxy contract with this registry
    ///dev Must be called by the user which the proxy is for, creates a new AuthenticatedProxy
    ///return New AuthenticatedProxy contract
    #[ink(message)]
    pub fn register_proxy() -> AccountId;

    /// Panic if the sender is no owner of the wallet.
    fn ensure_caller_is_owner(&self);
}
