
import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, Record, Link, AppAgentClient, EntryHash, ActionHash, AgentPubKey } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-circular-progress';
import { Task } from '@lit-labs/task';
import { LibrarySignal } from './types';

import { clientContext } from '../../contexts';
import './tool-detail';

@customElement('tools-for-developer-collective')
export class ToolsForDeveloperCollective extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal.toString() !== oldVal?.toString()
  })
  developerCollectiveHash!: ActionHash;

  @state()
  hashes: Array<ActionHash> = [];

  _fetchTools = new Task(this, ([developerCollectiveHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_tools_for_developer_collective',
      payload: developerCollectiveHash,
  }) as Promise<Array<Link>>, () => [this.developerCollectiveHash]);

  firstUpdated() {
    if (this.developerCollectiveHash === undefined) {
      throw new Error(`The developerCollectiveHash property is required for the tools-for-developer-collective element`);
    }

    this.client.on('signal', signal => {
      if (signal.zome_name !== 'library') return; 
      const payload = signal.payload as LibrarySignal;
      if (!(payload.type === 'EntryCreated' && payload.app_entry.type === 'Tool')) return;
      this._fetchTools.run();
    })
  }

  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No tools found for this developer collective.</span>`;
    
    return html`
      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash =>
          html`<tool-detail .toolHash=${hash} @tool-deleted=${() => { this._fetchTools.run(); this.hashes = []; } }></tool-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchTools.render({
      pending: () => html`
        <div style="display: flex; flex: 1; align-items: center; justify-content: center">
          <mwc-circular-progress indeterminate></mwc-circular-progress>
        </div>
      `,
      complete: (links) => this.renderList([...this.hashes, ...links.map(l => l.target)]),
      error: (e: any) => html`<span>Error fetching tools: ${e.data.data}.</span>`
    });
  }
}
