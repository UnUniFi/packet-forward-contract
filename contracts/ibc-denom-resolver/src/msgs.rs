use crate::types::{Config, FeeConfig, Route};
use cosmwasm_schema::{cw_serde, QueryResponses};
use std::time::Duration;

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub first_forward_contract: String,
    pub routes: Vec<Route>,
    pub treasury: String,
    pub fee: FeeConfig,
    pub timeout: Duration,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap(SwapMsg),
    UpdateConfig(UpdateConfigMsg),
}

#[cw_serde]
pub struct SwapMsg {
    pub receivers: Vec<String>,
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
    pub treasury: Option<String>,
    pub fee: Option<FeeConfig>,
    pub timeout: Option<Duration>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
