use hdk::prelude::*;
use library_integrity::*;
#[hdk_extern]
pub fn create_contributor_permission(
    contributor_permission: ContributorPermission,
) -> ExternResult<Record> {
    let contributor_permission_hash = create_entry(&EntryTypes::ContributorPermission(
        contributor_permission.clone(),
    ))?;
    create_link(
        contributor_permission.for_collective.clone(),
        contributor_permission_hash.clone(),
        LinkTypes::DeveloperCollectiveToContributorPermissions,
        // We also add a tag to the link for which agent that permission is
        LinkTag::new(contributor_permission.for_agent.get_raw_39()),
    )?;
    create_link(
        contributor_permission.for_agent.clone(),
        contributor_permission_hash.clone(),
        LinkTypes::ContributorToContributorPermissions,
        // We also add a tag to the link for which collective that permission is
        LinkTag::new(contributor_permission.for_collective.get_raw_39()),
    )?;
    let record = get(contributor_permission_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(
            "Could not find the newly created ContributorPermission".to_string()
        )),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_contributor_permission(
    contributor_permission_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(contributor_permission_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}
#[hdk_extern]
pub fn get_contributor_permissions_for_developer_collective(
    developer_collective_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            developer_collective_hash,
            LinkTypes::DeveloperCollectiveToContributorPermissions,
        )?
        .build(),
    )
}
#[hdk_extern]
pub fn get_contributor_permissions_for_contributor(
    contributor: AgentPubKey,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(contributor, LinkTypes::ContributorToContributorPermissions)?
            .build(),
    )
}

/// Gets the least restrictive permission status for a developer collective
#[hdk_extern]
pub fn get_my_permission(
    developer_collective_hash: ActionHash,
) -> ExternResult<Option<ActionHash>> {
    let agent = agent_info()?.agent_initial_pubkey;
    return get_agent_permission(GetAgentPermissionInput {
        developer_collective_hash,
        agent,
    });
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAgentPermissionInput {
    developer_collective_hash: ActionHash,
    agent: AgentPubKey,
}

#[hdk_extern]
pub fn get_agent_permission(input: GetAgentPermissionInput) -> ExternResult<Option<ActionHash>> {
    let developer_collective_record = get(
        input.developer_collective_hash.clone(),
        GetOptions::default(),
    )?;
    match developer_collective_record {
        Some(r) => {
            if r.action().author() == &input.agent {
                // We have full Creator permission on that DeveloperCollective
                return Ok(Some(r.action_address().clone()));
            }
            // Get all permission entries for the DeveloperCollective and find the ones that
            // are for me, if any
            let permission_links = get_contributor_permissions_for_developer_collective(
                input.developer_collective_hash,
            )?;

            let links_to_my_permissions = permission_links
                .into_iter()
                .filter_map(|link| {
                    let maybe_permitted_agent = AgentPubKey::from_raw_39(link.tag.0.clone()).ok();
                    if let Some(permitted_agent) = maybe_permitted_agent {
                        if permitted_agent == input.agent {
                            return Some(link);
                        }
                    }
                    None
                })
                .collect::<Vec<Link>>();

            match links_to_my_permissions.len() {
                0 => Ok(None),
                _ => {
                    // Get each permission and check whether it is expired
                    let mut expiring_permissions = Vec::new();
                    for link in links_to_my_permissions {
                        let maybe_permission_action_hash = ActionHash::try_from(link.target).ok();
                        if let Some(permission_action_hash) = maybe_permission_action_hash {
                            let maybe_permission_record =
                                get(permission_action_hash, GetOptions::default())?;
                            if let Some(permission_record) = maybe_permission_record {
                                match permission_record
                                    .entry()
                                    .to_app_option::<crate::ContributorPermission>()
                                {
                                    Ok(maybe_permission) => {
                                        if let Some(permission) = maybe_permission {
                                            match permission.expiry {
                                                None => {
                                                    return Ok(Some(
                                                        permission_record.action_address().clone(),
                                                    ))
                                                }
                                                Some(expiry) => {
                                                    expiring_permissions.push((
                                                        expiry,
                                                        permission_record.action_address().clone(),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => (),
                                }
                            }
                        }
                    }

                    let max_expiry_permission = expiring_permissions
                        .into_iter()
                        .max_by(|a, b| a.0.cmp(&b.0));

                    match max_expiry_permission {
                        Some((expiry, action_hash)) => {
                            let now = sys_time()?;
                            if now > expiry {
                                Ok(None)
                            } else {
                                Ok(Some(action_hash))
                            }
                        }
                        None => Ok(None),
                    }
                }
            }
        }
        None => Ok(None),
    }
}
