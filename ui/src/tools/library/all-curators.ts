import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { AppAgentClient, AgentPubKey, Link, EntryHash, ActionHash, Record, NewEntryAction } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import '@material/mwc-circular-progress';

import { clientContext } from '../../contexts';
import { LibrarySignal } from './types';

import './curator-detail';

@customElement('all-curators')
export class AllCurators extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;
  

  @state()
  signaledHashes: Array<ActionHash> = [];
  
  _fetchCurators = new Task(this, ([]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_all_curators',
      payload: null,
  }) as Promise<Array<Link>>, () => []);

  firstUpdated() {

    this.client.on('signal', signal => {
      if (signal.zome_name !== 'library') return; 
      const payload = signal.payload as LibrarySignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'Curator') return;
      this.signaledHashes = [payload.action.hashed.hash, ...this.signaledHashes];
    });
  }
  
  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No curators found.</span>`;
    
    return html`

      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash => 
          html`<curator-detail .curatorHash=${hash} style="margin-bottom: 16px;" @curator-deleted=${() => { this._fetchCurators.run(); this.signaledHashes = []; } }></curator-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchCurators.render({
      pending: () => html`
        <div style="display: flex; flex: 1; align-items: center; justify-content: center">
          <mwc-circular-progress indeterminate></mwc-circular-progress>
        </div>
      `,
      complete: (links) => this.renderList([...this.signaledHashes, ...links.map(l => l.target)]),
      error: (e: any) => html`<span>Error fetching the curators: ${e.data.data}.</span>`
    });
  }
}
