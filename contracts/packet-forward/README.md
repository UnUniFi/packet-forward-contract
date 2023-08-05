# Packet Forward

## Prerequisite knowledge

- IBC Hooks
  - <https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks>
- IBC Hooks Acknowledge Callback
  - <https://github.com/cosmos/ibc-apps/tree/main/modules/ibc-hooks>

## State transitions for each request

- After `ForwardMsg` is executed, the request will be saved in the contract storage `INITIATED_REQUESTS`.
- After `MsgTransferResponse` is retrieved by the `Reply` of `SubMsg`:
  - If the reply means `ok`:
    - The request will be saved in the contract storage `PENDING_REQUESTS`.
    - The request will be removed from the contract storage `INITIATED_REQUESTS`.
  - If the reply means `err`:
    - The request will be removed from the contract storage `INITIATED_REQUESTS`.
- After `AcknowledgePacket` is retrieved by the `SudoMsg` of IBC Hooks:
  - If the packet means `success: true`:
    - The request will be removed from the contract storage `PENDING_REQUESTS`.
  - If the packet means `success: false`:
    - The request will be saved in the contract storage `FAILED_REQUESTS`.
    - The request will be removed from the contract storage `PENDING_REQUESTS`.

The owner of the address `emergency_claimer` can claim the tokens of requests that are in `FAILED_REQUESTS`.
