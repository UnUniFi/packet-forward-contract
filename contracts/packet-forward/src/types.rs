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

#[cw_serde]
pub struct Request {
    pub id: u64,
    pub emergency_claimer: Addr,
    pub coin: Coin,
}
