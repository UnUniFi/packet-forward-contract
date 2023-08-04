use crate::{state::PENDING_REQUESTS, types::Request};
use cosmwasm_std::{Deps, StdResult};

pub fn query_pending_requests(deps: Deps, addr: String) -> StdResult<Vec<Request>> {
    let addr = deps.api.addr_validate(&addr.as_str())?;

    let pending_requests = PENDING_REQUESTS
        .prefix(&addr)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| Ok(item?.1))
        .collect::<StdResult<Vec<Request>>>()?;

    Ok(pending_requests)
}
