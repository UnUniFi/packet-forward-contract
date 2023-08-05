use crate::error::ContractError;
use crate::msgs::ClaimFailedRequestMsg;
use crate::state::get_sub_msg_id;
use crate::state::FAILED_REQUESTS;
use crate::{state::SUB_MSG_TYPE, types::SubMsgType};
use cosmwasm_std::ReplyOn;
use cosmwasm_std::SubMsg;
use cosmwasm_std::{BankMsg, CosmosMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Only owner can execute it.
#[cfg(not(feature = "library"))]
pub fn execute_claim_failed_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ClaimFailedRequestMsg,
) -> Result<Response, ContractError> {
    let request = FAILED_REQUESTS.load(deps.storage, (&info.sender, msg.request_id))?;

    let sub_msg_id = get_sub_msg_id(deps.storage)?;

    let sub_msg = SubMsg {
        id: sub_msg_id,
        msg: CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![request.coin],
        }),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    SUB_MSG_TYPE.save(
        deps.storage,
        sub_msg_id,
        &SubMsgType::ClaimFailedRequest(request.id, request.emergency_claimer),
    )?;

    let response = Response::new().add_submessage(sub_msg);
    // TODO: add events

    Ok(response)
}
