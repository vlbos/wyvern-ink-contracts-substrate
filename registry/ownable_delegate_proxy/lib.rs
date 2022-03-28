#![cfg_attr(not(feature = "std"), no_std)]
pub use self::ownable_delegate_proxy::{
    OwnableDelegateProxy,
    OwnableDelegateProxyRef,
};
use ink_lang as ink;

#[ink::contract]
mod ownable_delegate_proxy {
 use crate::upgradeable::{
        NotInitialized,
        Upgradeable,
    };
use owned_upgradeability_storage::OwnedUpgradeabilityStorage;
use owned_upgradeability_proxy::OwnedUpgradeabilityProxy;
use proxy::Proxy;

// OwnedUpgradeabilityProxy
    ///dev Event to show ownership has been transferred
    ///param previousOwner representing the of :AccountId the previous owner
    ///param new_owner representing the of :AccountId the new owner
    #[ink(event)]
    pub struct ProxyOwnershipTransferred {
        previous_owner: AccountId,
        new_owner: AccountId,
    }

    ///dev This event will be emitted every time the implementation gets upgraded
    ///param implementation representing the of :AccountId the upgraded implementation

    #[ink(event)]
    pub struct Upgraded {
        #[ink(topic)]
        implementation: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct OwnableDelegateProxy {
  // Current implementation
        _implementation: Upgradeable<Hash, NotInitialized>,

        // Owner of the contract
        _upgradeability_owner: Upgradeable<AccountId, NotInitialized>,
}

    impl OwnableDelegateProxy {
        #[ink(constructor)]
        pub fn new(owner: AccountId, initial_implementation: AccountId, calldata: Vec<u8>) -> Self {
            // set_upgradeability_owner(owner);
            // _upgradeTo(initialImplementation);
            // assert!(initialImplementation.delegatecall(calldata));
            let gas_limit=0;
            let transferred_value = 0;
            let mut selector=calldata;
            let mut input=selector.split_off(4);
            let result = build_call::<<Self as ::ink_lang::reflect::ContractEnv>::Env>()
                .call_type(
                    Call::new()
                        .callee(self.env().account_id())
                        .gas_limit(gas_limit)
                        .transferred_value(transferred_value),
                )
                .exec_input(
                    ExecutionInput::new(selector.into()).push_arg(CallInput(&input)),
                )
                .returns::<()>()
                .fire()
                .map_err(|_| Error::TransactionFailed);

            
            assert!(result.is_ok())
            Self {}
        }

        /// Changes the `Hash` of the contract where any call that does
        /// not match a selector of this contract is delegated to.
        #[ink(message)]
        pub fn change_delegate_code(&mut self, new_code_hash: Hash) {
            assert_eq!(
                self.env().caller(),
                *self._upgradeability_owner,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                *self._upgradeability_owner,
            );
            *self._implementation = new_code_hash;
        }
 /// Fallback message for a contract call that doesn't match any
        /// of the other message selectors. Proxy contract delegates the execution
        /// of that message to the `forward_to` contract with all input data.
        ///
        /// # Note:
        ///
        /// - We allow payable messages here and would forward any optionally supplied
        ///   value as well.
        /// - If the self receiver were `forward(&mut self)` here, this would not
        ///   have any effect whatsoever on the contract we forward to.
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) -> u32 {
            ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(DelegateCall::new().code_hash(*self._implementation))
                .call_flags(
                    ink_env::CallFlags::default()
                        // We don't plan to use the input data after the delegated call, so the 
                        // input data can be forwarded to delegated contract to reduce the gas usage.
                        .set_forward_input(true)
                        // We don't plan to return back to that contract after execution, so we 
                        // marked delegated call as "tail", to end the execution of the contract.
                        .set_tail_call(true),
                )
                .fire()
                .unwrap_or_else(|err| {
                    panic!(
                        "delegate call to {:?} failed due to {:?}",
                        *self._implementation, err
                    )
                });
            unreachable!(
                "the forwarded call will never return since `tail_call` was set"
            );
        }
    }

impl OwnedUpgradeabilityStorage for OwnableDelegateProxy {
     
///dev Tells the of :AccountId the owner
        ///return the of :AccountId the owner
        #[ink(message)]
        pub fn upgradeability_owner() -> AccountId {
            *self._upgradeability_owner
        }

