use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, StdResult,
};

use crate::state::{COLLECTION_PERMISSIONS, USER_ROLES};

pub fn query_collection_permissions(
    deps: Deps,
    collection: String,
) -> StdResult<Binary> {
    let permissions = COLLECTION_PERMISSIONS.may_load(deps.storage, collection)?
        .unwrap_or_default();
    to_json_binary(&permissions)
}

pub fn query_user_roles(
    deps: Deps,
    user: String,
) -> StdResult<Binary> {
    let user_addr = deps.api.addr_validate(&user)?;
    let roles = USER_ROLES.may_load(deps.storage, user_addr)?
        .unwrap_or_default();
    to_json_binary(&roles)
}

pub fn query_check_permission(
    deps: Deps,
    collection: String,
    user: String,
    action: String,
) -> StdResult<Binary> {
    let user_addr = deps.api.addr_validate(&user)?;
    let has_permission = super::check_permission(deps, &collection, &user_addr, &action)?;
    to_json_binary(&has_permission)
} 