use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DeveloperCollective {
    pub name: String,
    pub description: String,
    pub website: String,
    pub contact: String,
    pub icon: String,
    pub meta_data: Option<String>,
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
        return Ok(
            ValidateCallbackResult::Invalid(
                "Links to DeveloperCollective entry updates can only be created by the agent that created the DeveloperCollective entry."
                    .into(),
            ),
        );
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
/// Rules
/// 1. Link must point a way from the all_developer_collectives anchor
/// 2. Link must point to a DeveloperCollective entry action hash
/// 3. Only the agent that created the DeveloperCollective entry can create a link from the all_developer_collectives anchor
///    to the DeveloperCollective entry
pub fn validate_create_link_all_developer_collectives(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check that base address is pointing away from the all_developer_collectives anchor
    let base_address_entry_hash = EntryHash::try_from(base_address).map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Base address is not an entry hash".into()
        ))
    })?;
    let path = Path::from("all_developer_collectives");
    if path.path_entry_hash()? != base_address_entry_hash {
        return Ok(ValidateCallbackResult::Invalid(
            "AllDeveloperCollectives link is not pointing away from the all_developer_collectives anchor."
                .into(),
        ));
    }

    // Check the entry type for the given action hash
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _developer_collective: crate::DeveloperCollective = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference a DeveloperCollective entry".to_string()
        )))?;

    if &action.author != record.action().author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Links from the all_developer_collectives anchor to a DeveloperCollective entry can only be created by the agent that created the DeveloperCollective entry.".into(),
        ));
    }
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
/// Rules
/// 1. Only the agent that created the link (and therefore created the DeveloperCollective entry according
///    to the rules when creating those links) can delete the link
pub fn validate_delete_link_all_developer_collectives(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author != original_action.author {
        return Ok(ValidateCallbackResult::Invalid(
            "Links from the all_developer_collectives anchor to a DeveloperCollective entry can only be deleted by the agent that created the link and therefore the DeveloperCollective entry.".into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
