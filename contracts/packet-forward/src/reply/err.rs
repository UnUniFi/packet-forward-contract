use crate::error::ContractError;
use crate::state::INITIATED_REQUESTS;
use crate::state::SUB_MSG_TYPE;
use crate::types::SubMsgId;
use crate::types::SubMsgType;
use cosmwasm_std::{DepsMut, Response};

#[cfg(not(feature = "library"))]
pub fn reply_err(deps: DepsMut, id: SubMsgId, _err: String) -> Result<Response, ContractError> {
    let sub_msg_type = SUB_MSG_TYPE.load(deps.storage, id)?;

    let response = match sub_msg_type {
        SubMsgType::InitiateRequest() => {
            INITIATED_REQUESTS.remove(deps.storage, id);

            // TODO: add events
            Response::new()
        }
        _ => Response::new(),
    };

    SUB_MSG_TYPE.remove(deps.storage, id);

    Ok(response)
}
