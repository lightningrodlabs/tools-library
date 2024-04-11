use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn create_contributor_permission(
    contributor_permission: ContributorPermission,
) -> ExternResult<Record> {
    let contributor_permission_hash = create_entry(
        &EntryTypes::ContributorPermission(contributor_permission.clone()),
    )?;
    create_link(
        contributor_permission.for_collective.clone(),
        contributor_permission_hash.clone(),
        LinkTypes::DeveloperCollectiveToContributorPermissions,
        (),
    )?;
    create_link(
        contributor_permission.for_agent.clone(),
        contributor_permission_hash.clone(),
        LinkTypes::ContributorToContributorPermissions,
        (),
    )?;
    let record = get(contributor_permission_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest("Could not find the newly created ContributorPermission"
                .to_string())
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_contributor_permission(
    contributor_permission_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(contributor_permission_hash, GetOptions::default())?
    else {
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
pub fn get_contributor_permissions_for_developer_collective(
    developer_collective_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
                developer_collective_hash,
                LinkTypes::DeveloperCollectiveToContributorPermissions,
            )?
            .build(),
    )
}
#[hdk_extern]
pub fn get_contributor_permissions_for_contributor(
    contributor: AgentPubKey,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
                contributor,
                LinkTypes::ContributorToContributorPermissions,
            )?
            .build(),
    )
}
