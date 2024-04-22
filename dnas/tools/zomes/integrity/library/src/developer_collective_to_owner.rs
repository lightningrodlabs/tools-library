use hdi::prelude::*;
/// Rules:
///
/// 1. Only the creator (owner) of the developer collective can create a link to their public key
pub fn validate_create_link_developer_collective_to_owner(
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

    let _developer_collective: crate::DeveloperCollective = collective_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // Check the entry type for the given action hash
    let owner_agent_key =
        target_address
            .into_agent_pub_key()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No agent public key associated with link".to_string()
            )))?;

    if owner_agent_key != action.author {
        return Ok(ValidateCallbackResult::Invalid("Only the creator of a DeveloperCollective can create a DeveloperCollectiveToOwner link from that DeveloperCollective".into()));
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_developer_collective_to_owner(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author != original_action.author {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the creator of a DeveloperCollectiveToOwner link can delete that link.".into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
