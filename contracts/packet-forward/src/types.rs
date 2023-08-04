use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct Request {
    pub emergency_claimer: Addr,
    pub coin: Coin,
}
