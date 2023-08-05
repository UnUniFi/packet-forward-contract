use crate::types::{Config, FeeConfig, Route};
use cosmwasm_schema::{cw_serde, QueryResponses};
use std::time::Duration;

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub timeout: Duration,
    pub routes: Vec<Route>,
    pub fee: FeeConfig,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap(SwapMsg),
    UpdateConfig(UpdateConfigMsg),
    ClaimProfit(ClaimProfitMsg),
}

#[cw_serde]
pub struct SwapMsg {
    pub receivers: Vec<String>,
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
    pub timeout: Option<Duration>,
    pub fee: Option<FeeConfig>,
}

#[cw_serde]
pub struct ClaimProfitMsg {
    pub recipient: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
