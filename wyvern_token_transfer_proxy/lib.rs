#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod wyvern_token_transfer_proxy {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct WyvernTokenTransferProxy {
        // Authentication registry.
        registry: AccountId,
    }

    impl WyvernTokenTransferProxy {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }
    }

    impl TokenTransferProxy for WyvernTokenTransferProxy {
        /// Call ERC20 `transferFrom`
        /// @dev Authenticated contract only
        /// @param token ERC20 token address
        /// @param from From address
        /// @param to To address
        /// @param amount Transfer amount
        #[ink(message)]
        pub fn transfer_from(
            token: AccountId,
            from: AccountId,
            to: AccountId,
            amount: Balance,
        ) -> bool {
            require(registry.contracts(self.env().caller()));
            return ERC20(token).transferFrom(from, to, amount);
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

        /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let wyvern_token_transfer_proxy = WyvernTokenTransferProxy::default();
        //     assert_eq!(wyvern_token_transfer_proxy.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut wyvern_token_transfer_proxy = WyvernTokenTransferProxy::new(false);
        //     assert_eq!(wyvern_token_transfer_proxy.get(), false);
        //     wyvern_token_transfer_proxy.flip();
        //     assert_eq!(wyvern_token_transfer_proxy.get(), true);
        // }
    }
}
