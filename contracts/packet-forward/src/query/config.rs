use crate::{state::CONFIG, types::Config};
use cosmwasm_std::{Deps, StdResult};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(config)
}
