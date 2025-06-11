use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use serde_json;

use crate::msg::{ExecuteMsg, WriteOperation, WriteType};
use crate::state::{Document, CollectionPermissions, DOCUMENTS, ADMIN, COLLECTION_PERMISSIONS, USER_ROLES};

pub mod set;
pub mod update;
pub mod delete;
pub mod batch;
pub mod permissions;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Set { collection, document, data } => {
            set::execute_set(deps, env, info, collection, document, data)
        }
        ExecuteMsg::Update { collection, document, data } => {
            update::execute_update(deps, env, info, collection, document, data)
        }
        ExecuteMsg::Delete { collection, document } => {
            delete::execute_delete(deps, env, info, collection, document)
        }
        ExecuteMsg::BatchWrite { operations } => {
            batch::execute_batch_write(deps, env, info, operations)
        }
        ExecuteMsg::SetCollectionPermissions { collection, permissions } => {
            permissions::execute_set_permissions(deps, env, info, collection, permissions)
        }
        ExecuteMsg::GrantRole { user, role } => {
            permissions::execute_grant_role(deps, env, info, user, role)
        }
        ExecuteMsg::RevokeRole { user, role } => {
            permissions::execute_revoke_role(deps, env, info, user, role)
        }
        ExecuteMsg::TransferAdmin { new_admin } => {
            permissions::execute_transfer_admin(deps, env, info, new_admin)
        }
    }
} 