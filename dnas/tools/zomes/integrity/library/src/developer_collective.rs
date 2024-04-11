use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DeveloperCollective {
    pub name: String,
    pub description: String,
    pub website: String,
    pub contact: String,
    pub icon: String,
    pub meta_data: String,
}
pub fn validate_create_developer_collective(
    _action: EntryCreationAction,
    _developer_collective: DeveloperCollective,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_developer_collective(
    _action: Update,
    _developer_collective: DeveloperCollective,
    _original_action: EntryCreationAction,
    _original_developer_collective: DeveloperCollective,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_developer_collective(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_developer_collective: DeveloperCollective,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_developer_collective_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _developer_collective: crate::DeveloperCollective = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _developer_collective: crate::DeveloperCollective = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_developer_collective_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("DeveloperCollectiveUpdates links cannot be deleted"),
        ),
    )
}