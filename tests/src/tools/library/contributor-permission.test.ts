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

import { createContributorPermission, sampleContributorPermission } from './common.js';

test('create ContributorPermission', async () => {
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

    // Alice creates a ContributorPermission
    const record: Record = await createContributorPermission(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read ContributorPermission', async () => {
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

    const sample = await sampleContributorPermission(alice.cells[0]);

    // Alice creates a ContributorPermission
    const record: Record = await createContributorPermission(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created ContributorPermission
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_contributor_permission",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the DeveloperCollectives for the new ContributorPermission
    let linksToDeveloperCollectives: Link[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_contributor_permissions_for_developer_collective",
      payload: sample.for_collective
    });
    assert.equal(linksToDeveloperCollectives.length, 1);
    assert.deepEqual(linksToDeveloperCollectives[0].target, record.signed_action.hashed.hash);
    // Bob gets the Contributors for the new ContributorPermission
    let linksToContributors: Link[] = await bob.cells[0].callZome({
      zome_name: "library",
      fn_name: "get_contributor_permissions_for_contributor",
      payload: sample.for_agent
    });
    assert.equal(linksToContributors.length, 1);
    assert.deepEqual(linksToContributors[0].target, record.signed_action.hashed.hash);
  });
});


