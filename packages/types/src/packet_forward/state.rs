use crate::packet_forward::types::{Config, Request, RequestId, Sequence, SubMsgId, SubMsgType};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

pub const SUB_MSG_ID: Item<SubMsgId> = Item::new("sub_msg_id");
pub const SUB_MSG_TYPE: Map<SubMsgId, SubMsgType> = Map::new("sub_msg_type");

pub const REQUEST_ID: Item<SubMsgId> = Item::new("request_id");
pub const INITIATED_REQUESTS: Map<SubMsgId, Request> = Map::new("initiated_requests");
pub const PENDING_REQUESTS: Map<Sequence, Request> = Map::new("pending_requests");
pub const FAILED_REQUESTS: Map<(&Addr, RequestId), Request> = Map::new("failed_requests");
