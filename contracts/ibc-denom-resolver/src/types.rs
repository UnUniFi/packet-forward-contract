use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cosmwasm_std::Decimal;
use cosmwasm_std::Uint128;
use std::time::Duration;

#[cw_serde]
pub enum Destination {
    Terminal,
    PacketForwardMiddleware,
    IbcHooks { contract: String },
}

#[cw_serde]
pub struct Route {
    pub src_port: String,
    pub src_channel: String,
    pub destination: Destination,
}
/// https://medium.com/the-interchain-foundation/moving-beyond-simple-token-transfers-d42b2b1dc29b
/// https://github.com/strangelove-ventures/packet-forward-middleware
#[cw_serde]
pub struct Memo {
    pub forward: Option<PacketForwardMetadata>,
    pub wasm: Option<IbcHooksMetadata>,
}

#[cw_serde]
pub struct PacketForwardMetadata {
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: Option<String>,
    pub retries: Option<u32>,
    pub next: Option<Box<Memo>>,
}

#[cw_serde]
pub struct Msg {
    pub raw_message_fields: String,
}

#[cw_serde]
pub struct IbcHooksMetadata {
    pub contract: String,
    pub msg: Msg,
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
    pub timeout: Duration,
    pub routes: Vec<Route>,
    pub fee: FeeInfo,
}
