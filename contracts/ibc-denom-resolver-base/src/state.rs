use cosmwasm_std::{Addr, Uint128};
use ibc_denom_resolver::resolver::{FeeInfo, Route};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub denom: String,
    pub routes: Vec<Route>,
    pub fee: FeeInfo,
}

pub const CONFIG: Item<Config> = Item::new("config");
