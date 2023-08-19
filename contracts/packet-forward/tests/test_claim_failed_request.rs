use std::time::Duration;

use cosmwasm_std::{Decimal, Uint128, Coin, Api};
use cosmwasm_std::testing::{mock_env, mock_info};
use cw_utils::one_coin;
use helpers::{th_query, CONTRACT_PORT, DEFAULT_TIMEOUT};
use packet_forward::error::ContractError;
use packet_forward::execute::claim_failed_request::execute_claim_failed_request;
use packet_forward::execute::forward::execute_forward;
use packet_forward::msgs::{QueryMsg, UpdateConfigMsg, ForwardMsg, ClaimFailedRequestMsg};
use packet_forward::execute::update_config::execute_update_config;
use packet_forward::state::FAILED_REQUESTS;
use packet_forward::types::{Config, FeeConfig, Request};

use crate::helpers::setup;

mod helpers;

#[test]
fn test_claim_failed_request() {
    let mut deps = setup();
    let sender = "anyone";

    // Success case
    {
        let claimer = deps.api.addr_validate(sender).unwrap();
        let request = Request {
            id: 1,
            emergency_claimer: claimer.clone(),
            coin: Coin{denom: String::from("uguu"), amount: Uint128::from(100 as u32)},
        };

        FAILED_REQUESTS.save(
            deps.as_mut().storage,
            (&claimer, request.id),
            &request,
        ).unwrap();

        let claim_failed_request_msg = ClaimFailedRequestMsg {
            request_id: 1,
        };

        let res = execute_claim_failed_request(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            claim_failed_request_msg,
        )
            .unwrap();

        assert_eq!(1, res.messages.len());
    }
}
