use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn create_tool(tool: Tool) -> ExternResult<Record> {
    let tool_hash = create_entry(&EntryTypes::Tool(tool.clone()))?;
    create_link(
        tool.developer_collective.clone(),
        tool_hash.clone(),
        LinkTypes::DeveloperCollectiveToTools,
        // TODO add a tag containing the permission action hash here
        (),
    )?;
    let record = get(tool_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Tool".to_string())
    ))?;
    Ok(record)
}
#[hdk_extern]
pub fn get_latest_tool(original_tool_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(original_tool_hash.clone(), LinkTypes::ToolUpdates)?.build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_tool_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_tool_hash.clone(),
    };
    get(latest_tool_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_original_tool(original_tool_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_tool_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}
#[hdk_extern]
pub fn get_all_revisions_for_tool(original_tool_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_tool(original_tool_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(original_tool_hash.clone(), LinkTypes::ToolUpdates)?.build(),
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "No action hash associated with link".to_string()
                    )))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().flatten().collect();
    records.insert(0, original_record);
    Ok(records)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateToolInput {
    pub original_tool_hash: ActionHash,
    pub previous_tool_hash: ActionHash,
    pub updated_tool: Tool,
}
#[hdk_extern]
pub fn update_tool(input: UpdateToolInput) -> ExternResult<Record> {
    let updated_tool_hash = update_entry(input.previous_tool_hash.clone(), &input.updated_tool)?;
    create_link(
        input.original_tool_hash.clone(),
        updated_tool_hash.clone(),
        LinkTypes::ToolUpdates,
        (),
    )?;
    let record = get(updated_tool_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated Tool".to_string())
    ))?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_tool(original_tool_hash: ActionHash) -> ExternResult<ActionHash> {
    let details =
        get_details(original_tool_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("{pascal_entry_def_name} not found"))
        ))?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed get details response"
        )))),
    }?;
    let entry = record
        .entry()
        .as_option()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Tool record has no entry".to_string()
        )))?;
    let tool = Tool::try_from(entry)?;
    let links = get_links(
        GetLinksInputBuilder::try_new(
            tool.developer_collective.clone(),
            LinkTypes::DeveloperCollectiveToTools,
        )?
        .build(),
    )?;
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if action_hash.eq(&original_tool_hash) {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_tool_hash)
}
#[hdk_extern]
pub fn get_all_deletes_for_tool(
    original_tool_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_tool_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}
#[hdk_extern]
pub fn get_oldest_delete_for_tool(
    original_tool_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_tool(original_tool_hash)? else {
        return Ok(None);
    };
    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });
    Ok(deletes.first().cloned())
}
#[hdk_extern]
pub fn get_tools_for_developer_collective(
    developer_collective_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            developer_collective_hash,
            LinkTypes::DeveloperCollectiveToTools,
        )?
        .build(),
    )
}
#[hdk_extern]
pub fn get_deleted_tools_for_developer_collective(
    developer_collective_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        developer_collective_hash,
        LinkTypes::DeveloperCollectiveToTools,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
