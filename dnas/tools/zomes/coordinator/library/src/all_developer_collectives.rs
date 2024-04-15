use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn get_all_developer_collectives(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_developer_collectives");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllDeveloperCollectives)?
            .build(),
    )
}
