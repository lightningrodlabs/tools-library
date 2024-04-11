use hdk::prelude::*;
use library_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeveloperCollectiveForCuratorInput {
    pub base_curator_hash: ActionHash,
    pub target_developer_collective_hash: ActionHash,
}
#[hdk_extern]
pub fn add_developer_collective_for_curator(
    input: AddDeveloperCollectiveForCuratorInput,
) -> ExternResult<()> {
    create_link(
        input.base_curator_hash.clone(),
        input.target_developer_collective_hash.clone(),
        LinkTypes::CuratorToDeveloperCollectives,
        (),
    )?;
    create_link(
        input.target_developer_collective_hash,
        input.base_curator_hash,
        LinkTypes::DeveloperCollectiveToCurators,
        (),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_developer_collectives_for_curator(curator_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(curator_hash, LinkTypes::CuratorToDeveloperCollectives)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_deleted_developer_collectives_for_curator(
    curator_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        curator_hash,
        LinkTypes::CuratorToDeveloperCollectives,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_curators_for_developer_collective(
    developer_collective_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            developer_collective_hash,
            LinkTypes::DeveloperCollectiveToCurators,
        )?
        .build(),
    )
}

#[hdk_extern]
pub fn get_deleted_curators_for_developer_collective(
    developer_collective_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        developer_collective_hash,
        LinkTypes::DeveloperCollectiveToCurators,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDeveloperCollectiveForCuratorInput {
    pub base_curator_hash: ActionHash,
    pub target_developer_collective_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_developer_collective_for_curator(
    input: RemoveDeveloperCollectiveForCuratorInput,
) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            input.base_curator_hash.clone(),
            LinkTypes::CuratorToDeveloperCollectives,
        )?
        .build(),
    )?;

    for link in links {
        if link
            .target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?
            .eq(&input.target_developer_collective_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }

    let links = get_links(
        GetLinksInputBuilder::try_new(
            input.target_developer_collective_hash.clone(),
            LinkTypes::DeveloperCollectiveToCurators,
        )?
        .build(),
    )?;

    for link in links {
        if link
            .target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?
            .eq(&input.base_curator_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }

    Ok(())
}
