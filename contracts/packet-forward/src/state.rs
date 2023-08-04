use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Request {
    pub id: u64,
    pub emergency_claimer: Addr,
    pub coin: Coin,
}

pub const REQUEST_ID: Item<u64> = Item::new("request_id");
pub const PENDING: Map<u64, Request> = Map::new("pending");
pub const FAILED: Map<u64, Request> = Map::new("pending");
