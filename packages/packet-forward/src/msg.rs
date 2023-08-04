use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),
    Forward(ForwardMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ForwardMsg {
    pub emergency_claimer: String,
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub memo: Option<Memo>,
}

/// https://medium.com/the-interchain-foundation/moving-beyond-simple-token-transfers-d42b2b1dc29b
/// https://github.com/strangelove-ventures/packet-forward-middleware
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Memo {
    pub forward: Option<PacketForwardMetadata>,
    pub wasm: Option<IbcHooksMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PacketForwardMetadata {
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: Option<String>,
    pub retries: Option<u32>,
    pub next: Option<Box<Memo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Msg {
    raw_message_fields: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IbcHooksMetadata {
    contract: String,
    msg: Msg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
