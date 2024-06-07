use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ContributorPermissionClaim {
    pub permission_hash: ActionHash,
}

/// A ContributorPermissionClaim is used to ba able to validate the deletion of links.
/// A ContributorPermission claim entry needs to be created before deleting a link,
/// which can then be used in validation of the DeletLink via action.prev_action
///
/// Rules:
/// 1. A Contributor permission claim must point to a valid ContributorPermission Record
pub fn validate_create_contributor_permission_claim(
    _action: EntryCreationAction,
    contributor_permission_claim: ContributorPermissionClaim,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(contributor_permission_claim.permission_hash.clone())?;
    let _contributor_permission: crate::ContributorPermission = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "permission_hash must point to a valid ContributorPermission entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_contributor_permission_claim(
    _action: Update,
    _contributor_permission_claim: ContributorPermissionClaim,
    _original_action: EntryCreationAction,
    _original_contributor_permission: ContributorPermissionClaim,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ContributorPermissionClaims cannot be updated",
    )))
}
pub fn validate_delete_contributor_permission_claim(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_contributor_permission_claim: ContributorPermissionClaim,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ContributorPermissionClaims cannot be deleted",
    )))
}
