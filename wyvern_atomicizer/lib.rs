//! # WyvernAtomicizer Wallet
//!
//! This implements a plain multi owner wallet.
//!
//! ## Warning
//!
//! This contract is an *example*. It is neither audited nor endorsed for production use.
//! Do **not** rely on it to keep anything of value secure.
//!
//! ## Overview
//!
//! Each instantiation of this contract has a set of `owners` and a `requirement` of
//! how many of them need to agree on a `Transaction` for it to be able to be executed.
//! Every owner can submit a transaction and when enough of the other owners confirm
//! it will be able to be executed. The following invariant is enforced by the contract:
//!
//! ```ignore
//! 0 < requirement && requirement <= owners && owners <= MAX_OWNERS
//! ```
//!
//! ## Error Handling
//!
//! With the exception of `execute_transaction` no error conditions are signalled
//! through return types. Any error or invariant violation triggers a panic and therefore
//! rolls back the transaction.
//!
//! ## Interface
//!
//! The interface is modelled after the popular Gnosis wyvern_atomicizer wallet. However, there
//! are subtle variations from the interface. For example the `confirm_transaction`
//! will never trigger the execution of a `Transaction` even if the threshold is reached.
//! A call of `execute_transaction` is always required. This can be called by anyone.
//!
//! All the messages that are declared as only callable by the wallet must go through
//! the usual submit, confirm, execute cycle as any other transaction that should be
//! called by the wallet. For example, to add an owner you would submit a transaction
//! that calls the wallets own `add_owner` message through `submit_transaction`.
//!
//! ### Owner Management
//!
//! The messages `add_owner`, `remove_owner`, and `replace_owner` can be used to manage
//! the owner set after instantiation.
//!
//! ### Changing the Requirement
//!
//! `change_requirement` can be used to tighten or relax the `requirement` of how many
//! owner signatures are needed to execute a `Transaction`.
//!
//! ### Transaction Management
//!
//! `submit_transaction`, `cancel_transaction`, `confirm_transaction`,
//! `revoke_confirmation` and `execute_transaction` are the bread and butter messages
//! of this contract. Use them to dispatch arbitrary messages to other contracts
//! with the wallet as a sender.

#![cfg_attr(not(feature = "std"), no_std)]

pub use self::wyvern_atomicizer::WyvernAtomicizer;
use ink_lang as ink;

