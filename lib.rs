#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Incrementer {
        value: ink_storage::Mapping<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // This call is required to correctly initialize the mapping of the contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.value.insert(&caller, &init_value);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            let caller = self.env().caller();
            let value = self.get();
            self.value.insert(caller, &(value + by));
        }

        #[ink(message)]
        pub fn remove(&mut self) {
            self.value.remove(&self.env().caller())
        }

        // Get the number associated with the caller's AccountId, if it exists
        #[ink(message)]
        pub fn get(&self) -> i32 {
            let caller = Self::env().caller();
            self.value.get(&caller).unwrap_or_default()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn initialises() {
            let contract = Incrementer::new(5);
            assert_eq!(contract.get(), 5);
        }

        #[ink::test]
        fn inc_works() {
            let mut contract = Incrementer::new(1);
            assert_eq!(contract.get(), 1);
            contract.inc(5);
            assert_eq!(contract.get(), 6);
            contract.inc(5);
            assert_eq!(contract.get(), 11);
        }

        #[ink::test]
        fn remove_works() {
            let mut contract = Incrementer::new(1);
            assert_eq!(contract.get(), 1);
            contract.inc(5);
            assert_eq!(contract.get(), 6);
            contract.remove();
            assert_eq!(contract.get(), 0);
        }
    }
}
