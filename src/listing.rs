use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{PanicOnDefault};

// a project listing
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Listing {
    pub description: String,
    pub price: u128,
}

impl Listing {
    pub fn new(
        descripton: String,
        price: u128,
    ) -> Self {
        Self {
            description: descripton,
            price: price,
        }
    }
}

