/*!
 * This contract implements the Futureland project management API
 */

/*!
 * Template copied from Near's setup example project here: https://docs.near.org/docs/develop/contracts/rust/intro
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

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FT_METADATA_SPEC,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};
use near_sdk::json_types::U128;
use super::{nep141};


// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Project {
    shares: nep141::NEP141,
}

impl Project {
    /// Creates a new project in the Futureland ecosystem
    pub fn new(
        owner_id: AccountId,
        total_shares: U128,
        mut project_name: String,
        project_symbol: String,
    ) -> Self {
        project_name.push_str(&std::string::String::from(" share"));
        let this = Self {
            shares: nep141::NEP141::new(owner_id,
                                        total_shares,
                                        FungibleTokenMetadata {
                                            spec: FT_METADATA_SPEC.to_string(),
                                            name: project_name,
                                            symbol: project_symbol,
                                            icon: None,
                                            reference: None,
                                            reference_hash: None,
                                            decimals: 24,
                                        }
                                    )
        };

        // TODO: transfer some shares to Futureland

        this
    }

    // TODO: extend this with more project functionality
}

// TODO: set up tests
