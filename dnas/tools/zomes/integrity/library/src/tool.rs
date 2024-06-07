use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Tool {
    pub developer_collective: ActionHash,
    pub permission_hash: ActionHash, // Either the CreateAction hash of the DeveloperCollective entry or an ActionHash of a ContributorPermission entry
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub icon: String, // base64 string
    pub version: String,
    pub source: String, // JSON string containing information about where to get this Tool from
    pub hashes: String, // Hashes related to this Tool to verify its integrity
    pub changelog: Option<String>,
    pub meta_data: Option<String>,
    pub deprecation: Option<String>,
}
/// Rules:
/// 1. Only the creator of a DeveloperCollective entry or an agent with a valid ContributorPermission
///    can create a Tool for a DeveloperCollective
pub fn validate_create_tool(
    action: EntryCreationAction,
    tool: Tool,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(tool.developer_collective.clone())?;
    let _developer_collective: crate::DeveloperCollective = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;

    validate_contributor_permission(
        tool.permission_hash,
        action.author().clone(),
        tool.developer_collective,
        action.timestamp().clone(),
    )
}

/// Rules:
/// 1. The developer_collective field may never be changed
/// 2. Only the creator of a DevloperCollective or agents with ContributorPermission for
///    the DeveloperCollective the Tool is published under (by being referenced in the
///    developer_collective field) are allowed to update a Tool
pub fn validate_update_tool(
    action: Update,
    tool: Tool,
    _original_action: EntryCreationAction,
    original_tool: Tool,
) -> ExternResult<ValidateCallbackResult> {
    if tool.developer_collective != original_tool.developer_collective {
        return Ok(ValidateCallbackResult::Invalid(
            "The developer_collective field may not be updated.".into(),
        ));
    }

    validate_contributor_permission(
        tool.permission_hash,
        action.author,
        original_tool.developer_collective,
        action.timestamp,
    )
}
/// Rules:
/// 1. Only the creator of a DeveloperCollective entry or the agent that originally created the Tool
///    can delete a Tool for a DeveloperCollective
pub fn validate_delete_tool(
    action: Delete,
    original_action: EntryCreationAction,
    original_tool: Tool,
) -> ExternResult<ValidateCallbackResult> {
    if &action.author == original_action.author() {
        Ok(ValidateCallbackResult::Valid)
    } else {
        let collective_record = must_get_valid_record(original_tool.developer_collective)?;
        if collective_record.action().author() == &action.author {
            return Ok(ValidateCallbackResult::Valid);
        }
        Ok(ValidateCallbackResult::Invalid("Only the creator of the DeveloperCollective entry or the creator of the link is allowed to delete a link from a DeveloperCollective entry to a Tool entry".into()))
    }
}
/// Rules:
/// 1. Only the creator of a DeveloperCollective entry or an agent with a valid ContributorPermission
///    can create a link from a DeveloperCollective entry to a Tool entry
/// 2. The link must be pointing to a tool which is indeed published under the
///    DeveloperCollective referenced in the base address of the link
pub fn validate_create_link_developer_collective_to_tools(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let permission_action_hash = match ActionHash::from_raw_39(tag.0) {
        Ok(ah) => ah,
        Err(e) => {
            return Ok(ValidateCallbackResult::Invalid(format!(
                "Link tag does not contain a valid action hash. Conversion failed with error: {}",
                e
            )))
        }
    };

    let collective_action_hash =
        base_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;

    validate_contributor_permission(
        permission_action_hash,
        action.author,
        collective_action_hash.clone(),
        action.timestamp,
    )?;

    let collective_record = must_get_valid_record(collective_action_hash.clone())?;
    let _developer_collective: crate::DeveloperCollective = collective_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    let tool_action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let tool_record = must_get_valid_record(tool_action_hash)?;
    let tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    // Validate that the link is pointing to a tool which is indeed published under the
    // DeveloperCollective referenced in the base address of the link
    if tool.developer_collective != collective_action_hash {
        return Ok(ValidateCallbackResult::Invalid("Link from a DeveloperCollective entry to a Tool can only be created if the Tool references that same DeveloperCollective entry in the developer_collective field.".into()));
    }

    Ok(ValidateCallbackResult::Valid)
}

/// Rules:
/// 1. Only the creator of the referenced DeveloperCollective entry or the agent that originally created
///    the link can delete such a link
pub fn validate_delete_link_developer_collective_to_tools(
    action: DeleteLink,
    original_action: CreateLink,
    base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Only the agent that created the link or the creator of the DeveloperCollective entry can remove such a link
    if action.author == original_action.author {
        Ok(ValidateCallbackResult::Valid)
    } else {
        let collective_action_hash = base
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Link base is not an action hash. This link should never have passed validation in the first place!".to_string()
            )))?;

        let collective_record = must_get_valid_record(collective_action_hash)?;
        if collective_record.action().author() == &action.author {
            return Ok(ValidateCallbackResult::Valid);
        }

        Ok(ValidateCallbackResult::Invalid("Only the creator of the DeveloperCollective entry or the creator of the link is allowed to delete a link from a DeveloperCollective entry to a Tool entry".into()))
    }
}

