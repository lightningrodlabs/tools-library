import { LitElement, html } from 'lit';
import { state, property, customElement } from 'lit/decorators.js';
import { AgentPubKey, Link, EntryHash, ActionHash, Record, AppAgentClient, NewEntryAction } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import '@material/mwc-circular-progress';

import { clientContext } from '../../contexts';
import './developer-collective-detail';
import { LibrarySignal } from './types';

@customElement('developer-collectives-for-curator')
export class DeveloperCollectivesForCurator extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  curatorHash!: ActionHash; 

  @state()
  signaledHashes: Array<ActionHash> = [];

  _fetchDeveloperCollectives = new Task(this, ([curatorHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_developer_collectives_for_curator',
      payload: curatorHash,
  }) as Promise<Array<Link>>, () => [this.curatorHash]);

  firstUpdated() {
    if (this.curatorHash === undefined) {
      throw new Error(`The curatorHash property is required for the developer-collectives-for-curator element`);
    }

    this.client.on('signal', signal => {
      if (signal.zome_name !== 'library') return;
      const payload = signal.payload as LibrarySignal;
      if (payload.type !== 'LinkCreated') return;
      if (payload.link_type !== 'CuratorToDeveloperCollectives') return;

      this.signaledHashes = [payload.action.hashed.content.target_address, ...this.signaledHashes];
    });
  }

  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No developer collectives found for this curator</span>`;
    
    return html`
      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash => 
          html`<developer-collective-detail .developerCollectiveHash=${hash} style="margin-bottom: 16px;"></developer-collective-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchDeveloperCollectives.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (links) => this.renderList([...this.signaledHashes, ...links.map(l => l.target)]),
      error: (e: any) => html`<span>Error fetching the developer collectives: ${e.data.data}.</span>`
    });
  }
}
