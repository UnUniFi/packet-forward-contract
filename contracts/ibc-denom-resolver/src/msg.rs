use crate::types::{Config, FeeInfo, Route};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub timeout_seconds: u64,
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
    pub timeout_seconds: Option<u64>,
    pub fee: Option<FeeInfo>,
}

#[cw_serde]
pub struct SwapMsg {
    pub recipient: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
