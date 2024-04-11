import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { Curator } from './types';

@customElement('edit-curator')
export class EditCurator extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalCuratorHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentCurator() {
    return decode((this.currentRecord.entry as any).Present.entry) as Curator;
  }
 

  isCuratorValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-curator element`);
    }

    if (this.originalCuratorHash === undefined) {
      throw new Error(`The originalCuratorHash property is required for the edit-curator element`);
    }
    
  }

  async updateCurator() {
    const curator: Curator = { 
      name: this.currentCurator.name,
      description: this.currentCurator.description,
      icon: this.currentCurator.icon,
      website: this.currentCurator.website,
      email: this.currentCurator.email,
      meta_data: this.currentCurator.meta_data,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'update_curator',
        payload: {
          original_curator_hash: this.originalCuratorHash,
          previous_curator_hash: this.currentRecord.signed_action.hashed.hash,
          updated_curator: curator
        },
      });
  
      this.dispatchEvent(new CustomEvent('curator-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalCuratorHash: this.originalCuratorHash,
          previousCuratorHash: this.currentRecord.signed_action.hashed.hash,
          updatedCuratorHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the curator: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Curator</span>


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
            .disabled=${!this.isCuratorValid()}
            @click=${() => this.updateCurator()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
