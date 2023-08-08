use crate::error::ContractError;
use crate::proto::ibc::applications::transfer::v1::MsgTransferResponse;
use packet_forward_types::packet_forward::{
    SUB_MSG_TYPE, SubMsgId, SubMsgType, INITIATED_REQUESTS, PENDING_REQUESTS
};
use cosmwasm_std::{DepsMut, Response, SubMsgResponse};
use prost::Message;

#[cfg(not(feature = "library"))]
pub fn reply_ok(
    deps: DepsMut,
    id: SubMsgId,
    res: SubMsgResponse,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let sub_msg_type = SUB_MSG_TYPE.load(deps.storage, id)?;

    response = match sub_msg_type {
        SubMsgType::InitiateRequest => {
            let transfer_response = MsgTransferResponse::decode(&res.data.unwrap().0[..])?;
            let request = INITIATED_REQUESTS.load(deps.storage, id)?;

            PENDING_REQUESTS.save(deps.storage, transfer_response.sequence, &request)?;
            INITIATED_REQUESTS.remove(deps.storage, id);

            // TODO: add events
            response.add_attribute("action", "initiate_request")
        }
    };

    SUB_MSG_TYPE.remove(deps.storage, id);

    Ok(response)
}
