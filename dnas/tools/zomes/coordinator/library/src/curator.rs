use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn create_curator(curator: Curator) -> ExternResult<Record> {
    let curator_hash = create_entry(&EntryTypes::Curator(curator.clone()))?;
    let record = get(curator_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Could not find the newly created Curator"
                .to_string())
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_latest_curator(
    original_curator_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
                original_curator_hash.clone(),
                LinkTypes::CuratorUpdates,
            )?
            .build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_curator_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("No action hash associated with link"
                        .to_string())
                    ),
                )?
        }
        None => original_curator_hash.clone(),
    };
    get(latest_curator_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_original_curator(
    original_curator_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_curator_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest("Malformed get details response".to_string())
                ),
            )
        }
    }
}
#[hdk_extern]
pub fn get_all_revisions_for_curator(
    original_curator_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_curator(original_curator_hash.clone())?
    else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(
                original_curator_hash.clone(),
                LinkTypes::CuratorUpdates,
            )?
            .build(),
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| Ok(
            GetInput::new(
                link
                    .target
                    .into_action_hash()
                    .ok_or(
                        wasm_error!(
                            WasmErrorInner::Guest("No action hash associated with link"
                            .to_string())
                        ),
                    )?
                    .into(),
                GetOptions::default(),
            ),
        ))
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().flatten().collect();
    records.insert(0, original_record);
    Ok(records)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCuratorInput {
    pub original_curator_hash: ActionHash,
    pub previous_curator_hash: ActionHash,
    pub updated_curator: Curator,
}
#[hdk_extern]
pub fn update_curator(input: UpdateCuratorInput) -> ExternResult<Record> {
    let updated_curator_hash = update_entry(
        input.previous_curator_hash.clone(),
        &input.updated_curator,
    )?;
    create_link(
        input.original_curator_hash.clone(),
        updated_curator_hash.clone(),
        LinkTypes::CuratorUpdates,
        (),
    )?;
    let record = get(updated_curator_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Could not find the newly updated Curator"
                .to_string())
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_curator(original_curator_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_curator_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("{pascal_entry_def_name} not found".to_string())
            ),
        )?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest("Malformed get details response".to_string())
                ),
            )
        }
    }?;
    delete_entry(original_curator_hash)
}
#[hdk_extern]
pub fn get_all_deletes_for_curator(
    original_curator_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_curator_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}
#[hdk_extern]
pub fn get_oldest_delete_for_curator(
    original_curator_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_curator(original_curator_hash)? else {
        return Ok(None);
    };
    deletes
        .sort_by(|delete_a, delete_b| {
            delete_a.action().timestamp().cmp(&delete_b.action().timestamp())
        });
    Ok(deletes.first().cloned())
}
