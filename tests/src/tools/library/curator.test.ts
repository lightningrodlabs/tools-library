import { assert, test } from "vitest";

import { runScenario, dhtSync } from "@holochain/tryorama";
import {
  Record,
  SignedActionHashed,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import { createCurator, sampleCurator } from "./common.js";

test("create Curator", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/tools-library.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, _bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Curator
    const record: Record = await createCurator(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and read Curator", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/tools-library.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleCurator(alice.cells[0]);

    // Alice creates a Curator
    const record: Record = await createCurator(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Curator
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_original_curator",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(
      sample,
      decode((createReadOutput.entry as any).Present.entry) as any
    );
  });
});

test("create and update Curator", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/tools-library.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Curator
    const record: Record = await createCurator(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the Curator
    let contentUpdate: any = await sampleCurator(alice.cells[0]);
    let updateInput = {
      original_curator_hash: originalActionHash,
      previous_curator_hash: originalActionHash,
      updated_curator: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "update_curator",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated Curator
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_latest_curator",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput0.entry as any).Present.entry) as any
    );

    // Alice updates the Curator again
    contentUpdate = await sampleCurator(alice.cells[0]);
    updateInput = {
      original_curator_hash: originalActionHash,
      previous_curator_hash: updatedRecord.signed_action.hashed.hash,
      updated_curator: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "update_curator",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated Curator
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_latest_curator",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput1.entry as any).Present.entry) as any
    );

    // Bob gets all the revisions for Curator
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_revisions_for_curator",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(
      contentUpdate,
      decode((revisions[2].entry as any).Present.entry) as any
    );
  });
});

test("create and delete Curator", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/tools-library.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleCurator(alice.cells[0]);

    // Alice creates a Curator
    const record: Record = await createCurator(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice deletes the Curator
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "delete_curator",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the Curator
    const oldestDeleteForCurator = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_oldest_delete_for_curator",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForCurator);

    // Bob gets the deletions for Curator
    const deletesForCurator: SignedActionHashed[] = await bob.cells[0].callZome(
      {
        zome_name: "library",
        fn_name: "get_all_deletes_for_curator",
        payload: record.signed_action.hashed.hash,
      }
    );
    assert.equal(deletesForCurator.length, 1);
  });
});
