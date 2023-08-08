use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub treasury: Addr,
    pub fee: FeeConfig,
}

#[cw_serde]
pub struct FeeConfig {
    pub commission_rate: Decimal,
}

pub type SubMsgId = u64;
pub type RequestId = u64;
pub type Sequence = u64;

#[cw_serde]
pub struct Request {
    pub id: RequestId,
    pub emergency_claimer: Addr,
    pub coin: Coin,
}

#[cw_serde]
pub enum SubMsgType {
    InitiateRequest,
}
