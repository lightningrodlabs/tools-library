use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn get_all_curators(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_curators");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllCurators)?
            .build(),
    )
}
