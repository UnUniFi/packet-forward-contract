# IBC Denom Resolver

## Config

`denom` and `routes` will be fixed for each instantiated contract.

```rust
pub struct Config {
    pub owner: Addr,
    pub denom: String,
    pub routes: Vec<Route>,
    pub treasury: Addr,
    pub fee: FeeConfig,
    pub timeout: Duration,
}
```

```rust
pub enum Destination {
    Terminal,
    PacketForwardMiddleware,
    PacketForwardContract { address: String },
}

pub struct Route {
    pub src_port: String,
    pub src_channel: String,
    pub destination: Destination,
}
```

## SwapMsg

```rust
pub struct SwapMsg {
    pub receivers: Vec<String>,
}
```

- `receivers` must have same length to `routes` in `Config`.
  - They will be the `emergency_claimer` for each route if the packet is failed on the way.
