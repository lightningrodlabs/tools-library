use hdi::prelude::*;
pub fn validate_create_link_curator_to_tools(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let curator_action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let curator_record = must_get_valid_record(curator_action_hash)?;
    if curator_record.action().author() != &action.author {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of a Curator entry can create links to Tool entries"
                    .into(),
            ),
        );
    }
    let _curator: crate::Curator = curator_record
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
    let _tool: crate::Tool = tool_record
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
pub fn validate_delete_link_curator_to_tools(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author != original_action.author {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of a link from a Curator to a Tool entry can delete that link."
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_tool_to_curators(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let tool_action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let tool_record = must_get_valid_record(tool_action_hash)?;
    let _tool: crate::Tool = tool_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    let curator_action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let curator_record = must_get_valid_record(curator_action_hash)?;
    if curator_record.action().author() != &action.author {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of a Curator entry can create links from Tool entries to that Curator entry"
                    .into(),
            ),
        );
    }
    let _curator: crate::Curator = curator_record
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
pub fn validate_delete_link_tool_to_curators(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author != original_action.author {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Only the creator of a link from a Tool entry to a Curator entry can delete that link."
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
