import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { DeveloperCollective } from './types';

@customElement('create-developer-collective')
export class CreateDeveloperCollective extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  name!: string;
  @property()
  description!: string;
  @property()
  website!: string;
  @property()
  contact!: string;
  @property()
  icon!: string;
  @property()
  metaData!: string;

  
  firstUpdated() {
    if (this.name === undefined) {
      throw new Error(`The name input is required for the create-developer-collective element`);
    }
    if (this.description === undefined) {
      throw new Error(`The description input is required for the create-developer-collective element`);
    }
    if (this.website === undefined) {
      throw new Error(`The website input is required for the create-developer-collective element`);
    }
    if (this.contact === undefined) {
      throw new Error(`The contact input is required for the create-developer-collective element`);
    }
    if (this.icon === undefined) {
      throw new Error(`The icon input is required for the create-developer-collective element`);
    }
    if (this.metaData === undefined) {
      throw new Error(`The metaData input is required for the create-developer-collective element`);
    }
  }

  isDeveloperCollectiveValid() {
    return true;
  }

  async createDeveloperCollective() {
    const developerCollective: DeveloperCollective = { 
        name: this.name,
        description: this.description,
        website: this.website,
        contact: this.contact,
        icon: this.icon,
        meta_data: this.metaData,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'create_developer_collective',
        payload: developerCollective,
      });

      this.dispatchEvent(new CustomEvent('developer-collective-created', {
        composed: true,
        bubbles: true,
        detail: {
          developerCollectiveHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the developer collective: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Developer Collective</span>


        <mwc-button 
          raised
          label="Create Developer Collective"
          .disabled=${!this.isDeveloperCollectiveValid()}
          @click=${() => this.createDeveloperCollective()}
        ></mwc-button>
    </div>`;
  }
}
