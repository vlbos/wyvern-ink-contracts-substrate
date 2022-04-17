// This contract keeps track of the upgradeability owner
#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_env::Hash;
use ink_lang as ink;
#[ink::trait_definition]
pub trait OwnedUpgradeabilityStorage {
    ///dev Tells the of :AccountId the owner
    ///return the of :AccountId the owner
    #[ink(message)]
    fn upgradeability_owner(&self) -> AccountId;

    ///dev Sets the of :AccountId the owner
    // #[ink(message)]
    // fn set_upgradeability_owner(&mut self, new_upgradeability_owner: AccountId);

    ///dev Tells the of :AccountId the current implementation
    ///return of :AccountId the current implementation
    #[ink(message)]
    fn implementation(&self) -> Hash;

    ///dev Tells the proxy type (EIP 897)
    ///return Proxy type, 2 for forwarding proxy
    #[ink(message)]
    fn proxy_type(&self) -> u32;
}
