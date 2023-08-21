use cosmwasm_std::{Uint128, Coin};
use cosmwasm_std::testing::{mock_env};
use packet_forward::ibc_hooks::IBCLifecycleComplete;
use packet_forward::state::{FAILED_REQUESTS, PENDING_REQUESTS};
use packet_forward::sudo::ibc_lifecycle_complete::ibc_lifecycle_complete;
use packet_forward::types::Request;

use crate::helpers::setup;

mod helpers;

#[test]
fn test_ibc_lifecycle_complete() {
    let mut deps = setup();
    let sender = "anyone";
    let claimer = deps.as_ref().api.addr_validate(sender).unwrap();

    // Success case of IBC lifecycle
    {
        // set the PENDING_REQUESTS before
        PENDING_REQUESTS.save(
            deps.as_mut().storage,
            0,
            &Request {
                id: 0,
                emergency_claimer: claimer.clone(),
                coin: Coin{denom: String::from("uguu"), amount: Uint128::from(100 as u32)},
            },
        ).unwrap();

        let msg = IBCLifecycleComplete::IBCAck {
            channel: String::from("channel-random"),
            sequence: 0,
            ack: String::from("ack"),
            success: true,
        };

        let res = ibc_lifecycle_complete(
            deps.as_mut(),
            mock_env(),
            msg,
        )
            .unwrap();

        assert_eq!(0, res.messages.len());

        let err = PENDING_REQUESTS.load(deps.as_ref().storage, 0);
        assert!(err.is_err());
    }

    // Failure case of IBC lifecycle
    {
        // set the PENDING_REQUESTS before
        PENDING_REQUESTS.save(
            deps.as_mut().storage,
            0,
            &Request {
                id: 0,
                emergency_claimer: claimer.clone(),
                coin: Coin{denom: String::from("uguu"), amount: Uint128::from(100 as u32)},
            },
        ).unwrap();

        let msg = IBCLifecycleComplete::IBCAck {
            channel: String::from("channel-random"),
            sequence: 0,
            ack: String::from("ack"),
            success: false,
        };

        let res = ibc_lifecycle_complete(
            deps.as_mut(),
            mock_env(),
            msg,
        )
            .unwrap();

        assert_eq!(0, res.messages.len());

        let err = PENDING_REQUESTS.load(deps.as_ref().storage, 0);
        assert!(err.is_err());
        let failed_requests = FAILED_REQUESTS.load(deps.as_ref().storage, (&claimer, 0)).unwrap();
        assert_eq!(failed_requests.id, 0);
    }

    // Timeout case of IBC lifecycle
    {
        // set the PENDING_REQUESTS before
        PENDING_REQUESTS.save(
            deps.as_mut().storage,
            0,
            &Request {
                id: 0,
                emergency_claimer: claimer.clone(),
                coin: Coin{denom: String::from("uguu"), amount: Uint128::from(100 as u32)},
            },
        ).unwrap();

        let msg = IBCLifecycleComplete::IBCTimeout {
            channel: String::from("channel-random"),
            sequence: 0,
        };

        let res = ibc_lifecycle_complete(
            deps.as_mut(),
            mock_env(),
            msg,
        )
            .unwrap();

        assert_eq!(0, res.messages.len());

        let err = PENDING_REQUESTS.load(deps.as_ref().storage, 0);
        assert!(err.is_err());
        let failed_requests = FAILED_REQUESTS.load(deps.as_ref().storage, (&claimer, 0)).unwrap();
        assert_eq!(failed_requests.id, 0);
    }
}
