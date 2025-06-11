use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use serde_json;

use crate::state::{Document, DOCUMENTS, ADMIN};
use crate::query::check_permission;

pub fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    document_id: String,
    data: String,
) -> StdResult<Response> {
    let key = (collection.clone(), document_id.clone());
    
    // Load existing document
    let mut doc = DOCUMENTS.load(deps.storage, key.clone())?;
    
    // Check if user owns document OR has update permission for collection
    let admin = ADMIN.load(deps.storage)?;
    let owns_document = doc.owner == info.sender;
    let is_admin = info.sender == admin;
    let has_update_permission = check_permission(deps.as_ref(), &collection, &info.sender, "update")?;
    
    if !owns_document && !is_admin && !has_update_permission {
        return Err(StdError::generic_err("Unauthorized: Must own document or have update permission"));
    }
    
    // Merge JSON data
    let existing: serde_json::Value = serde_json::from_str(&doc.data)
        .map_err(|e| StdError::generic_err(e.to_string()))?;
    let new_data: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| StdError::generic_err(e.to_string()))?;
    
    let merged = merge_json(existing, new_data);
    
    doc.data = serde_json::to_string(&merged)
        .map_err(|e| StdError::generic_err(e.to_string()))?;
    doc.updated_at = env.block.time;
    
    DOCUMENTS.save(deps.storage, key, &doc)?;
    
    Ok(Response::new()
        .add_attribute("action", "update")
        .add_attribute("collection", collection)
        .add_attribute("document", document_id))
}

// Helper function to merge JSON objects
fn merge_json(mut existing: serde_json::Value, new: serde_json::Value) -> serde_json::Value {
    if let (serde_json::Value::Object(ref mut existing_map), serde_json::Value::Object(new_map)) = (&mut existing, &new) {
        for (key, value) in new_map {
            existing_map.insert(key.clone(), value.clone());
        }
    }
    existing
} 