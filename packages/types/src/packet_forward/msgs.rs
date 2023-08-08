use crate::packet_forward::{
    ibc_hooks::IBCLifecycleComplete,
    types::{Config, FeeConfig, Request},
};

use cosmwasm_schema::{cw_serde, QueryResponses};
use std::time::Duration;

#[cw_serde]
pub struct InstantiateMsg {
    pub treasury: String,
    pub fee: FeeConfig,
}

#[cw_serde]
pub enum ExecuteMsg {
    Forward(ForwardMsg),
    ClaimFailedRequest(ClaimFailedRequestMsg),
    UpdateConfig(UpdateConfigMsg),
}

#[cw_serde]
pub struct ForwardMsg {
    pub emergency_claimer: String,
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: Duration,
    pub memo: String,
}

#[cw_serde]
pub struct ClaimFailedRequestMsg {
    pub request_id: u64,
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
    pub treasury: Option<String>,
    pub fee: Option<FeeConfig>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(Vec<Request>)]
    FailedRequests { address: String },
}

#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}

#[cw_serde]
pub struct MigrateMsg {}
