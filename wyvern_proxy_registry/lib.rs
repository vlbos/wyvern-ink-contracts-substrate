#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
// mod upgradeable;

#[ink::contract]
mod wyvern_proxy_registry {
    use ink_storage::Mapping;
    // use authenticated_proxy::AuthenticatedProxyRef;
    // use ownable_delegate_proxy::OwnableDelegateProxyRef;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use ownable::Ownable;
    use proxy_registry::ProxyRegistry;

    //  use crate::upgradeable::{
    //         NotInitialized,
    //         Upgradeable,
    //     };
    //  Delay period for adding an authenticated contract.
    //    This mitigates a particular class of potential attack on the Wyvern DAO (which owns this registry) - if at any point the value of assets held by proxy contracts exceeded the value of half the WYV supply (votes in the DAO),
    //    a malicious but rational attacker could buy half the Wyvern and grant themselves access to all the proxy contracts. A delay period renders this attack nonthreatening - given two weeks, if that happened, users would have
    //    plenty of time to notice and transfer their assets.
    const DELAY_PERIOD: Timestamp = 2;

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        previous_owner: AccountId,
        #[ink(topic)]
        new_owner: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct WyvernProxyRegistry {
        /// Whether the initial auth has :AccountId been set.
        initial_address_set: bool,
        _owner: AccountId,

        /// DelegateProxy implementation contract. Must be initialized.
        delegate_proxy_implementation: AccountId,

        /// Authenticated proxies by user.
        proxies: Mapping<AccountId, AccountId>,

        /// Contracts pending access.
        pending: Mapping<AccountId, u64>,

        /// Contracts allowed to call those proxies.
        contracts: Mapping<AccountId, bool>,
        ownable_delegate_proxy_code_hash: Hash,
    }

    impl WyvernProxyRegistry {
        /// Instantiate a `delegator` contract with the given sub-contract codes.
        #[ink(constructor)]
        pub fn new(
            authenticatedproxy_code_hash: Hash,
            _ownable_delegate_proxy_code_hash: Hash,
        ) -> Self {
            let total_balance = Self::env().balance();
            // let init_value: i32 = 0;
            let version: u32 = 0;
            let salt = version.to_le_bytes();
            // let _authenticatedproxy = AuthenticatedProxyRef::new()
            //     .endowment(total_balance / 4)
            //     .code_hash(authenticatedproxy_code_hash)
            //     .salt_bytes(salt)
            //     .instantiate()
            //     .unwrap_or_else(|error| {
            //         panic!(
            //             "failed at instantiating the Accumulator contract: {:?}",
            //             error
            //         )
            //     });

            // Self {
            //     delegate_proxy_implementation: authenticatedproxy,
            //     ownable_delegate_proxy_code_hash,
            // }
            ink_lang::utils::initialize_contract(|_contract: &mut Self| {
                // owners.sort_unstable();
                // owners.dedup();
                // ensure_requirement_is_valid(owners.len() as u32, requirement);

                // for owner in &owners {
                //     contract.is_owner.insert(owner, &());
                // }

                // contract.owners = owners;
                // contract.transaction_list = Default::default();
                // contract.requirement = requirement;
            })
        }

        /// Grant authentication to the initial Exchange protocol contract
        ///dev No delay, can only be called once - after that the standard registry process with a delay must be used
        ///param auth_address :AccountId of the contract to grant authentication
        #[ink(message)]
        pub fn grant_initial_authentication(&mut self, auth_address: AccountId) {
            // onlyOwner
            assert!(!self.initial_address_set);
            self.initial_address_set = true;
            // contracts[auth_address] = true;
            self.contracts.insert(&auth_address, &true);
        }
    }

    impl ProxyRegistry for WyvernProxyRegistry {
        /// Start the process to enable access for specified contract. Subject to delay period.
        ///dev ProxyRegistry owner only
        ///param addr to :AccountId which to grant permissions
        #[ink(message)]
        fn start_grant_authentication(&mut self, addr: AccountId) {
            self.only_owner();
            assert!(
                !self.contracts.get(&addr).unwrap_or(false)
                    && self.pending.get(&addr).unwrap_or(0) == 0
            );
            self.pending.insert(&addr, &self.env().block_timestamp());
        }

