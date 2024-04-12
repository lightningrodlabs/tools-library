use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ContributorPermission {
    pub for_collective: ActionHash,
    pub for_agent: AgentPubKey,
    pub expiry: Option<Timestamp>,
}
pub fn validate_create_contributor_permission(
    action: EntryCreationAction,
    contributor_permission: ContributorPermission,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(contributor_permission.for_collective.clone())?;
    if record.action().author() != action.author() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of the developer collective can issue contributor permissions for it."
                    .into(),
            ),
        );
    }
    let _developer_collective: crate::DeveloperCollective = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_contributor_permission(
    _action: Update,
    _contributor_permission: ContributorPermission,
    _original_action: EntryCreationAction,
    _original_contributor_permission: ContributorPermission,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Contributor Permissions cannot be updated",
    )))
}
pub fn validate_delete_contributor_permission(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_contributor_permission: ContributorPermission,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Contributor Permissions cannot be deleted",
    )))
}
/// Rules
/// 1. Only the creator of a DeveloperCollective entry is allowed to create links to ContributorPermission
///    entries
/// 2. Links from a DeveloperCollective can only point to a ContributorPermission for that same
///    DeveloperCollective entry
/// 3. A link from a DeveloperCollective to a ContributorPermission must contain in its tag the AgentPubKey
///    of the agent that the ContributorPermission is for
pub fn validate_create_link_developer_collective_to_contributor_permissions(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let developer_collective_action_hash =
        base_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let developer_collective_record =
        must_get_valid_record(developer_collective_action_hash.clone())?;
    if developer_collective_record.action().author() != &action.author {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of the developer collective is allowed to create links to contributor permissions."
                    .into(),
            ),
        );
    }
    let _developer_collective: crate::DeveloperCollective = developer_collective_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    let contributor_permission_action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let contributor_permission_record = must_get_valid_record(contributor_permission_action_hash)?;
    let contributor_permission: crate::ContributorPermission = contributor_permission_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    // Validate that the tag is for the agent that the permission is for
    let agent_in_tag = AgentPubKey::from_raw_39(tag.0.clone()).map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Link tag does not contain a valid agent public key".into()
        ))
    })?;

    if agent_in_tag != contributor_permission.for_agent {
        return Ok(ValidateCallbackResult::Invalid(
            "Link tag contains the wrong agent public key.".into(),
        ));
    }

    if contributor_permission.for_collective != developer_collective_action_hash {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Links from developer collectives can only point to contributor permissions for that same developer collective"
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_developer_collective_to_contributor_permissions(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "DeveloperCollectiveToContributorPermissions links cannot be deleted",
    )))
}
/// Rules
/// 1. Only the creator of a DeveloperCollective entry is allowed to create links to ContributorPermission
///    entries
/// 2. Links from an agent can only point to a ContributorPermission for that same agent
/// 3. A link from an agent to a ContributorPermission must contain in its tag the ActionHash
///    of the DeveloperCollective that the ContributorPermission is for
pub fn validate_create_link_contributor_to_contributor_permissions(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let contributor_permission_record = must_get_valid_record(action_hash)?;
    if contributor_permission_record.action().author() != &action.author {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of the developer collective is allowed to create links from contributor agents to their permissions."
                    .into(),
            ),
        );
    }
    let contributor_permission: crate::ContributorPermission = contributor_permission_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry.".to_string()
        )))?;
    let contributor_agent_key =
        base_address
            .into_agent_pub_key()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No agent hash as link base.".to_string()
            )))?;

    // Validate that the tag is for the DeveloperCollective that the permission is for
    let developer_collective_in_tag = ActionHash::from_raw_39(tag.0.clone()).map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Link tag does not contain a valid action hash".into()
        ))
    })?;

    if developer_collective_in_tag != contributor_permission.for_collective {
        return Ok(ValidateCallbackResult::Invalid(
            "Link tag does not contain the action hash of the DeveloperCollective for which the ContributorPermission is for.".into(),
        ));
    }

    if contributor_permission.for_agent != contributor_agent_key {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Links from agents to contributor permissions can only be created from agents the permission refers to."
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_contributor_to_contributor_permissions(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ContributorToContributorPermissions links cannot be deleted",
    )))
}
