use cosmwasm_std::{
    to_json_binary, Binary, Deps, StdResult,
};

use crate::msg::DocumentResponse;
use crate::state::DOCUMENTS;

pub fn query_get(
    deps: Deps,
    collection: String,
    document_id: String,
) -> StdResult<Binary> {
    let key = (collection, document_id);
    let doc = DOCUMENTS.may_load(deps.storage, key)?;
    
    let response = DocumentResponse {
        exists: doc.is_some(),
        document: doc,
    };
    
    to_json_binary(&response)
} 