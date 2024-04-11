import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { EntryHash, Record, ActionHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import './edit-developer-collective';

import { clientContext } from '../../contexts';
import { DeveloperCollective } from './types';

@customElement('developer-collective-detail')
export class DeveloperCollectiveDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  developerCollectiveHash!: ActionHash;

  _fetchRecord = new Task(this, ([developerCollectiveHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_latest_developer_collective',
      payload: developerCollectiveHash,
  }) as Promise<Record | undefined>, () => [this.developerCollectiveHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.developerCollectiveHash === undefined) {
      throw new Error(`The developerCollectiveHash property is required for the developer-collective-detail element`);
    }
  }

  async deleteDeveloperCollective() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'delete_developer_collective',
        payload: this.developerCollectiveHash,
      });
      this.dispatchEvent(new CustomEvent('developer-collective-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          developerCollectiveHash: this.developerCollectiveHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the developer collective: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const developerCollective = decode((record.entry as any).Present.entry) as DeveloperCollective;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteDeveloperCollective()}></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderDeveloperCollective(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested developer collective was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-developer-collective
    	  .originalDeveloperCollectiveHash=${this.developerCollectiveHash}
    	  .currentRecord=${maybeRecord}
    	  @developer-collective-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-developer-collective>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderDeveloperCollective(maybeRecord),
      error: (e: any) => html`<span>Error fetching the developer collective: ${e.data.data}</span>`
    });
  }
}
