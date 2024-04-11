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

import './edit-tool';

import { clientContext } from '../../contexts';
import { Tool } from './types';

@customElement('tool-detail')
export class ToolDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  toolHash!: ActionHash;

  _fetchRecord = new Task(this, ([toolHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_latest_tool',
      payload: toolHash,
  }) as Promise<Record | undefined>, () => [this.toolHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.toolHash === undefined) {
      throw new Error(`The toolHash property is required for the tool-detail element`);
    }
  }

  async deleteTool() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'delete_tool',
        payload: this.toolHash,
      });
      this.dispatchEvent(new CustomEvent('tool-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          toolHash: this.toolHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the tool: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const tool = decode((record.entry as any).Present.entry) as Tool;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteTool()}></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderTool(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested tool was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-tool
    	  .originalToolHash=${this.toolHash}
    	  .currentRecord=${maybeRecord}
    	  @tool-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-tool>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderTool(maybeRecord),
      error: (e: any) => html`<span>Error fetching the tool: ${e.data.data}</span>`
    });
  }
}
