import { assert, test } from "vitest";

import { runScenario, dhtSync } from '@holochain/tryorama';
import {
  Record,
  Link,
} from '@holochain/client';

import { createDeveloperCollective } from './common.js';

test('create a DeveloperCollective and get all developer collectives', async () => {
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

    // Bob gets all developer collectives
    let collectionOutput: Link[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_developer_collective_links",
      payload: null
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a DeveloperCollective
    const createRecord: Record = await createDeveloperCollective(alice.cells[0]);
    assert.ok(createRecord);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets all developer collectives again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_developer_collective_links",
      payload: null
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createRecord.signed_action.hashed.hash, collectionOutput[0].target);

    // Alice deletes the DeveloperCollective
    await alice.cells[0].callZome({
      zome_name: "library",
      fn_name: "delete_developer_collective",
      payload: createRecord.signed_action.hashed.hash
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets all developer collectives again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_all_developer_collective_links",
      payload: null
    });
    assert.equal(collectionOutput.length, 0);
  });
});

