import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  Link,
  CreateLink,
  DeleteLink,
  SignedActionHashed,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import { createCurator } from "./common.js";
import { createTool } from "./common.js";

test("empty test", () => {});

// test('link a Curator to a Tool', async () => {
//   await runScenario(async scenario => {
//     // Construct proper paths for your app.
//     // This assumes app bundle created by the `hc app pack` command.
//     const testAppPath = process.cwd() + '/../workdir/tools-library.happ';

//     // Set up the app to be installed
//     const appSource = { appBundleSource: { path: testAppPath } };

//     // Add 2 players with the test app to the Scenario. The returned players
//     // can be destructured.
//     const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

//     // Shortcut peer discovery through gossip and register all agents in every
//     // conductor of the scenario.
//     await scenario.shareAllAgents();

//     const baseRecord = await createCurator(alice.cells[0]);
//     const baseAddress = baseRecord.signed_action.hashed.hash;
//     const targetRecord = await createTool(alice.cells[0]);
//     const targetAddress = targetRecord.signed_action.hashed.hash;

//     // Bob gets the links, should be empty
//     let linksOutput: Link[] = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_tools_for_curator",
//       payload: baseAddress
//     });
//     assert.equal(linksOutput.length, 0);

//     // Alice creates a link from Curator to Tool
//     await alice.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "add_tool_for_curator",
//       payload: {
//         base_curator_hash: baseAddress,
//         target_tool_hash: targetAddress
//       }
//     });

//     await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

//     // Bob gets the links again
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_tools_for_curator",
//       payload: baseAddress
//     });
//     assert.equal(linksOutput.length, 1);
//     assert.deepEqual(targetAddress, linksOutput[0].target);

//     // Bob gets the links in the inverse direction
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_curators_for_tool",
//       payload: targetAddress
//     });
//     assert.equal(linksOutput.length, 1);
//     assert.deepEqual(baseAddress, linksOutput[0].target);

//     await alice.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "remove_tool_for_curator",
//       payload: {
//         base_curator_hash: baseAddress,
//         target_tool_hash: targetAddress
//       }
//     });

//     await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

//     // Bob gets the links again
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_tools_for_curator",
//       payload: baseAddress
//     });
//     assert.equal(linksOutput.length, 0);

//     // Bob gets the deleted links
//     let deletedLinksOutput: Array<[SignedActionHashed<CreateLink>, SignedActionHashed<DeleteLink>[]]> = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_deleted_tools_for_curator",
//       payload: baseAddress
//     });
//     assert.equal(deletedLinksOutput.length, 1);

//     // Bob gets the links in the inverse direction
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_curators_for_tool",
//       payload: targetAddress
//     });
//     assert.equal(linksOutput.length, 0);

//     // Bob gets the deleted links in the inverse direction
//     deletedLinksOutput = await bob.cells[0].callZome({
//       zome_name: "library",
//       fn_name: "get_deleted_curators_for_tool",
//       payload: targetAddress
//     });
//     assert.equal(deletedLinksOutput.length, 1);

//   });
// });
