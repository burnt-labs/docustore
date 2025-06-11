use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use serde_json;

use crate::state::{Document, DOCUMENTS};
use crate::query::check_permission;

pub fn execute_set(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    document_id: String,
    data: String,
) -> StdResult<Response> {
    // Check create permission
    if !check_permission(deps.as_ref(), &collection, &info.sender, "create")? {
        return Err(StdError::generic_err("Insufficient permissions to create documents in this collection"));
    }
    
    // Validate JSON
    serde_json::from_str::<serde_json::Value>(&data)
        .map_err(|e| StdError::generic_err(e.to_string()))?;
    
    let doc = Document {
        data,
        owner: info.sender.clone(),
        created_at: env.block.time,
        updated_at: env.block.time,
    };
    
    let key = (collection.clone(), document_id.clone());
    DOCUMENTS.save(deps.storage, key, &doc)?;
    
    Ok(Response::new()
        .add_attribute("action", "set")
        .add_attribute("collection", collection)
        .add_attribute("document", document_id)
        .add_attribute("owner", info.sender))
} 