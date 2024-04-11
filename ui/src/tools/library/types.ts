import { 
  Record, 
  ActionHash, 
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type LibrarySignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'ContributorPermission'; } & ContributorPermission)
 | ({ type: 'DeveloperCollective'; } & DeveloperCollective)
 | ({  type: 'Curator'; } & Curator);



export interface Curator { 
  name: string;

  description: string;

  icon: string;

  website: string | undefined;

  email: string | undefined;

  meta_data: string | undefined;
}





export interface DeveloperCollective { 
  name: string;

  description: string;

  website: string;

  contact: string;

  icon: string;

  meta_data: string;
}





export interface ContributorPermission { 
  for_collective: ActionHash;

  for_agent: AgentPubKey;

  expiry: number | undefined;
}


