import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
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
  fakeEntryHash
} from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createTool, sampleTool } from './common.js';

test('create Tool', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/tools-library.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Tool
    const record: Record = await createTool(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read Tool', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/tools-library.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleTool(alice.cells[0]);

    // Alice creates a Tool
    const record: Record = await createTool(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Tool
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_original_tool",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the DeveloperCollectives for the new Tool
    let linksToDeveloperCollectives: Link[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_tools_for_developer_collective",
      payload: sample.developer_collective
    });
    assert.equal(linksToDeveloperCollectives.length, 1);
    assert.deepEqual(linksToDeveloperCollectives[0].target, record.signed_action.hashed.hash);
  });
});

test('create and update Tool', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/tools-library.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Tool
    const record: Record = await createTool(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the Tool
    let contentUpdate: any = await sampleTool(alice.cells[0]);
    let updateInput = {
      original_tool_hash: originalActionHash,
      previous_tool_hash: originalActionHash,
      updated_tool: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "update_tool",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated Tool
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_latest_tool",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the Tool again
    contentUpdate = await sampleTool(alice.cells[0]);
    updateInput = { 
      original_tool_hash: originalActionHash,
      previous_tool_hash: updatedRecord.signed_action.hashed.hash,
      updated_tool: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "update_tool",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated Tool
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_latest_tool",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);

    // Bob gets all the revisions for Tool
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_revisions_for_tool",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(contentUpdate, decode((revisions[2].entry as any).Present.entry) as any);
  });
});

test('create and delete Tool', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/tools-library.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleTool(alice.cells[0]);

    // Alice creates a Tool
    const record: Record = await createTool(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the DeveloperCollectives for the new Tool
    let linksToDeveloperCollectives: Link[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_tools_for_developer_collective",
      payload: sample.developer_collective
    });
    assert.equal(linksToDeveloperCollectives.length, 1);
    assert.deepEqual(linksToDeveloperCollectives[0].target, record.signed_action.hashed.hash);

    // Alice deletes the Tool
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "delete_tool",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the Tool
    const oldestDeleteForTool = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_oldest_delete_for_tool",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForTool);
        
    // Bob gets the deletions for Tool
    const deletesForTool: SignedActionHashed[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_deletes_for_tool",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(deletesForTool.length, 1);

    // Bob gets the DeveloperCollectives for the Tool again
    linksToDeveloperCollectives = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_tools_for_developer_collective",
      payload: sample.developer_collective
    });
    assert.equal(linksToDeveloperCollectives.length, 0);

    // Bob gets the deleted DeveloperCollectives for the Tool 
    const deletedLinksToDeveloperCollectives: Array<[SignedActionHashed<CreateLink>, SignedActionHashed<DeleteLink>[]]> = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_deleted_tools_for_developer_collective",
      payload: sample.developer_collective
    });
    assert.equal(deletedLinksToDeveloperCollectives.length, 1);

  });
});
