import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleCurator(cell: CallableCell, partialCurator = {}) {
    return {
        ...{
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  icon: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  website: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  email: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  meta_data: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialCurator
    };
}

export async function createCurator(cell: CallableCell, curator = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "library",
      fn_name: "create_curator",
      payload: curator || await sampleCurator(cell),
    });
}



export async function sampleDeveloperCollective(cell: CallableCell, partialDeveloperCollective = {}) {
    return {
        ...{
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  website: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  contact: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  icon: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  meta_data: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialDeveloperCollective
    };
}

export async function createDeveloperCollective(cell: CallableCell, developerCollective = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "library",
      fn_name: "create_developer_collective",
      payload: developerCollective || await sampleDeveloperCollective(cell),
    });
}



export async function sampleContributorPermission(cell: CallableCell, partialContributorPermission = {}) {
    return {
        ...{
          for_collective: (await createDeveloperCollective(cell)).signed_action.hashed.hash,
          for_agent: cell.cell_id[1],
	  expiry: 1674053334548000,
        },
        ...partialContributorPermission
    };
}

export async function createContributorPermission(cell: CallableCell, contributorPermission = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "library",
      fn_name: "create_contributor_permission",
      payload: contributorPermission || await sampleContributorPermission(cell),
    });
}



export async function sampleTool(cell: CallableCell, partialTool = {}) {
    return {
        ...{
          developer_collective: (await createDeveloperCollective(cell)).signed_action.hashed.hash,
	  permission_hash: (await fakeActionHash()),
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  subtitle: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  icon: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  source: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  hashes: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  changelog: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  meta_data: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  deprecation: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialTool
    };
}

export async function createTool(cell: CallableCell, tool = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "library",
      fn_name: "create_tool",
      payload: tool || await sampleTool(cell),
    });
}

