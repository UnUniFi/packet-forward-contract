# DEX Aggregator Spec

## Architecture

- DEX Aggregator contract is deployed in each chain like UnUniFi, Osmosis, Terra, Sei, Juno, Secret and so on.
- DEX Aggregator contract in A chain interacts with another DEX Aggregator contract on B chain.
  - DEX Aggregator on B chain relays the function call from DEX Aggregator on A chain to DEX Adapter on B chain.
- DEX Adapter is a contract for wrapping DEX to unify the interface of DEX functionalities.