        /// End the process to nable access for specified contract after delay period has passed.
        ///dev ProxyRegistry owner only
        ///param addr to :AccountId which to grant permissions
        #[ink(message)]
        fn end_grant_authentication(&mut self, addr: AccountId) {
            self.only_owner();
            // assert!(
            //     !contracts[addr] && pending[addr] != 0 && ((pending[addr] + DELAY_PERIOD) < now)
            // );
            assert!(
                !self.contracts.get(&addr).unwrap_or(false)
                    && self.pending.get(&addr).unwrap_or(0) != 0
                    && self.pending.get(&addr).unwrap_or(0) + DELAY_PERIOD
                        < self.env().block_timestamp()
            );
            self.pending.insert(&addr, &0);
            self.contracts.insert(&addr, &true);
        }

        /// Revoke access for specified contract. Can be done instantly.
        ///dev ProxyRegistry owner only
        ///param addr of :AccountId which to revoke permissions
        #[ink(message)]
        fn revoke_authentication(&mut self, addr: AccountId) {
            self.only_owner();
            self.contracts.insert(&addr, &false);
        }

        /// Register a proxy contract with this registry
        ///dev Must be called by the user which the proxy is for, creates a new AuthenticatedProxy
        ///return New AuthenticatedProxy contract
        #[ink(message)]
        fn register_proxy(&mut self) {
            assert!(self.proxies.get(self.env().caller()).is_none());
            // proxy = new OwnableDelegateProxy(self.env().caller(), delegate_proxy_implementation, abi.encodeWithSignature("initialize(AccountId,AccountId)", self.env().caller(), AccountId(this)));
            //             let total_balance = Self::env().balance();
            //             // let init_value: i32 = 0;
            //             let version: u32 = 0;
            //             let salt = version.to_le_bytes();
            // let owner= AccountId::default();
            // let initial_implementation= AccountId::default();
            //   let  calldata:Vec<u8> =Vec::new();
            // let _ownable_delegate_proxy = OwnableDelegateProxyRef::new(owner,initial_implementation,calldata)
            //     .endowment(total_balance / 4)
            //     .code_hash(self.ownable_delegate_proxy_code_hash)
            //     .salt_bytes(salt)
            //     .instantiate()
            //     .unwrap_or_else(|error| {
            //         panic!(
            //             "failed at instantiating the Accumulator contract: {:?}",
            //             error
            //         )
            //     });
            // self.proxies
            //     .insert(&self.env().caller(), &_ownable_delegate_proxy.contract_address());
        }
        /// Panic if the sender is no owner of the wallet.
        #[ink(message)]
        fn ensure_caller_is_owner(&self) {
            assert_eq!(self.env().caller(), self.env().account_id());
        }
    }

    impl Ownable for WyvernProxyRegistry {
        ///dev Initializes the contract setting the deployer as the initial owner.
        // #[ink(constructor)]
        //fn  new() -> Self {
        //     ink_lang::utils::initialize_contract(|contract: &mut Self| {
        //         contract._owner = self.env().caller();
        //     })
        // }

        ///dev Returns the of :AccountId the current owner.
        #[ink(message)]
        fn owner(&self) -> AccountId {
            self._owner
        }

        ///dev Throws if called by any account other than the owner.
        #[ink(message)]
        fn only_owner(&self) {
            // require(owner() == _msgSender(), "Ownable: caller is not the owner");
            // _;
            assert_eq!(self.env().caller(), self.owner());
        }

        ///dev Leaves the contract without owner. It will not be possible to call
        /// `onlyOwner` functions anymore. Can only be called by the current owner.
        ///
        /// NOTE: Renouncing ownership will leave the contract without an owner,
        /// thereby removing any functionality that is only available to the owner.
        #[ink(message)]
        fn renounce_ownership(&mut self) {
            self.only_owner();
            self._set_owner(AccountId::default());
        }

        ///dev Transfers ownership of the contract to a new account (`new_owner`).
        /// Can only be called by the current owner.
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) {
            self.only_owner();
            assert!(
                new_owner != AccountId::default(),
                "Ownable: new owner is the zero address"
            ); //,
            self._set_owner(new_owner);
        }
        #[ink(message)]
        fn _set_owner(&mut self, new_owner: AccountId) {
            let old_owner: AccountId = self._owner;
            self._owner = new_owner;
            // emit OwnershipTransferred(oldOwner, new_owner);
            self.env().emit_event(OwnershipTransferred {
                previous_owner: old_owner,
                new_owner,
            });
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        // /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let wyvern_proxy_registry = WyvernProxyRegistry::default();
        //     assert_eq!(wyvern_proxy_registry.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut wyvern_proxy_registry = WyvernProxyRegistry::new(false);
        //     assert_eq!(wyvern_proxy_registry.get(), false);
        //     wyvern_proxy_registry.flip();
        //     assert_eq!(wyvern_proxy_registry.get(), true);
        // }
    }
}
