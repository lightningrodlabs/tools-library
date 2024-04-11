import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { ContributorPermission } from './types';

@customElement('create-contributor-permission')
export class CreateContributorPermission extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  forCollective!: ActionHash;
  @property()
  forAgent!: AgentPubKey;
  @property()
  expiry: number | undefined;

  
  firstUpdated() {
    if (this.forCollective === undefined) {
      throw new Error(`The forCollective input is required for the create-contributor-permission element`);
    }
    if (this.forAgent === undefined) {
      throw new Error(`The forAgent input is required for the create-contributor-permission element`);
    }
  }

  isContributorPermissionValid() {
    return true;
  }

  async createContributorPermission() {
    const contributorPermission: ContributorPermission = { 
        for_collective: this.forCollective,
        for_agent: this.forAgent,
        expiry: this.expiry,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'create_contributor_permission',
        payload: contributorPermission,
      });

      this.dispatchEvent(new CustomEvent('contributor-permission-created', {
        composed: true,
        bubbles: true,
        detail: {
          contributorPermissionHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the contributor permission: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Contributor Permission</span>


        <mwc-button 
          raised
          label="Create Contributor Permission"
          .disabled=${!this.isContributorPermissionValid()}
          @click=${() => this.createContributorPermission()}
        ></mwc-button>
    </div>`;
  }
}
