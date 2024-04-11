import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { Curator } from './types';

@customElement('create-curator')
export class CreateCurator extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  name!: string;
  @property()
  description!: string;
  @property()
  icon!: string;
  @property()
  website: string | undefined;
  @property()
  email: string | undefined;
  @property()
  metaData: string | undefined;

  
  firstUpdated() {
    if (this.name === undefined) {
      throw new Error(`The name input is required for the create-curator element`);
    }
    if (this.description === undefined) {
      throw new Error(`The description input is required for the create-curator element`);
    }
    if (this.icon === undefined) {
      throw new Error(`The icon input is required for the create-curator element`);
    }
  }

  isCuratorValid() {
    return true;
  }

  async createCurator() {
    const curator: Curator = { 
        name: this.name,
        description: this.description,
        icon: this.icon,
        website: this.website,
        email: this.email,
        meta_data: this.metaData,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'create_curator',
        payload: curator,
      });

      this.dispatchEvent(new CustomEvent('curator-created', {
        composed: true,
        bubbles: true,
        detail: {
          curatorHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the curator: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Curator</span>


        <mwc-button 
          raised
          label="Create Curator"
          .disabled=${!this.isCuratorValid()}
          @click=${() => this.createCurator()}
        ></mwc-button>
    </div>`;
  }
}
