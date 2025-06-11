use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, StdResult,
};

use crate::msg::{QueryMsg, DocumentResponse, CollectionResponse};
use crate::state::{DOCUMENTS, ADMIN, COLLECTION_PERMISSIONS, USER_ROLES};

pub mod get;
pub mod collection;
pub mod permissions;
pub mod stats;

pub fn query(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Get { collection, document } => {
            get::query_get(deps, collection, document)
        }
        QueryMsg::Collection { collection, limit, start_after } => {
            collection::query_collection(deps, collection, limit, start_after)
        }
        QueryMsg::UserDocuments { owner, collection, limit, start_after } => {
            collection::query_user_documents(deps, owner, collection, limit, start_after)
        }
        QueryMsg::GetCollectionPermissions { collection } => {
            permissions::query_collection_permissions(deps, collection)
        }
        QueryMsg::GetUserRoles { user } => {
            permissions::query_user_roles(deps, user)
        }
        QueryMsg::CheckPermission { collection, user, action } => {
            permissions::query_check_permission(deps, collection, user, action)
        }
        QueryMsg::GetDocumentStats { collection } => {
            stats::query_document_stats(deps, collection)
        }
        QueryMsg::ListCollections { limit, start_after } => {
            stats::query_list_collections(deps, limit, start_after)
        }
        QueryMsg::SearchDocuments { collection, query, limit, start_after } => {
            stats::query_search_documents(deps, collection, query, limit, start_after)
        }
    }
}

// Permission checking helper function
pub fn check_permission(
    deps: Deps,
    collection: &str,
    user: &Addr,
    action: &str,
) -> StdResult<bool> {
    // Admin always has permission
    let admin = ADMIN.load(deps.storage)?;
    if user == &admin {
        return Ok(true);
    }
    
    // Get collection permissions (use defaults if not set)
    let permissions = COLLECTION_PERMISSIONS.may_load(deps.storage, collection.to_string())?
        .unwrap_or_default();
    
    let permission_level = match action {
        "create" => &permissions.create,
        "update" => &permissions.update,
        "delete" => &permissions.delete,
        "read" => &permissions.read,
        _ => return Ok(false), // Unknown action
    };
    
    match permission_level {
        PermissionLevel::Anyone => Ok(true),
        PermissionLevel::AdminOnly => Ok(user == &admin),
        PermissionLevel::AllowList(allowed_users) => Ok(allowed_users.contains(&user.to_string())),
        PermissionLevel::DenyList(denied_users) => Ok(!denied_users.contains(&user.to_string())),
        PermissionLevel::RequireRole(required_role) => {
            let user_roles = USER_ROLES.may_load(deps.storage, user.clone())?
                .unwrap_or_default();
            Ok(user_roles.contains(required_role))
        }
    }
} 