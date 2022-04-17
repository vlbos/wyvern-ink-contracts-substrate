#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod wyvern_token_transfer_proxy {

    use ink_env::call::{build_call, Call,  ExecutionInput};
    use ink_prelude::vec::Vec;

    // use token_transfer_proxy::TokenTransferProxy;

   /// Errors that can occur upon calling this contract.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the call failed.
        TransactionFailed,
    }
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
        pub fn new(registry: AccountId) -> Self {
            Self { registry }
        }
    // }

    // impl TokenTransferProxy for WyvernTokenTransferProxy {

        /// Call ERC20 `transferFrom`
        ///  Authenticated contract only
        /// token ERC20 token address
        /// from From address
        /// to To address
        /// amount Transfer amount
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            token: AccountId,
            from: AccountId,
            to: AccountId,
            amount: Balance,
        )  {
            // require(registry.contracts(self.env().caller()));
            // return ERC20(token).transferFrom(from, to, amount);
            let transferred_value = Balance::default();
            let gas_limit = 0;
            let contracts_selector = [0x80, 0x05, 0xa4, 0x70];
            let result = build_call::<<Self as ::ink_lang::reflect::ContractEnv>::Env>()
                .call_type(
                    Call::new()
                        .callee(self.registry)
                        .gas_limit(gas_limit)
                        .transferred_value(transferred_value),
                )
                .exec_input(
                    ExecutionInput::new(contracts_selector.into())
                        .push_arg(self.env().caller()),
                       )
                .returns::<Vec<u8>>()
                .fire()
                .map_err(|_| Error::TransactionFailed);
            assert!(result.is_ok());

           let selector = [0x0b, 0x39, 0x6f, 0x18];
            let result = build_call::<<Self as ::ink_lang::reflect::ContractEnv>::Env>()
                .call_type(
                    Call::new()
                        .callee(token)
                        .gas_limit(gas_limit)
                        .transferred_value(transferred_value),
                )
                .exec_input(
                    ExecutionInput::new(selector.into())
                        .push_arg(from)
                        .push_arg(to)
                        .push_arg(amount),
                )
                .returns::<()>()
                .fire()
                .map_err(|_| Error::TransactionFailed);
            assert!(result.is_ok());
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
    }
}
