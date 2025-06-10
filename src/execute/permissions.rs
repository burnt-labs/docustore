use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::state::{CollectionPermissions, ADMIN, COLLECTION_PERMISSIONS, USER_ROLES};

pub fn execute_set_permissions(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    permissions: CollectionPermissions,
) -> StdResult<Response> {
    // Only admin can set permissions
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(StdError::generic_err("Only admin can set collection permissions"));
    }
    
    COLLECTION_PERMISSIONS.save(deps.storage, collection.clone(), &permissions)?;
    
    Ok(Response::new()
        .add_attribute("action", "set_permissions")
        .add_attribute("collection", collection))
}

pub fn execute_grant_role(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user: String,
    role: String,
) -> StdResult<Response> {
    // Only admin can grant roles
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(StdError::generic_err("Only admin can grant roles"));
    }
    
    let user_addr = deps.api.addr_validate(&user)?;
    let mut user_roles = USER_ROLES.may_load(deps.storage, user_addr.clone())?.unwrap_or_default();
    
    if !user_roles.contains(&role) {
        user_roles.push(role.clone());
        USER_ROLES.save(deps.storage, user_addr, &user_roles)?;
    }
    
    Ok(Response::new()
        .add_attribute("action", "grant_role")
        .add_attribute("user", user)
        .add_attribute("role", role))
}

pub fn execute_revoke_role(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user: String,
    role: String,
) -> StdResult<Response> {
    // Only admin can revoke roles
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(StdError::generic_err("Only admin can revoke roles"));
    }
    
    let user_addr = deps.api.addr_validate(&user)?;
    let mut user_roles = USER_ROLES.may_load(deps.storage, user_addr.clone())?.unwrap_or_default();
    
    user_roles.retain(|r| r != &role);
    USER_ROLES.save(deps.storage, user_addr, &user_roles)?;
    
    Ok(Response::new()
        .add_attribute("action", "revoke_role")
        .add_attribute("user", user)
        .add_attribute("role", role))
}

pub fn execute_transfer_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_admin: String,
) -> StdResult<Response> {
    // Only current admin can transfer admin
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(StdError::generic_err("Only admin can transfer admin role"));
    }
    
    let new_admin_addr = deps.api.addr_validate(&new_admin)?;
    ADMIN.save(deps.storage, &new_admin_addr)?;
    
    Ok(Response::new()
        .add_attribute("action", "transfer_admin")
        .add_attribute("old_admin", admin)
        .add_attribute("new_admin", new_admin_addr))
} 