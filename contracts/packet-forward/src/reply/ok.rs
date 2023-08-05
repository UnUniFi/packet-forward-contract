use crate::error::ContractError;
use crate::proto::ibc::applications::transfer::v1::MsgTransferResponse;
use crate::state::{INITIATED_REQUESTS, PENDING_REQUESTS};
use cosmwasm_std::{DepsMut, Response, SubMsgResponse};
use prost::Message;

#[cfg(not(feature = "library"))]
pub fn reply_ok(deps: DepsMut, id: u64, res: SubMsgResponse) -> Result<Response, ContractError> {
    let transfer_response = MsgTransferResponse::decode(&res.data.unwrap().0[..])?;
    let request = INITIATED_REQUESTS.load(deps.storage, id)?;

    PENDING_REQUESTS.save(deps.storage, transfer_response.sequence, &request)?;
    INITIATED_REQUESTS.remove(deps.storage, id);

    let response = Response::new();
    // TODO: add events

    Ok(response)
}
