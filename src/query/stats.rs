use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, StdResult, Order,
};
use cw_storage_plus::Bound;
use std::collections::HashSet;

use crate::msg::{DocumentStats, CollectionInfo, CollectionListResponse, SearchResult, SearchResponse};
use crate::state::{Document, DOCUMENTS};

pub fn query_document_stats(
    deps: Deps,
    collection: String,
) -> StdResult<Binary> {
    let start = Bound::exclusive((collection.clone(), String::new()));
    let end = Bound::exclusive((format!("{}~", collection), String::new()));
    
    let mut total_documents = 0u64;
    let mut total_size = 0u64;
    let mut last_updated = 0u64;
    let mut unique_owners = HashSet::new();
    
    for item in DOCUMENTS.range(deps.storage, Some(start), Some(end), Order::Ascending) {
        let ((_, _), doc) = item?;
        total_documents += 1;
        total_size += doc.data.len() as u64;
        if doc.updated_at.seconds() > last_updated {
            last_updated = doc.updated_at.seconds();
        }
        unique_owners.insert(doc.owner);
    }
    
    let stats = DocumentStats {
        total_documents,
        total_size,
        last_updated,
        unique_owners: unique_owners.len() as u64,
    };
    
    to_json_binary(&stats)
}

pub fn query_list_collections(
    deps: Deps,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30) as usize;
    let mut collections = Vec::new();
    let mut current_collection = None;
    let mut document_count = 0u64;
    let mut last_activity = 0u64;
    
    // Get all documents and group by collection
    for item in DOCUMENTS.range(deps.storage, None, None, Order::Ascending) {
        let ((coll, _), doc) = item?;
        
        if current_collection.as_ref() != Some(&coll) {
            // Save previous collection if exists
            if let Some(prev_coll) = current_collection {
                if collections.len() >= limit {
                    break;
                }
                collections.push(CollectionInfo {
                    name: prev_coll,
                    document_count,
                    last_activity,
                });
            }
            
            // Start new collection
            current_collection = Some(coll);
            document_count = 1;
            last_activity = doc.updated_at.seconds();
        } else {
            document_count += 1;
            if doc.updated_at.seconds() > last_activity {
                last_activity = doc.updated_at.seconds();
            }
        }
    }
    
    // Add the last collection if exists
    if let Some(prev_coll) = current_collection {
        if collections.len() < limit {
            collections.push(CollectionInfo {
                name: prev_coll,
                document_count,
                last_activity,
            });
        }
    }
    
    let next_start_after = if collections.len() == limit {
        collections.last().map(|info| info.name.clone())
    } else {
        None
    };
    
    let response = CollectionListResponse {
        collections,
        next_start_after,
    };
    
    to_json_binary(&response)
}

pub fn query_search_documents(
    deps: Deps,
    collection: String,
    query: String,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive((collection.clone(), s.clone())));
    let end = Bound::exclusive((format!("{}~", collection), String::new()));
    
    let mut results = Vec::new();
    
    // Parse the query as JSON
    let query_value: serde_json::Value = serde_json::from_str(&query)
        .map_err(|e| StdError::generic_err(format!("Invalid query JSON: {}", e)))?;
    
    for item in DOCUMENTS.range(deps.storage, start, Some(end), Order::Ascending) {
        let ((_, doc_id), doc) = item?;
        
        // Parse document data as JSON
        let doc_value: serde_json::Value = serde_json::from_str(&doc.data)
            .map_err(|e| StdError::generic_err(format!("Invalid document JSON: {}", e)))?;
        
        // Find matching fields and calculate relevance
        let (matching_fields, relevance) = find_matches(&doc_value, &query_value);
        
        if !matching_fields.is_empty() {
            results.push(SearchResult {
                document_id,
                matching_fields,
                relevance_score: relevance,
            });
            
            if results.len() >= limit {
                break;
            }
        }
    }
    
    let next_start_after = if results.len() == limit {
        results.last().map(|result| result.document_id.clone())
    } else {
        None
    };
    
    let response = SearchResponse {
        results,
        next_start_after,
    };
    
    to_json_binary(&response)
}

// Helper function to find matching fields and calculate relevance
fn find_matches(doc: &serde_json::Value, query: &serde_json::Value) -> (Vec<String>, f64) {
    let mut matching_fields = Vec::new();
    let mut relevance = 0.0;
    
    if let (serde_json::Value::Object(doc_obj), serde_json::Value::Object(query_obj)) = (doc, query) {
        for (key, query_value) in query_obj {
            if let Some(doc_value) = doc_obj.get(key) {
                if doc_value == query_value {
                    matching_fields.push(key.clone());
                    relevance += 1.0;
                } else if let (serde_json::Value::String(doc_str), serde_json::Value::String(query_str)) = (doc_value, query_value) {
                    // Simple text matching for strings
                    if doc_str.to_lowercase().contains(&query_str.to_lowercase()) {
                        matching_fields.push(key.clone());
                        relevance += 0.5;
                    }
                }
            }
        }
    }
    
    (matching_fields, relevance)
} 