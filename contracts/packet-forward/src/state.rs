use crate::types::{Config, Request};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

type Id = u64;
type Sequence = u64;

pub const REQUEST_ID: Item<u64> = Item::new("request_id");
pub const INITIATED_REQUESTS: Map<Id, Request> = Map::new("initiated_requests");
pub const PENDING_REQUESTS: Map<Sequence, Request> = Map::new("pending_requests");
pub const FAILED_REQUESTS: Map<(&Addr, Sequence), Request> = Map::new("failed_requests");
