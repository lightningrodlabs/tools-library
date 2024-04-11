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

import './edit-curator';

import { clientContext } from '../../contexts';
import { Curator } from './types';

@customElement('curator-detail')
export class CuratorDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  curatorHash!: ActionHash;

  _fetchRecord = new Task(this, ([curatorHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_latest_curator',
      payload: curatorHash,
  }) as Promise<Record | undefined>, () => [this.curatorHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.curatorHash === undefined) {
      throw new Error(`The curatorHash property is required for the curator-detail element`);
    }
  }

  async deleteCurator() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'delete_curator',
        payload: this.curatorHash,
      });
      this.dispatchEvent(new CustomEvent('curator-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          curatorHash: this.curatorHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the curator: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const curator = decode((record.entry as any).Present.entry) as Curator;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteCurator()}></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderCurator(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested curator was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-curator
    	  .originalCuratorHash=${this.curatorHash}
    	  .currentRecord=${maybeRecord}
    	  @curator-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-curator>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderCurator(maybeRecord),
      error: (e: any) => html`<span>Error fetching the curator: ${e.data.data}</span>`
    });
  }
}
