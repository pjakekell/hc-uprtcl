#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};
use holochain_wasm_utils::api_serialization::get_entry::GetEntryResult;

use hdk_proc_macros::zome;

pub mod commit;
pub mod context;
pub mod perspective;
pub mod proof;
pub mod utils;

#[zome]
mod uprtcl {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    // Entry definitions

    #[entry_def]
    fn commit_entry_def() -> ValidatingEntryType {
        commit::definition()
    }

    #[entry_def]
    fn perspective_entry_def() -> ValidatingEntryType {
        perspective::definition()
    }

    #[entry_def]
    fn context_entry_def() -> ValidatingEntryType {
        context::definition()
    }

    // Create entries

    #[zome_fn("hc_public")]
    fn create_commit(commit_data: commit::CommitData) -> ZomeApiResult<Address> {
        commit::create_commit(commit_data)
    }

    #[zome_fn("hc_public")]
    fn create_perspective(
        perspective_data: perspective::PerspectiveData,
    ) -> ZomeApiResult<Address> {
        perspective::create_perspective(perspective_data)
    }

    #[zome_fn("hc_public")]
    fn create_context(context_data: context::ContextData) -> ZomeApiResult<Address> {
        context::create_context(context_data)
    }

    // Clone entries

    #[zome_fn("hc_public")]
    fn clone_commit(
        previous_address: Option<Address>,
        commit: commit::Commit,
    ) -> ZomeApiResult<Address> {
        utils::clone_entry(previous_address, commit)
    }

    #[zome_fn("hc_public")]
    fn clone_perspective(
        previous_address: Option<Address>,
        perspective: perspective::Perspective,
    ) -> ZomeApiResult<Address> {
        utils::clone_entry(previous_address, perspective)
    }
    #[zome_fn("hc_public")]
    fn clone_context(
        previous_address: Option<Address>,
        context: context::Context,
    ) -> ZomeApiResult<Address> {
        utils::clone_entry(previous_address, context)
    }

    // Getters

    #[zome_fn("hc_public")]
    fn get_head(perspective_address: Address) -> ZomeApiResult<Option<Address>> {
        perspective::get_head(perspective_address)
    }

    #[zome_fn("hc_public")]
    fn get_context_perspectives(
        context_address: Address,
    ) -> ZomeApiResult<Vec<ZomeApiResult<GetEntryResult>>> {
        context::get_context_perspectives(context_address)
    }

    // Setters
    #[zome_fn("hc_public")]
    fn update_head(perspective_address: Address, head_address: Address) -> ZomeApiResult<()> {
        perspective::update_head(perspective_address, head_address)
    }

    #[zome_fn("hc_public")]
    fn update_perspective_context(
        perspective_address: Address,
        context_address: Address,
    ) -> ZomeApiResult<()> {
        context::update_perspective_context(perspective_address, context_address)
    }

}
