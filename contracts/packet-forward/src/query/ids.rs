use packet_forward_types::packet_forward::{RequestId, SubMsgId, SUB_MSG_ID, REQUEST_ID};
use cosmwasm_std::{StdResult, Storage};

pub fn get_sub_msg_id(storage: &mut dyn Storage) -> StdResult<SubMsgId> {
    let sub_msg_id = SUB_MSG_ID.load(storage)?;
    SUB_MSG_ID.save(storage, &(sub_msg_id + 1))?;

    Ok(sub_msg_id)
}

pub fn get_request_id(storage: &mut dyn Storage) -> StdResult<RequestId> {
    let request_id = REQUEST_ID.load(storage)?;
    REQUEST_ID.save(storage, &(request_id + 1))?;

    Ok(request_id)
}
