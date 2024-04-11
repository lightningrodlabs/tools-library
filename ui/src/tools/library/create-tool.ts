import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { Tool } from './types';

@customElement('create-tool')
export class CreateTool extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  developerCollective!: ActionHash;
  @property()
  permissionHash!: ActionHash;
  @property()
  title!: string;
  @property()
  subtitle!: string;
  @property()
  description!: string;
  @property()
  icon!: string;
  @property()
  source!: string;
  @property()
  hashes!: string;
  @property()
  changelog: string | undefined;
  @property()
  metaData: string | undefined;
  @property()
  deprecation: string | undefined;

  
  firstUpdated() {
    if (this.developerCollective === undefined) {
      throw new Error(`The developerCollective input is required for the create-tool element`);
    }
    if (this.permissionHash === undefined) {
      throw new Error(`The permissionHash input is required for the create-tool element`);
    }
    if (this.title === undefined) {
      throw new Error(`The title input is required for the create-tool element`);
    }
    if (this.subtitle === undefined) {
      throw new Error(`The subtitle input is required for the create-tool element`);
    }
    if (this.description === undefined) {
      throw new Error(`The description input is required for the create-tool element`);
    }
    if (this.icon === undefined) {
      throw new Error(`The icon input is required for the create-tool element`);
    }
    if (this.source === undefined) {
      throw new Error(`The source input is required for the create-tool element`);
    }
    if (this.hashes === undefined) {
      throw new Error(`The hashes input is required for the create-tool element`);
    }
  }

  isToolValid() {
    return true;
  }

  async createTool() {
    const tool: Tool = { 
        developer_collective: this.developerCollective,
        permission_hash: this.permissionHash,
        title: this.title,
        subtitle: this.subtitle,
        description: this.description,
        icon: this.icon,
        source: this.source,
        hashes: this.hashes,
        changelog: this.changelog,
        meta_data: this.metaData,
        deprecation: this.deprecation,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'tools',
        zome_name: 'library',
        fn_name: 'create_tool',
        payload: tool,
      });

      this.dispatchEvent(new CustomEvent('tool-created', {
        composed: true,
        bubbles: true,
        detail: {
          toolHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the tool: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Tool</span>


        <mwc-button 
          raised
          label="Create Tool"
          .disabled=${!this.isToolValid()}
          @click=${() => this.createTool()}
        ></mwc-button>
    </div>`;
  }
}
