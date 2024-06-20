use hdi::prelude::*;

pub fn validate_create_link_all_agents(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check that base address is pointing away from the all_agents anchor
    let base_address_entry_hash = EntryHash::try_from(base_address).map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Base address is not an entry hash".into()
        ))
    })?;
    let path = Path::from("all_agents");
    if path.path_entry_hash()? != base_address_entry_hash {
        return Ok(ValidateCallbackResult::Invalid(
            "AllAgents link is not pointing away from the all_agents anchor.".into(),
        ));
    }

    // Check the entry type for the given action hash
    let agent_pub_key =
        target_address
            .into_agent_pub_key()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Link target is not an agent public key".to_string()
            )))?;

    if action.author != agent_pub_key {
        return Ok(ValidateCallbackResult::Invalid(
            "Links from the all_agents anchor can only point to the agent public key of the agent that creates the link.".into(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}

/// Rules
/// 1. Cannot be deleted
pub fn validate_delete_link_all_agents(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Links from the all_agents anchor cannot be deleted.",
    )))
}
