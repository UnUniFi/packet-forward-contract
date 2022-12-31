use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Exchange {
    pub name: String,
    pub contract_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pair {
    pub base_denom: String,
    pub quote_denom: String,
}

pub const STATE: Item<State> = Item::new("state");

pub const EXCHANGES: Map<String, Exchange> = Map::new("exchanges");

pub const PAIRS: Map<String, Vec<Pair>> = Map::new("pairs");