        ///dev Sets the of :AccountId the owner
        #[ink(message)]
        pub fn set_upgradeability_owner(new_upgradeability_owner: AccountId) {
            *self._upgradeability_owner = new_upgradeability_owner;
        }


    }


impl Proxy for OwnableDelegateProxy {
   

        ///dev Tells the of :AccountId the current implementation
        ///return of :AccountId the current implementation
        #[ink(message)]
        pub fn implementation() -> AccountId {
            *self._implementation
        }

        ///dev Tells the proxy type (EIP 897)
        ///return Proxy type, 2 for forwarding proxy
        #[ink(message)]
        pub fn proxy_type() -> u32 {
            2
        }

    }

impl OwnedUpgradeabilityProxy for OwnableDelegateProxy {
     
 ///dev Upgrades the implementation address
        ///param implementation representing the of :AccountId the new implementation to be set
        #[ink(message)]
        fn _upgradeTo(implementation: AccountId) {
            assert!(self._implementation != implementation);
            self._implementation = implementation;
            self.env().emit_event(Upgraded { implementation });
        }

        ///dev Throws if called by any account other than the owner.
        fn only_proxy_owner() {
            assert!(self.env().caller() == proxy_Owner());
            // _;
        }

        ///dev Tells the of :AccountId the proxy owner
        ///return the of :AccountId the proxy owner
        #[ink(message)]
        pub fn proxy_Owner() -> AccountId {
            self.upgradeability_owner()
        }

        ///dev Allows the current owner to transfer control of the contract to a new_owner.
        ///param new_owner The to :AccountId transfer ownership to.
        #[ink(message)]
        pub fn transfer_proxy_ownership(new_owner: AccountId) {
            self.only_proxy_owner();
            assert!(new_owner != AccountId::default());
            // emit ProxyOwnershipTransferred(proxy_Owner(), new_owner);
            self.env().emit_event(ProxyOwnershipTransferred {
                previous_owner: proxy_Owner(),
                new_owner,
            });

            self.set_upgradeability_owner(new_owner);
        }

        ///dev Allows the upgradeability owner to upgrade the current implementation of the proxy.
        ///param implementation representing the of :AccountId the new implementation to be set.
        #[ink(message)]
        pub fn upgrade_to(implementation: AccountId) {
            self.only_proxy_owner();
            self._upgradeTo(implementation);
        }

        ///dev Allows the upgradeability owner to upgrade the current implementation of the proxy
        ///and delegatecall the new implementation for initialization.
        ///param implementation representing the of :AccountId the new implementation to be set.
        ///param data represents the msg.data to bet sent in the low level call. This parameter may include the pub fn
        ///signature of the implementation to be called with the needed payload
        #[ink(message, payable)]
        pub fn upgrade_to_and_call(implementation: AccountId, data: Vec<u8>) {
            self.only_proxy_owner();
            self.upgrade_to(implementation);

            let gas_limit=0;
            let transferred_value = 0;
            let mut selector=data;
            let mut input=selector.split_off(4);
            let result = build_call::<<Self as ::ink_lang::reflect::ContractEnv>::Env>()
                .call_type(
                    Call::new()
                        .callee(self.env().account_id())
                        .gas_limit(gas_limit)
                        .transferred_value(transferred_value),
                )
                .exec_input(
                    ExecutionInput::new(selector.into()).push_arg(CallInput(&input)),
                )
                .returns::<()>()
                .fire()
                .map_err(|_| Error::TransactionFailed);

            
            assert!(result.is_ok())
            // assert!(self.env().account_id().delegatecall(data));
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
        //     let ownable_delegate_proxy = OwnableDelegateProxy::default();
        //     assert_eq!(ownable_delegate_proxy.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut ownable_delegate_proxy = OwnableDelegateProxy::new(false);
        //     assert_eq!(ownable_delegate_proxy.get(), false);
        //     ownable_delegate_proxy.flip();
        //     assert_eq!(ownable_delegate_proxy.get(), true);
        // }
    }
}
