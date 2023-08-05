use crate::{
    error::ContractError,
    ibc_hooks::IBCLifecycleComplete,
    state::{FAILED_REQUESTS, PENDING_REQUESTS},
};
use cosmwasm_std::{DepsMut, Env, Response};

#[cfg(not(feature = "library"))]
pub fn ibc_lifecycle_complete(
    deps: DepsMut,
    _env: Env,
    msg: IBCLifecycleComplete,
) -> Result<Response, ContractError> {
    match msg {
        IBCLifecycleComplete::IBCAck {
            channel: _,
            sequence,
            ack: _,
            success: true,
        } => {
            PENDING_REQUESTS.remove(deps.storage, sequence);
        }
        IBCLifecycleComplete::IBCAck {
            channel: _,
            sequence,
            ack: _,
            success: false,
        }
        | IBCLifecycleComplete::IBCTimeout {
            channel: _,
            sequence,
        } => {
            let request = PENDING_REQUESTS.load(deps.storage, sequence)?;

            FAILED_REQUESTS.save(
                deps.storage,
                (&request.emergency_claimer, sequence),
                &request,
            )?;
            PENDING_REQUESTS.remove(deps.storage, sequence);
        }
    }

    Ok(Response::new())
}