/// Rules:
/// 1. Tool update links can only be created by agents with ContributorPermission for the DeveloperCollective
///    that the Tool in published under. We can assume that contributors of a developer collective do not have any
///    incentive to create fake UpdateLinks as they could just as well update the Tool itself. So we don't
///    need to add more complex validation logic to ensure that the update link poinst away from the right
///    Tool to the actual update of the Tool
pub fn validate_create_link_tool_updates(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let permission_action_hash = match ActionHash::from_raw_39(tag.0) {
        Ok(ah) => ah,
        Err(e) => {
            return Ok(ValidateCallbackResult::Invalid(format!(
                "Link tag does not contain a valid action hash. Conversion failed with error: {}",
                e
            )))
        }
    };

    let tool_action_hash =
        base_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let tool_record = must_get_valid_record(tool_action_hash)?;

    let tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    validate_contributor_permission(
        permission_action_hash,
        action.author,
        tool.developer_collective,
        action.timestamp,
    )?;

    // Check the entry type for the given action hash
    let tool_update_action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let tool_update_record = must_get_valid_record(tool_update_action_hash)?;
    let _tool: crate::Tool = tool_update_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    Ok(ValidateCallbackResult::Valid)
}

/// Rules:
/// 1. Cannot be deleted.
pub fn validate_delete_link_tool_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ToolUpdates links cannot be deleted",
    )))
}

/// Rules:
/// 1. Tool to Tag links can only be created by agents with ContributorPermission for the DeveloperCollective
///    that the Tool in published under.
/// 2. Tool tag link must point to an EntryHash
pub fn validate_create_link_tool_to_tag(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let permission_action_hash = match ActionHash::from_raw_39(tag.0) {
        Ok(ah) => ah,
        Err(e) => {
            return Ok(ValidateCallbackResult::Invalid(format!(
                "Link tag does not contain a valid action hash. Conversion failed with error: {}",
                e
            )))
        }
    };

    let tool_action_hash =
        base_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;

    let tool_record = must_get_valid_record(tool_action_hash)?;

    let tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    validate_contributor_permission(
        permission_action_hash,
        action.author,
        tool.developer_collective,
        action.timestamp,
    )?;

    // Check that the target is an entry hash
    match target_address
        .into_entry_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Link target is not an entry hash".to_string()
        ))) {
        Ok(_) => Ok(ValidateCallbackResult::Valid),
        Err(e) => Ok(ValidateCallbackResult::Invalid(e.into())),
    }
}

/// Rules:
/// 1. Tool to Tag links can only be deleted by agents with ContributorPermission for the DeveloperCollective
///    that the Tool is published under. To provide the permission hash they must have commited a
///    ContributorPermissionClaim as the previous action
pub fn validate_delete_link_tool_to_tag(
    action: DeleteLink,
    _original_action: CreateLink,
    base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(action.prev_action)?;

    let contributor_permission_claim: crate::ContributorPermissionClaim = match record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Previous action is not a ContributorPermissionClaim Create action"
        )))) {
        Ok(claim) => claim,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.into())),
    };

    let tool_action_hash = base
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;

    let tool_record = must_get_valid_record(tool_action_hash)?;

    let tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;

    validate_contributor_permission(
        contributor_permission_claim.permission_hash,
        action.author,
        tool.developer_collective,
        action.timestamp,
    )
}

/// Validates for
pub fn validate_contributor_permission(
    permission_hash: ActionHash,
    agent: AgentPubKey,
    developer_collective_hash: ActionHash,
    timestamp: Timestamp,
) -> ExternResult<ValidateCallbackResult> {
    let permission_record = must_get_valid_record(permission_hash.clone())?;

    // If it's the creator of the DeveloperCollective, i.e. the permission hash is the
    // Create action of the DeveloperCollective itself, get the DeveloperCollective entry
    // and validate that this agent is indeed the creator of the DeveloperCollective
    if permission_hash == developer_collective_hash {
        let developer_collective_record = must_get_valid_record(developer_collective_hash)?;
        if developer_collective_record.action().author() == &agent {
            Ok(ValidateCallbackResult::Valid)
        } else {
            Ok(ValidateCallbackResult::Invalid(
                "Permission claims to have Creator permission but that's not the case.".into(),
            ))
        }
    } else {
        let contributor_permission: crate::ContributorPermission = permission_record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Permission action hash does not point to a ContributorPermission entry"
                    .to_string()
            )))?;

        // 1. Validate that permission is for the correct agent
        if contributor_permission.for_agent != agent {
            return Ok(ValidateCallbackResult::Invalid(
                "ContributorPermission is for the wrong agent.".into(),
            ));
        }

        // 2. Validate that permission is for the correct developer collective
        if contributor_permission.for_collective != developer_collective_hash {
            return Ok(ValidateCallbackResult::Invalid(
                "ContributorPermission is for the wrong DeveloperCollective.".into(),
            ));
        }

        // 3. Validate that the permission is not expired
        if let Some(expiry) = contributor_permission.expiry {
            if expiry < timestamp {
                return Ok(ValidateCallbackResult::Invalid(
                    "ContributorPermission has expired.".into(),
                ));
            }
        }

        Ok(ValidateCallbackResult::Valid)
    }
}
