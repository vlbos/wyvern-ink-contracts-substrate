// Token transfer proxy. Uses the authentication table of a ProxyRegistry contract to grant ERC20 `transferFrom` access.
//   This means that users only need to authorize the proxy contract once for all future protocol versions.
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// #[ink::contract]
// mod token_transfer_proxy {
/// Defines the storage of your contract.
/// Add new fields to the below struct in order
/// to add new static storage fields to your contract.
// #[ink(storage)]
// pub struct TokenTransferProxy {
// // Authentication registry.
//     registry: AccountId,
// }
use ink_env::AccountId;
type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;
#[ink::trait_definition]
pub trait TokenTransferProxy {
    /// Call ERC20 `transferFrom`
    /// @dev Authenticated contract only
    /// @param token ERC20 token address
    /// @param from From address
    /// @param to To address
    /// @param amount Transfer amount
    #[ink(message)]
    fn transfer_from(&mut self,
        token: AccountId,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    );
}
