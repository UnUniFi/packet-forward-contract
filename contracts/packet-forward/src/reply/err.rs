use crate::error::ContractError;
use packet_forward_types::packet_forward::{INITIATED_REQUESTS, SUB_MSG_TYPE, SubMsgId, SubMsgType};
use cosmwasm_std::{DepsMut, Response};

#[cfg(not(feature = "library"))]
pub fn reply_err(deps: DepsMut, id: SubMsgId, _err: String) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let sub_msg_type = SUB_MSG_TYPE.load(deps.storage, id)?;

    response = match sub_msg_type {
        SubMsgType::InitiateRequest => {
            INITIATED_REQUESTS.remove(deps.storage, id);

            // TODO: add events
            response.add_attribute("action", "initiate_request")
        }
    };

    SUB_MSG_TYPE.remove(deps.storage, id);

    Ok(response)
}
