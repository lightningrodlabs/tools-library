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
    action: Update,
    _developer_collective: DeveloperCollective,
    original_action: EntryCreationAction,
    _original_developer_collective: DeveloperCollective,
) -> ExternResult<ValidateCallbackResult> {
    if &action.author != original_action.author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Developer collective entry can only be updated by the creator of the collective."
                .into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_developer_collective(
    action: Delete,
    original_action: EntryCreationAction,
    _original_developer_collective: DeveloperCollective,
) -> ExternResult<ValidateCallbackResult> {
    if &action.author != original_action.author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Developer collective entry can only be deleted by the creator of the collective."
                .into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_developer_collective_updates(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let collective_action_hash =
        base_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let collective_record = must_get_valid_record(collective_action_hash)?;

    if collective_record.action().author() != &action.author {
        return Ok(ValidateCallbackResult::Invalid(
            "Links to DeveloperCollective entry updates can only be created by the agent that created the DeveloperCollective entry.".into(),
        ));
    }

    let _developer_collective: crate::DeveloperCollective = collective_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Link base must be DeveloperCollective entry".to_string()
        )))?;

    let collective_update_action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let collective_update_record = must_get_valid_record(collective_update_action_hash)?;
    let _developer_collective: crate::DeveloperCollective = collective_update_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Link target must be DeveloperCollective entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_developer_collective_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "DeveloperCollectiveUpdates links cannot be deleted",
    )))
}
