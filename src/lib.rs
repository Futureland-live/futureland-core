/*!
 * This contract implements the Futureland ecosystem API
 */

/*!
 * Reference: Near's setup example project here: https://docs.near.org/docs/develop/contracts/rust/intro
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

mod nep141;
mod project;

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FT_METADATA_SPEC,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::json_types::{U128, U64};
use near_sdk::collections::Vector;

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

// some useful constants
const INITIAL_FLC_QUANTITY: u128 = 1000000;

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Futureland {
    flc: nep141::NEP141, // TODO: add this back in later
    projects: Vector<project::Project>,
    id_counter: u64,
}

#[near_bindgen]
impl Futureland {
    /// Creates an instance of the Futureland ecosystem and transfers some FLC to the creator
    #[init]
    pub fn new(
        // owner_id: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        let owner_id = env::signer_account_id();

        // Creates FutureLand Coin
        let this = Self {
            flc: nep141::NEP141::new(owner_id,
                                    U128::from(INITIAL_FLC_QUANTITY),
                                    FungibleTokenMetadata {
                                        spec: FT_METADATA_SPEC.to_string(),
                                        name: "FutureLand Coin".to_string(),
                                        symbol: "FLC".to_string(),
                                        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()), // TODO: change this to a different icon
                                        reference: None,
                                        reference_hash: None,
                                        decimals: 24,
                                    }),
            projects: Vector::new(0),
            id_counter:0,
        };

        this
    }

    /// Create a new project
    pub fn create_project(
        &mut self,
        // owner_id: AccountId,
        // total_shares: U128,
        project_name: String,
        project_description: String,
        // project_symbol: String
    ) -> U64 {
        self.projects.push(&project::Project::new(project_name, project_description));

        println!("Created project with id: {}", self.id_counter);

        self.id_counter += 1;

        U64::from(self.id_counter - 1)
    }

    /// retrieve a project name by its id
    pub fn get_project_name(
        &self,
        project_id: u64,
    ) -> String {
        self.projects.get(project_id).unwrap().get_name()
    }

    /// retrive a project description by its id
    pub fn get_project_description(
        &self,
        project_id: u64,
    ) -> String {
        self.projects.get(project_id).unwrap().get_description()
    }

    /// retrive a project by its id
    pub fn get_project(
        &self,
        project_id: u64,
    ) -> project::Project {
        self.projects.get(project_id).unwrap()
    }

    /// TODO: expand this with more functionality

    /// TODO: Connect APIs with core Project functions, here they are:
    /// Equalize Shares
    /// Upload Work
    /// Get New Files Information
    /// Stake
    ///Update Project Information

}

// TODO: set up tests
