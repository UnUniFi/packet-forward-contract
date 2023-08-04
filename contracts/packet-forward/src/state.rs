use crate::types::{Config, Request};
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

pub const REQUEST_ID: Item<u64> = Item::new("request_id");
pub const INITIATED_REQUESTS: Map<u64, (Addr, Coin)> = Map::new("initiated_requests");
pub const PENDING_REQUESTS: Map<(&Addr, u64), Request> = Map::new("pending_requests");
pub const FAILED_REQUESTS: Map<(&Addr, u64), Request> = Map::new("failed_requests");
