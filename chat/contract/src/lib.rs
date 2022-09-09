extern crate core;

use near_sdk::{BorshStorageKey, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::collections::{UnorderedMap};
use near_sdk::env::predecessor_account_id;

pub type PhoneNumber = String;

#[derive(BorshSerialize, BorshStorageKey)]
enum KeyStore {
    IdPerNumber
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // number_per_id: UnorderedMap<PhoneNumber, AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            // number_per_id: UnorderedMap::new(KeyStore::IdPerNumber)
        }
    }

    pub fn register_account(&mut self, phone_number: PhoneNumber) {
        // let account_id = predecessor_account_id();
        //
        // if self.number_per_id.get(&phone_number).is_some() {
        //     panic!("Phone number {} already registered", phone_number)
        // }
        //
        // self.number_per_id.insert(&phone_number, &account_id);
    }

    pub fn send_message(
        &self,
        message: String,
        sender_phone_number: PhoneNumber,
        receiver_phone_number: PhoneNumber
    )
    {
        // if self.number_per_id.get(&receiver_phone_number).is_none() {
        //     panic!("Receiver phone number {} not registered", receiver_phone_number)
        // }

        // let account_id = predecessor_account_id();
        // let sender_account_id = self.number_per_id.get(&sender_phone_number)
        //     .expect("Your number is not registered");
        //
        // if account_id != sender_account_id {
        //     panic!("You are not the owner of this number: {}", sender_phone_number)
        // }
    }
}

#[cfg(test)]
mod tests {
}