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

import { createDeveloperCollective, sampleDeveloperCollective } from './common.js';

test('create DeveloperCollective', async () => {
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

    // Alice creates a DeveloperCollective
    const record: Record = await createDeveloperCollective(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read DeveloperCollective', async () => {
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

    const sample = await sampleDeveloperCollective(alice.cells[0]);

    // Alice creates a DeveloperCollective
    const record: Record = await createDeveloperCollective(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created DeveloperCollective
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_original_developer_collective",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

  });
});

test('create and update DeveloperCollective', async () => {
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

    // Alice creates a DeveloperCollective
    const record: Record = await createDeveloperCollective(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the DeveloperCollective
    let contentUpdate: any = await sampleDeveloperCollective(alice.cells[0]);
    let updateInput = {
      original_developer_collective_hash: originalActionHash,
      previous_developer_collective_hash: originalActionHash,
      updated_developer_collective: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "update_developer_collective",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated DeveloperCollective
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_latest_developer_collective",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the DeveloperCollective again
    contentUpdate = await sampleDeveloperCollective(alice.cells[0]);
    updateInput = { 
      original_developer_collective_hash: originalActionHash,
      previous_developer_collective_hash: updatedRecord.signed_action.hashed.hash,
      updated_developer_collective: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "update_developer_collective",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated DeveloperCollective
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_latest_developer_collective",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);

    // Bob gets all the revisions for DeveloperCollective
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_revisions_for_developer_collective",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(contentUpdate, decode((revisions[2].entry as any).Present.entry) as any);
  });
});

test('create and delete DeveloperCollective', async () => {
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

    const sample = await sampleDeveloperCollective(alice.cells[0]);

    // Alice creates a DeveloperCollective
    const record: Record = await createDeveloperCollective(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);


    // Alice deletes the DeveloperCollective
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "delete_developer_collective",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the DeveloperCollective
    const oldestDeleteForDeveloperCollective = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_oldest_delete_for_developer_collective",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForDeveloperCollective);
        
    // Bob gets the deletions for DeveloperCollective
    const deletesForDeveloper Collective: SignedActionHashed[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_deletes_for_developer_collective",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(deletesForDeveloper Collective.length, 1);


  });
});
