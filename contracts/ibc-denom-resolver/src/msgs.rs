use std::time::Duration;

use crate::types::{Config, FeeInfo, Route};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub timeout: Duration,
    pub routes: Vec<Route>,
    pub fee: FeeInfo,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),
    Swap(SwapMsg),
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
    pub timeout: Option<Duration>,
    pub fee: Option<FeeInfo>,
}

#[cw_serde]
pub struct SwapMsg {
    pub receivers: Vec<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
