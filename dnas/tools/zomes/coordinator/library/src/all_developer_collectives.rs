use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn get_all_developer_collective_links(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_developer_collectives");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDeveloperCollectives)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_all_original_developer_collectives(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_developer_collectives");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDeveloperCollectives)?
            .build(),
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
    Ok(records.into_iter().flatten().collect())
}
