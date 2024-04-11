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


import { clientContext } from '../../contexts';
import { ContributorPermission } from './types';

@customElement('contributor-permission-detail')
export class ContributorPermissionDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  contributorPermissionHash!: ActionHash;

  _fetchRecord = new Task(this, ([contributorPermissionHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'tools',
      zome_name: 'library',
      fn_name: 'get_contributor_permission',
      payload: contributorPermissionHash,
  }) as Promise<Record | undefined>, () => [this.contributorPermissionHash]);

  
  firstUpdated() {
    if (this.contributorPermissionHash === undefined) {
      throw new Error(`The contributorPermissionHash property is required for the contributor-permission-detail element`);
    }
  }


  renderDetail(record: Record) {
    const contributorPermission = decode((record.entry as any).Present.entry) as ContributorPermission;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
        </div>

      </div>
    `;
  }
  
  renderContributorPermission(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested contributor permission was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderContributorPermission(maybeRecord),
      error: (e: any) => html`<span>Error fetching the contributor permission: ${e.data.data}</span>`
    });
  }
}
