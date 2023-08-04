use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

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
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct Request {
    pub id: u64,
    pub emergency_claimer: Addr,
    pub coin: Coin,
}
