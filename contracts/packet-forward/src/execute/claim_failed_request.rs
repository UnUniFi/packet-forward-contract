use crate::error::ContractError;
use crate::msgs::ClaimFailedRequestMsg;
use crate::state::FAILED_REQUESTS;
use cosmwasm_std::{BankMsg, CosmosMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_claim_failed_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ClaimFailedRequestMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    // This line returns error when the sender of the msg is not the correct emergency claimer.
    let request = FAILED_REQUESTS.load(deps.storage, (&info.sender, msg.request_id))?;

    FAILED_REQUESTS.remove(deps.storage, (&request.emergency_claimer, request.id));
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![request.coin],
    });

    response = response.add_message(msg);

    response = response.add_attribute("action", "claim_failed_request");
    // TODO: add events

    Ok(response)
}
