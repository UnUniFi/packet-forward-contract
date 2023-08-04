use crate::types::{Config, Request};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

pub const REQUEST_ID: Item<u64> = Item::new("request_id");
pub const PENDING: Map<u64, Request> = Map::new("pending");
pub const FAILED: Map<u64, Request> = Map::new("pending");
