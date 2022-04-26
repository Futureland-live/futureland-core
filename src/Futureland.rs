/*!
 * This contract implements the Futureland ecosystem API
 */

/*!
 * Copied from Near's setup example project here: https://docs.near.org/docs/develop/contracts/rust/intro
 */

/*!
 * Original License:
 * 
 * Copyright 2020 NEAR Inc
 * 
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 * 
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
mod NEP141;

near_sdk::setup_alloc!();

// some useful constants
const INITIAL_FLC_QUANTITY: u128 = 1000000;

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Futureland {
    flc: NEP141::NEP141,
}

#[near_bindgen]
impl Futureland {
    /// Creates an instance of the Futureland ecosystem and transfers some FLC to the creator
    #[init]
    pub fn new(
        owner_id: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        /// Creates FutureLand Coin
        let mut this = Self {
            flc: NEP141::new(owner_id,
                             INITIAL_FLC_QUANTITY,
                             FungibleTokenMetadata {
                                spec: FT_METADATA_SPEC.to_string(),
                                name: "FutureLand Coin".to_string(),
                                symbol: "FLC".to_string(),
                                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()), // TODO: change this to a different icon
                                reference: None,
                                reference_hash: None,
                                decimals: 24,
                             }),
        };

        this
    }

    // TODO: expand this with more functionality
}

// TODO: set up tests
