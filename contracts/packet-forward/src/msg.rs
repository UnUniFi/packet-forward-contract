use crate::types::{Config, Memo, Request};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::IbcTimeout;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),
    Forward(ForwardMsg),
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub struct ForwardMsg {
    pub emergency_claimer: String,
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: IbcTimeout,
    pub memo: Option<Memo>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(Vec<Request>)]
    PendingRequests { address: String },
    #[returns(Vec<Request>)]
    FailedRequests { address: String },
}

#[cw_serde]
pub struct MigrateMsg {}
