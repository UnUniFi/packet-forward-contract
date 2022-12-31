use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap {
        amount: Coin,
        routes: Vec<SwapRoute>,
    },
}

#[cw_serde]
pub struct SwapRoute {
    pair_name: String,
    direction: SwapDirection,
}

#[cw_serde]
pub enum SwapDirection {
    BaseToQuote,
    QuoteToBase,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