#[ink::contract]
mod wyvern_atomicizer {
    use ink_env::call::{build_call, Call, ExecutionInput};
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{PackedLayout, SpreadAllocate, SpreadLayout},
        Mapping,
    };
    use scale::Output;

    /// Tune this to your liking but be wary that allowing too many owners will not perform well.
    const MAX_OWNERS: u32 = 50;

    type TransactionId = u32;
    const WRONG_TRANSACTION_ID: &str = "The user specified an invalid transaction id. Abort.";

    /// A wrapper that allows us to encode a blob of Vec<u8>.
    ///
    /// We use this to pass the set of untyped (Vec<u8>) parameters to the `CallBuilder`.
    struct CallInput<'a>(&'a [u8]);

    impl<'a> scale::Encode for CallInput<'a> {
        fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
            dest.write(self.0);
        }
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the call failed.
        TransactionFailed,
    }

    /// Emitted when an owner confirms a transaction.
    #[ink(event)]
    pub struct Confirmation {
        /// The transaction that was confirmed.
        #[ink(topic)]
        selector: [u8; 4],
        /// The owner that sent the confirmation.
        #[ink(topic)]
        from: AccountId,
        /// The confirmation status after this confirmation was applied.
        #[ink(topic)]
        to: AccountId,
    }

    /// Emitted when a transaction was executed.
    #[ink(event)]
    pub struct Execution {
        /// The transaction that was executed.
        #[ink(topic)]
        callee: AccountId,

        #[ink(topic)]
        value: Balance,
        /// Indicates whether the transaction executed successfully. If so the `Ok` value holds
        /// the output in Vec<u8>. The Option is `None` when the transaction was executed through
        /// `invoke_transaction` rather than `evaluate_transaction`.
        #[ink(topic)]
        result: Result<Option<Vec<u8>>, Error>,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct WyvernAtomicizer {}
    impl WyvernAtomicizer {
        /// The only constructor of the contract.
        ///
        /// A list of owners must be supplied and a number of how many of them must
        /// confirm a transaction. Duplicate owners are silently dropped.
        ///
        /// # Panics
        ///
        /// If `requirement` violates our invariant.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {})
        }

        /// Evaluate a confirmed execution and return its output as Vec<u8>.
        ///
        /// Its return value indicates whether the called transaction was successful and contains
        /// its output when successful.
        /// This can be called by anyone.
        #[ink(message, payable)]
        pub fn atomicize(
            &mut self,
            selector: [u8; 4],
            callees: Vec<AccountId>,
            from: AccountId,
            to: AccountId,
            values: Vec<Balance>,
        ) -> Result<(), Error> {
            let transferred_value = Balance::default();
            let gas_limit = 0;

            for (i, &callee) in callees.iter().enumerate() {
                let result = build_call::<<Self as ::ink_lang::reflect::ContractEnv>::Env>()
                    .call_type(
                        Call::new()
                            .callee(callee)
                            .gas_limit(gas_limit)
                            .transferred_value(transferred_value),
                    )
                    .exec_input(
                        ExecutionInput::new(selector.into())
                            .push_arg(from)
                            .push_arg(to)
                            .push_arg(values[i]),
                    )
                    .returns::<()>()
                    .fire()
                    .map_err(|_| Error::TransactionFailed);
                self.env().emit_event(Execution {
                    callee,
                    value: values[i],
                    result: result.map(|_| None),
                });
            }
            self.env().emit_event(Confirmation { selector, from, to });
            Ok(())
        }

        /// Evaluate a confirmed execution and return its output as Vec<u8>.
        ///
        /// Its return value indicates whether the called transaction was successful and contains
        /// its output when successful.
        /// This can be called by anyone.
        #[ink(message, payable)]
        pub fn eval_atomicize(
            &mut self,
            selector: [u8; 4],
            callees: Vec<AccountId>,
            from: AccountId,
            to: AccountId,
            values: Vec<Balance>,
        ) -> Result<(), Error> {
            let transferred_value = Balance::default();
            let gas_limit = 0;
            for (i, &callee) in callees.iter().enumerate() {
                let result = build_call::<<Self as ::ink_lang::reflect::ContractEnv>::Env>()
                    .call_type(
                        Call::new()
                            .callee(callee)
                            .gas_limit(gas_limit)
                            .transferred_value(transferred_value),
                    )
                    .exec_input(
                        ExecutionInput::new(selector.into())
                            .push_arg(from)
                            .push_arg(to)
                            .push_arg(values[i]),
                    )
                    .returns::<Vec<u8>>()
                    .fire()
                    .map_err(|_| Error::TransactionFailed);
                self.env().emit_event(Execution {
                    callee,
                    value: values[i],
                    result: result.clone().map(Some),
                });
            }
            self.env().emit_event(Confirmation { selector, from, to });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::{call, test};
        use ink_lang as ink;

        const WALLET: [u8; 32] = [7; 32];

        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }

        fn set_from_wallet() {
            let callee = AccountId::from(WALLET);
            set_caller(callee);
        }

        fn set_from_owner() {
            let accounts = default_accounts();
            set_caller(accounts.alice);
        }

        fn set_from_no_owner() {
            let accounts = default_accounts();
            set_caller(accounts.django);
        }

        fn default_accounts() -> test::DefaultAccounts<Environment> {
            ink_env::test::default_accounts::<Environment>()
        }

        fn build_contract() -> WyvernAtomicizer {
            // Set the contract's address as `WALLET`.
            let callee: AccountId = AccountId::from(WALLET);
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(callee);

            let accounts = default_accounts();
            let owners = vec![accounts.alice, accounts.bob, accounts.eve];
            WyvernAtomicizer::new()
        }
    }
}
