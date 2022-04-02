//   Token recipient. Modified very slightly from the example on http://ethereum.org/dao (just to index log parameters).

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

///@title TokenRecipient
///@author Project Wyvern Developers
    use ink_prelude::vec::Vec;

use ink_env::AccountId;
type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;

#[ink::trait_definition]
pub trait TokenRecipient {
        /// Receive tokens and generate a log event
        /// from from :AccountId which to transfer tokens
        /// value Amount of tokens to transfer
        /// token of :AccountId token
        /// extra_data Additional data to log
        #[ink(message)]
        fn receive_approval(&self,   from: AccountId,
            value: Balance,
            token: AccountId,
            extra_data: Vec<u8>,
        );

        /// Receive Ether and generate a log event
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
        fn was_it_ten(&self);

}
