use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Curator {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub website: Option<String>,
    pub email: Option<String>,
    pub meta_data: Option<String>,
}
pub fn validate_create_curator(
    _action: EntryCreationAction,
    _curator: Curator,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_curator(
    action: Update,
    _curator: Curator,
    original_action: EntryCreationAction,
    _original_curator: Curator,
) -> ExternResult<ValidateCallbackResult> {
    if &action.author != original_action.author() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Curator entries can only be updated by the agent that created the entry."
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_curator(
    action: Delete,
    original_action: EntryCreationAction,
    _original_curator: Curator,
) -> ExternResult<ValidateCallbackResult> {
    if &action.author != original_action.author() {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Curator entries can only be deleted by the agent that created the entry."
                    .into(),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_curator_updates(
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
                "Links to curator entry updates can only be created by the agent that created the curator entry."
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
                WasmErrorInner::Guest("Link base of a link to a Curator update must be Curator entry"
                .to_string())
            ),
        )?;
    let curator_update_action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let curator_update_record = must_get_valid_record(curator_update_action_hash)?;
    let _curator_update: crate::Curator = curator_update_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Link target of a link to a Curator update must be a Curator entry"
                .to_string())
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_curator_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("CuratorUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_curators(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("No action hash associated with link".to_string())
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _curator: crate::Curator = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Linked action must reference an entry"
                .to_string())
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_curators(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
