use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cosmwasm_std::Decimal;
use cosmwasm_std::Uint128;

#[cw_serde]
pub enum Destination {
    PacketForwardMiddleware,
    IbcHooks { contract: String },
}

#[cw_serde]
pub struct Route {
    pub port: String,
    pub channel: String,
    pub dst_bech32_prefix: String,
    pub destination: Destination,
}

#[cw_serde]
pub struct FeeInfo {
    pub commission_rate: Decimal,
    pub max_fee: Uint128,
}

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub denom: String,
    pub timeout_seconds: u64,
    pub routes: Vec<Route>,
    pub fee: FeeInfo,
}
