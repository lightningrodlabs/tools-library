import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { Tool } from './types';

@customElement('edit-tool')
export class EditTool extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalToolHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentTool() {
    return decode((this.currentRecord.entry as any).Present.entry) as Tool;
  }
 

  isToolValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-tool element`);
    }

    if (this.originalToolHash === undefined) {
      throw new Error(`The originalToolHash property is required for the edit-tool element`);
    }
    
  }

  async updateTool() {
    const tool: Tool = { 
      developer_collective: this.currentTool.developer_collective,
      permission_hash: this.currentTool.permission_hash,
      title: this.currentTool.title,
      subtitle: this.currentTool.subtitle,
      description: this.currentTool.description,
      icon: this.currentTool.icon,
      source: this.currentTool.source,
      hashes: this.currentTool.hashes,
      changelog: this.currentTool.changelog,
      meta_data: this.currentTool.meta_data,
      deprecation: this.currentTool.deprecation,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'update_tool',
        payload: {
          original_tool_hash: this.originalToolHash,
          previous_tool_hash: this.currentRecord.signed_action.hashed.hash,
          updated_tool: tool
        },
      });
  
      this.dispatchEvent(new CustomEvent('tool-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalToolHash: this.originalToolHash,
          previousToolHash: this.currentRecord.signed_action.hashed.hash,
          updatedToolHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the tool: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Tool</span>


        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            @click=${() => this.dispatchEvent(new CustomEvent('edit-canceled', {
              bubbles: true,
              composed: true
            }))}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>
          <mwc-button 
            raised
            label="Save"
            .disabled=${!this.isToolValid()}
            @click=${() => this.updateTool()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
