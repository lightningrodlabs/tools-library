use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn get_all_agents(_: ()) -> ExternResult<Vec<AgentPubKey>> {
    let path = Path::from("all_agents");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?.build(),
    )?;
    Ok(links
        .into_iter()
        .map(|l| l.target.into_agent_pub_key())
        .filter_map(|l| l)
        .collect())
}
