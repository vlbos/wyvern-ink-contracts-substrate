// This contract combines an upgradeability proxy with basic authorization control functionalities
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::trait_definition]
pub trait OwnedUpgradeabilityProxy {
         ///dev Upgrades the implementation address
        ///param implementation representing the of :AccountId the new implementation to be set
        #[ink(message)]
        fn _upgradeTo(implementation: AccountId);

        ///dev Throws if called by any account other than the owner.
        fn only_proxy_owner() ;

        ///dev Tells the of :AccountId the proxy owner
        ///return the of :AccountId the proxy owner
        #[ink(message)]
        fn proxy_Owner() -> AccountId ;

        ///dev Allows the current owner to transfer control of the contract to a new_owner.
        ///param new_owner The to :AccountId transfer ownership to.
        #[ink(message)]
        fn transfer_proxy_ownership(new_owner: AccountId);

        ///dev Allows the upgradeability owner to upgrade the current implementation of the proxy.
        ///param implementation representing the of :AccountId the new implementation to be set.
        #[ink(message)]
        fn upgrade_to(implementation: AccountId);

        ///dev Allows the upgradeability owner to upgrade the current implementation of the proxy
        ///and delegatecall the new implementation for initialization.
        ///param implementation representing the of :AccountId the new implementation to be set.
        ///param data represents the msg.data to bet sent in the low level call. This parameter may include the pub fn
        ///signature of the implementation to be called with the needed payload
        #[ink(message, payable)]
        fn upgrade_to_and_call(implementation: AccountId, data: Vec<u8>);
    }

