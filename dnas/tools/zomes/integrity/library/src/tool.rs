use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Tool {
    pub developer_collective: ActionHash,
    pub permission_hash: ActionHash,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub icon: String,
    pub version: String,
    pub source: String,
    pub hashes: String,
    pub changelog: Option<String>,
    pub meta_data: Option<String>,
    pub deprecation: Option<String>,
}
pub fn validate_create_tool(
    action: EntryCreationAction,
    tool: Tool,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(tool.developer_collective.clone())?;
    let _developer_collective: crate::DeveloperCollective = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))
            ),
        )?;
    validate_contributor_permission(
        tool.permission_hash,
        action.author().clone(),
        tool.developer_collective,
        action.timestamp().clone(),
    )
}
pub fn validate_update_tool(
    action: Update,
    tool: Tool,
    _original_action: EntryCreationAction,
    original_tool: Tool,
) -> ExternResult<ValidateCallbackResult> {
    if tool.developer_collective != original_tool.developer_collective {
        return Ok(
            ValidateCallbackResult::Invalid(
                "The developer_collective field may not be updated.".into(),
            ),
        );
    }
    validate_contributor_permission(
        tool.permission_hash,
        action.author,
        original_tool.developer_collective,
        action.timestamp,
    )
}
pub fn validate_delete_tool(
    action: Delete,
    original_action: EntryCreationAction,
    original_tool: Tool,
) -> ExternResult<ValidateCallbackResult> {
    if &action.author == original_action.author() {
        Ok(ValidateCallbackResult::Valid)
    } else {
        let collective_record = must_get_valid_record(
            original_tool.developer_collective,
        )?;
        if collective_record.action().author() == &action.author {
            return Ok(ValidateCallbackResult::Valid);
        }
        Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of the DeveloperCollective entry or the creator of the link is allowed to delete a link from a DeveloperCollective entry to a Tool entry"
                    .into(),
            ),
        )
    }
}
pub fn validate_create_link_developer_collective_to_tools(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let permission_action_hash = match ActionHash::from_raw_39(tag.0) {
        Ok(ah) => ah,
        Err(e) => {
            return Ok(
                ValidateCallbackResult::Invalid(
                    format!(
                        "Link tag does not contain a valid action hash. Conversion failed with error: {}",
                        e
                    ),
                ),
            );
        }
    };
    let collective_action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
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
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    let tool_action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let tool_record = must_get_valid_record(tool_action_hash)?;
    let tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    if tool.developer_collective != collective_action_hash {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Link from a DeveloperCollective entry to a Tool can only be created if the Tool references that same DeveloperCollective entry in the developer_collective field."
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_developer_collective_to_tools(
    action: DeleteLink,
    original_action: CreateLink,
    base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author == original_action.author {
        Ok(ValidateCallbackResult::Valid)
    } else {
        let collective_action_hash = base
            .into_action_hash()
            .ok_or(
                wasm_error!(
                    WasmErrorInner::Guest("Link base is not an action hash. This link should never have passed validation in the first place!"
                    .to_string())
                ),
            )?;
        let collective_record = must_get_valid_record(collective_action_hash)?;
        if collective_record.action().author() == &action.author {
            return Ok(ValidateCallbackResult::Valid);
        }
        Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of the DeveloperCollective entry or the creator of the link is allowed to delete a link from a DeveloperCollective entry to a Tool entry"
                    .into(),
            ),
        )
    }
}
pub fn validate_create_link_tool_updates(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let permission_action_hash = match ActionHash::from_raw_39(tag.0) {
        Ok(ah) => ah,
        Err(e) => {
            return Ok(
                ValidateCallbackResult::Invalid(
                    format!(
                        "Link tag does not contain a valid action hash. Conversion failed with error: {}",
                        e
                    ),
                ),
            );
        }
    };
    let tool_action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let tool_record = must_get_valid_record(tool_action_hash)?;
    let tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    validate_contributor_permission(
        permission_action_hash,
        action.author,
        tool.developer_collective,
        action.timestamp,
    )?;
    let tool_update_action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let tool_update_record = must_get_valid_record(tool_update_action_hash)?;
    let _tool: crate::Tool = tool_update_record
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
pub fn validate_delete_link_tool_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ToolUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_contributor_permission(
    permission_hash: ActionHash,
    agent: AgentPubKey,
    developer_collective_hash: ActionHash,
    timestamp: Timestamp,
) -> ExternResult<ValidateCallbackResult> {
    let permission_record = must_get_valid_record(permission_hash.clone())?;
    if permission_hash == developer_collective_hash {
        let developer_collective_record = must_get_valid_record(
            developer_collective_hash,
        )?;
        if developer_collective_record.action().author() == &agent {
            Ok(ValidateCallbackResult::Valid)
        } else {
            Ok(
                ValidateCallbackResult::Invalid(
                    "Permission claims to have Creator permission but that's not the case."
                        .into(),
                ),
            )
        }
    } else {
        let contributor_permission: crate::ContributorPermission = permission_record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(
                wasm_error!(
                    WasmErrorInner::Guest("Permission action hash does not point to a ContributorPermission entry"
                    .to_string())
                ),
            )?;
        if contributor_permission.for_agent != agent {
            return Ok(
                ValidateCallbackResult::Invalid(
                    "ContributorPermission is for the wrong agent.".into(),
                ),
            );
        }
        if contributor_permission.for_collective != developer_collective_hash {
            return Ok(
                ValidateCallbackResult::Invalid(
                    "ContributorPermission is for the wrong DeveloperCollective.".into(),
                ),
            );
        }
        if let Some(expiry) = contributor_permission.expiry {
            if expiry < timestamp {
                return Ok(
                    ValidateCallbackResult::Invalid(
                        "ContributorPermission has expired.".into(),
                    ),
                );
            }
        }
        Ok(ValidateCallbackResult::Valid)
    }
}
