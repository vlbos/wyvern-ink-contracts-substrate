//   Proxy contract to hold access to assets on behalf of a user (e.g. ERC20 approve) and execute calls under particular conditions.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod authenticated_proxy {
         use token_recipient::TokenRecipient; 
use owned_upgradeability_storage::OwnedUpgradeabilityStorage;
    /// Delegate call could be used to atomically transfer multiple assets owned by the proxy contract with one order.
    #[derive(scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub enum HowToCall {
        Call,
        DelegateCall,
    }
    /// Errors that can occur upon calling this contract.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the call failed.
        TransactionFailed,
    }

// TokenRecipient
    #[ink(event)]
    pub struct ReceivedEther {
        #[ink(topic)]
        sender: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct ReceivedTokens {
        #[ink(topic)]
        from: AccountId,
        value: Balance,
        #[ink(topic)]
        token: AccountId,
        extra_data: Vec<u8>,
    }


    /// Event fired when the proxy access is revoked or unrevoked.
    #[ink(event)]
    pub struct Revoked {
        revoked: bool,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct AuthenticatedProxy {
        /// Whether initialized.
        initialized: bool,

        /// which :AccountId owns this proxy.
        user: AccountId,

        /// Associated registry with contract authentication information.
        registry: ProxyRegistry,

        /// Whether access has been revoked.
        revoked: bool,
    }

    impl AuthenticatedProxy {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        ///Initialize an AuthenticatedProxy
        ///@param addrUser of :AccountId user on whose behalf this proxy will act
        ///@param addr_registry of :AccountId ProxyRegistry contract which will manage this proxy
        #[ink(message)]
        pub fn initialize(addrUser: AccountId, addr_registry: AccountId) {
            assert!(!self.initialized);
            self.initialized = true;
            self.user = addrUser;
            self.registry = addr_registry;
        }

        ///Set the revoked flag (allows a user to revoke ProxyRegistry access)
        ///@dev Can be called by the user only
        ///@param revoke Whether or not to revoke access
        #[ink(message)]
        pub fn set_revoke(revoke: bool) {
            assert_eq!(self.env().caller(), self.user);
            self.revoked = revoke;
            self.env().emit_event(Revoked { revoked: revoke });
        }

        ///Execute a message call from the proxy contract
        ///@dev Can be called by the user, or by a contract authorized by the registry as long as the user has not revoked access
        ///@param dest to :AccountId which the call will be sent
        ///@param how_to_call Which kind of call to make
        ///@param calldata Calldata to send
        ///@return Result of the call (success or failure)
        #[ink(message)]
        pub fn proxy(dest: AccountId, how_to_call: HowToCall, calldata: Vec<u8>) -> bool {
            assert!(
                self.env().caller() == user
                    || (!revoked && registry.contracts(self.env().caller()))
            );
        let result = false;
            if (how_to_call == HowToCall::Call) {
                // result = dest.call(calldata);
            } else if (how_to_call == HowToCall::DelegateCall) {
                // result = dest.delegatecall(calldata);
            }
            return result;
        }

        ///Execute a message call and assert success
        ///
        ///@dev Same functionality as `proxy`, just asserts the return value
        ///@param dest to :AccountId which the call will be sent
        ///@param how_to_call What kind of call to make
        ///@param calldata Calldata to send

        #[ink(message)]
        pub fn proxy_assert(dest: AccountId, how_to_call: HowToCall, calldata: Vec<u8>) {
            assert!(proxy(dest, how_to_call, calldata));
        }
    }

impl TokenRecipient for AuthenticatedProxy{

        ///@dev Receive tokens and generate a log event
        ///@param from from :AccountId which to transfer tokens
        ///@param value Amount of tokens to transfer
        ///@param token of :AccountId token
        ///@param extra_data Additional data to log
        #[ink(message)]
        pub fn receive_approval(
            from: AccountId,
            value: Balance,
            token: AccountId,
            extra_data: Vec<u8>,
        ) {
            // ERC20 t = ERC20(token);
            // require(t.transferFrom(from, this, value));
            // emit ReceivedTokens(from, value, token, extra_data);
            let gas_limit = 0;
            let transferred_value = 0;
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
                        .push_arg(self.env().account_id())
                        .push_arg(values),
                )
                .returns::<()>()
                .fire()
                .map_err(|_| Error::TransactionFailed);
            assert!(result.is_ok());
            self.env().emit_event(ReceivedTokens {
                from,
                value,
                token,
                extra_data,
            });
        }
        ///@dev Receive Ether and generate a log event
        // pub fn () payable public {
        //     emit ReceivedEther(self.env().caller(), self.env().caller());
        // }
        /// Asserts that the token amount sent as payment with this call
        /// is exactly `10`. This method will fail otherwise, and the
        /// transaction would then be reverted.
        ///
        /// # Note
        ///
        /// The method needs to be annotated with `payable`; only then it is
        /// allowed to receive value as part of the call.
        #[ink(message, payable)]
        pub fn was_it_ten(&self) {
            ink_env::debug_println!("received payment: {}", self.env().transferred_value());
            assert!(self.env().transferred_value() == 10, "payment was not ten");
            self.env().emit_event(ReceivedEther {
                sender: self.env().caller(),
                amount: self.env().transferred_value(),
            });
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
        #[ink::test]
        fn default_works() {
            let authenticated_proxy = AuthenticatedProxy::default();
            assert_eq!(authenticated_proxy.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut authenticated_proxy = AuthenticatedProxy::new(false);
            assert_eq!(authenticated_proxy.get(), false);
            authenticated_proxy.flip();
            assert_eq!(authenticated_proxy.get(), true);
        }
    }
}
