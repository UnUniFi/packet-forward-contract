use packet_forward_types::packet_forward::{FAILED_REQUESTS, Request};
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_failed_requests(deps: Deps, addr: String) -> StdResult<Vec<Request>> {
    let addr = deps.api.addr_validate(&addr.as_str())?;

    let failed_requests = FAILED_REQUESTS
        .prefix(&addr)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| Ok(item?.1))
        .collect::<StdResult<Vec<Request>>>()?;

    Ok(failed_requests)
}
