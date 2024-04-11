import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { DeveloperCollective } from './types';

@customElement('edit-developer-collective')
export class EditDeveloperCollective extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalDeveloperCollectiveHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentDeveloperCollective() {
    return decode((this.currentRecord.entry as any).Present.entry) as DeveloperCollective;
  }
 

  isDeveloperCollectiveValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-developer-collective element`);
    }

    if (this.originalDeveloperCollectiveHash === undefined) {
      throw new Error(`The originalDeveloperCollectiveHash property is required for the edit-developer-collective element`);
    }
    
  }

  async updateDeveloperCollective() {
    const developerCollective: DeveloperCollective = { 
      name: this.currentDeveloperCollective.name,
      description: this.currentDeveloperCollective.description,
      website: this.currentDeveloperCollective.website,
      contact: this.currentDeveloperCollective.contact,
      icon: this.currentDeveloperCollective.icon,
      meta_data: this.currentDeveloperCollective.meta_data,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'update_developer_collective',
        payload: {
          original_developer_collective_hash: this.originalDeveloperCollectiveHash,
          previous_developer_collective_hash: this.currentRecord.signed_action.hashed.hash,
          updated_developer_collective: developerCollective
        },
      });
  
      this.dispatchEvent(new CustomEvent('developer-collective-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalDeveloperCollectiveHash: this.originalDeveloperCollectiveHash,
          previousDeveloperCollectiveHash: this.currentRecord.signed_action.hashed.hash,
          updatedDeveloperCollectiveHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the developer collective: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Developer Collective</span>


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
            .disabled=${!this.isDeveloperCollectiveValid()}
            @click=${() => this.updateDeveloperCollective()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
