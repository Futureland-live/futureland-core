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

// These statements bring libraries to implement FT Fungible Token technology
// as well as metadata and data engineering tools
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FT_METADATA_SPEC,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128};
use super::{nep141};
use near_sdk::serde::{Serialize, Deserialize};


// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    name: String,
    description: String,
    // shares: nep141::NEP141,  // TODO: add this back later
}

impl Project {
    /// Creates a new project in the Futureland ecosystem
    /// Projects will describe endeavors on the future of cities
    /// The most important features to be implemented in a Futureland project are:
    /// [1] Version ID (year): when is the project planned to happened i.e. 2035
    /// [2] Version Token (documents): unique encrypted key containing
    /// information of uploaded documents that validated a version of the future
    /// [3] Description: project vision, inspiration, etc.
    /// [4] Visionary: who started
    /// [5] Contributors: a series of futureland accounts
    /// TODO: this list of attriutes are non exhaustive, it can change in future versions

    /// FUNCTION: Create New Project
    pub fn new(
        //owner_id: AccountId,
        //total_shares: U128,
        project_name: String,
        project_description: String,
        //project_symbol: String,
    ) -> Self {
        //project_name.push_str(&std::string::String::from(" share"));
        let this = Self {
            name: project_name.clone(),
            description: project_description,
            /*
            shares: nep141::NEP141::new(owner_id,
                                        total_shares,
                                        FungibleTokenMetadata {
                                            spec: FT_METADATA_SPEC.to_string(),
                                            name: project_name.clone(),
                                            symbol: project_symbol,
                                            icon: None,
                                            reference: None,
                                            reference_hash: None,
                                            decimals: 24,
                                            },
                                        ),
            */
        };

        /// TODO: FUNCTION: Equalize Shares
        // Calculate contribution effort, distribute shares accordingly
        // Return: list of IDs and shares

        /// TODO: FUNCTION: Upload Work
        // Ask for a job upload - get project status
        // Return: True - good to go; else RaiseError - i.e. waiting merge

        /// TODO: FUNCTION: Get New Files Information
        // Ask for latests files uploaded
        // Return: List of transactions; else RaiseError

        /// TODO: FUNCTION: Stake
        // Ask for amount in stake
        // Return: Amount stake per crypto and their APRs; else RaiseError

        /// TODO: FUNCTION: Update Project Information
        // Send data to the core project branch
        // Return: True - ok; else RaiseError

        this
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    /// TODO: extend this with more project functionality
}

/// TODO: set up tests
