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

