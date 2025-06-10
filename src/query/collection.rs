use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, StdResult, Order,
};
use cw_storage_plus::Bound;

use crate::msg::CollectionResponse;
use crate::state::{Document, DOCUMENTS};

pub fn query_collection(
    deps: Deps,
    collection: String,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30) as usize;
    
    let start = start_after.as_ref().map(|s| Bound::exclusive((collection.clone(), s.clone())));
    let end = Bound::exclusive((format!("{}~", collection), String::new()));
    
    let documents: Vec<(String, Document)> = DOCUMENTS
        .range(deps.storage, start, Some(end), Order::Ascending)
        .take(limit)
        .map(|item| {
            let ((_, doc_id), doc) = item?;
            Ok((doc_id, doc))
        })
        .collect::<StdResult<Vec<_>>>()?;
    
    let next_start_after = if documents.len() == limit {
        documents.last().map(|(id, _)| id.clone())
    } else {
        None
    };
    
    let response = CollectionResponse {
        documents,
        next_start_after,
    };
    
    to_json_binary(&response)
}

pub fn query_user_documents(
    deps: Deps,
    owner: String,
    collection: Option<String>,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<Binary> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(30) as usize;
    
    let start = if let (Some(coll), Some(s)) = (collection.clone(), start_after.clone()) {
        Some(Bound::exclusive((coll, s)))
    } else {
        None
    };
    
    let documents: Vec<(String, Document)> = DOCUMENTS
        .idx
        .owner
        .prefix(owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            let (key, doc) = item.ok()?;
            let (coll, doc_id) = key;
            
            // Filter by collection if specified
            if let Some(ref filter_collection) = collection {
                if &coll != filter_collection {
                    return None;
                }
            }
            
            Some((doc_id, doc))
        })
        .take(limit)
        .collect();
    
    let next_start_after = if documents.len() == limit {
        documents.last().map(|(id, _)| id.clone())
    } else {
        None
    };
    
    let response = CollectionResponse {
        documents,
        next_start_after,
    };
    
    to_json_binary(&response)
} 