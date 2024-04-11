use hdk::prelude::*;
use library_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddToolForCuratorInput {
    pub base_curator_hash: ActionHash,
    pub target_tool_hash: ActionHash,
}
#[hdk_extern]
pub fn add_tool_for_curator(input: AddToolForCuratorInput) -> ExternResult<()> {
    create_link(input.base_curator_hash.clone(), input.target_tool_hash.clone(), LinkTypes::CuratorToTools, ())?;
    create_link(input.target_tool_hash, input.base_curator_hash, LinkTypes::ToolToCurators, ())?;

    Ok(())    
}

#[hdk_extern]
pub fn get_tools_for_curator(curator_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(curator_hash, LinkTypes::CuratorToTools)?.build(),
    )
}

#[hdk_extern]
pub fn get_deleted_tools_for_curator(
    curator_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        curator_hash,
        LinkTypes::CuratorToTools,
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
pub fn get_curators_for_tool(tool_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(tool_hash, LinkTypes::ToolToCurators)?.build(),
    )
}

#[hdk_extern]
pub fn get_deleted_curators_for_tool(
    tool_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        tool_hash,
        LinkTypes::ToolToCurators,
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
pub struct RemoveToolForCuratorInput {
    pub base_curator_hash: ActionHash,
    pub target_tool_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_tool_for_curator(input: RemoveToolForCuratorInput ) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.base_curator_hash.clone(), LinkTypes::CuratorToTools)?.build(),
    )?;
    
    for link in links {
        if link.target.clone().into_action_hash().ok_or(wasm_error!(WasmErrorInner::Guest("No action hash associated with link".to_string())))?.eq(&input.target_tool_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    
    let links = get_links(
        GetLinksInputBuilder::try_new(input.target_tool_hash.clone(), LinkTypes::ToolToCurators)?.build(),
    )?;

    for link in links {
        if link.target.clone().into_action_hash().ok_or(wasm_error!(WasmErrorInner::Guest("No action hash associated with link".to_string())))?.eq(&input.base_curator_hash) {
            delete_link(link.create_link_hash)?;
        }
    }

    Ok(())        
}
