use packet_forward_types::packet_forward::{CONFIG, Config};
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(config)
}
