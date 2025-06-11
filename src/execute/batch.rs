use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::WriteOperation;
use crate::execute::{set, update, delete};

pub fn execute_batch_write(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operations: Vec<WriteOperation>,
) -> StdResult<Response> {
    for op in operations {
        match op.operation {
            WriteType::Set { data } => {
                set::execute_set(deps.branch(), env.clone(), info.clone(), op.collection, op.document, data)?;
            }
            WriteType::Update { data } => {
                update::execute_update(deps.branch(), env.clone(), info.clone(), op.collection, op.document, data)?;
            }
            WriteType::Delete => {
                delete::execute_delete(deps.branch(), env.clone(), info.clone(), op.collection, op.document)?;
            }
        }
    }
    
    Ok(Response::new().add_attribute("action", "batch_write"))
} 