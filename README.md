# Packet Forward Contract

## The basic idea

There is already a [Packet Forward Middleware from StrangeLove](https://github.com/strangelove-ventures/packet-forward-middleware)
([Another link](https://github.com/cosmos/ibc-apps/tree/main/middleware/packet-forward-middleware)).

This enables the forwarding of ICS20 packets from one chain to another.

However, this middleware is not a smart contract, so it imposes chain developers to integrate it into their chain.

We developed a way of doing this in a smart contract, so that it can be deployed on any chain that supports CosmWasm.

## How it works

[IBC Hooks from Osmosis](https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks)
([Another link](https://github.com/cosmos/ibc-apps/tree/main/modules/ibc-hooks)) is the key to this contract.

This is an example of ICS20 packet data, quoted from Osmosis' repo.

```json
{
    //... other ibc fields that we don't care about
    "data":{
        "denom": "denom on counterparty chain (e.g. uatom)",  // will be transformed to the local denom (ibc/...)
        "amount": "1000",
        "sender": "addr on counterparty chain", // will be transformed
        "receiver": "contract addr or blank",
        "memo": {
           "wasm": {
              "contract": "osmo1contractAddr",
              "msg": {
                "raw_message_fields": "raw_message_data",
              }
            }
        }
    }
}
```

There is a `memo` field in the packet data.

The Packet Forward Middleware also utilizes this field, by detecting `forward` field in the `memo` object.

In the IBC Hooks module, the CosmWasm message written in the `memo` field will be executed by the contract specified in `contract` field.

This Packet Forward Contract can be designated as the contract to be executed here, to forward ICS20 packets.

The msg to be executed is defined as follows.

```rust
pub struct ForwardMsg {
    pub emergency_claimer: String,
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: Duration,
    pub memo: String,
}
```

It also contains a `memo` field recursively as well as the Packet Forward Middleware, so that it can be used in the same way to forward packets through chains more than two.

[IBC Denom Resolver](./contracts/ibc-denom-resolver/README.md) recursively designates the route of the packet forwarding, and enables us to resolve ibc denoms.

For example, "ATOM on UnUniFi from Cosmos Hub" can be converted to "ATOM on UnUniFi from Osmosis from Cosmos Hub" and vice versa.

See also README [here](./contracts/packet-forward/README.md).

## Appreciations

- <https://github.com/CosmWasm/cw-template>
