use crate::types::{Config, Request, RequestId, Sequence, SubMsgId, SubMsgType};
use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

pub const SUB_MSG_ID: Item<SubMsgId> = Item::new("sub_msg_id");
pub const SUB_MSG_TYPE: Map<SubMsgId, SubMsgType> = Map::new("sub_msg_type");

pub const REQUEST_ID: Item<SubMsgId> = Item::new("request_id");
pub const INITIATED_REQUESTS: Map<SubMsgId, Request> = Map::new("initiated_requests");
pub const PENDING_REQUESTS: Map<Sequence, Request> = Map::new("pending_requests");
pub const FAILED_REQUESTS: Map<(&Addr, RequestId), Request> = Map::new("failed_requests");

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
