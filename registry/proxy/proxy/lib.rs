//! Gives the possibility to delegate any call to a foreign implementation.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
#[ink::trait_definition]
pub trait Proxy {
    ///dev Tells the of :AccountId the implementation where every call will be delegated.
    ///return of :AccountId the implementation to which it will be delegated
    #[ink(message)]
    fn implementation(&self) -> AccountId;
    ///dev Tells the type of proxy (EIP 897)
    ///return Type of proxy, 2 for upgradeable proxy
    #[ink(message)]
    pub fn proxy_type(&self) -> u32;
}
