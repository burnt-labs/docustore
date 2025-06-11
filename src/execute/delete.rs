use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::state::{DOCUMENTS, ADMIN};
use crate::query::check_permission;

pub fn execute_delete(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    document_id: String,
) -> StdResult<Response> {
    let key = (collection.clone(), document_id.clone());
    
    // Check if document exists
    let doc = DOCUMENTS.load(deps.storage, key.clone())?;
    
    // Check if user owns document OR has delete permission for collection
    let admin = ADMIN.load(deps.storage)?;
    let owns_document = doc.owner == info.sender;
    let is_admin = info.sender == admin;
    let has_delete_permission = check_permission(deps.as_ref(), &collection, &info.sender, "delete")?;
    
    if !owns_document && !is_admin && !has_delete_permission {
        return Err(StdError::generic_err("Unauthorized: Must own document or have delete permission"));
    }
    
    DOCUMENTS.remove(deps.storage, key);
    
    Ok(Response::new()
        .add_attribute("action", "delete")
        .add_attribute("collection", collection)
        .add_attribute("document", document_id))
} 