
---

# Hydra DX audit details
- Total Prize Pool: $100,000 in USDC 
  - HM awards: $78,162.50 in USDC 
  - Analysis awards: $4,225 in USDC 
  - QA awards: $2,112.50 in USDC
  - Judge awards: $9,000 in USDC 
  - Lookout awards: $6,000 in USDC 
  - Scout awards: $500 in USDC 
- Join [C4 Discord](https://discord.gg/code4rena) to register
- Submit findings [using the C4 form](https://code4rena.com/contests/2024-02-hydradx/submit)
- [Read our guidelines for more details](https://docs.code4rena.com/roles/wardens)
- Starts February 02, 2024 20:00 UTC 
- Ends March 1, 2024 20:00 UTC 

## Automated Findings / Publicly Known Issues
_Note for C4 wardens: Anything included in this `Automated Findings / Publicly Known Issues` section is considered a publicly known issue and is ineligible for awards._

# Overview

## HydraDX
HydraDX is a next-gen DeFi protocol which is designed to bring an ocean of liquidity to Polkadot. Our tool for the job the HydraDX Omnipool - an innovative Automated Market Maker (AMM) which unlocks unparalleled efficiencies by combining all assets in a single trading pool.

## Main features in the audit scope
- Omnipool
  - Omnipool is type of AMM where all assets are pooled together into one single pool.
- Stableswap
  - Curve style AMM, designed to provide highly efficient and low-slippage trades for stablecoins.
- Oracle
  - This pallet provides exponential moving average (EMA) oracles of different periods for price, volume and liquidity for a combination of source and asset pair based on data coming in from different sources.
- Circuit breaker
  - Provides tracking and limiting the percentage of the liquidity of a pool that can be traded (net volume), added or removed in a single block.


## Links

- **Previous audits:**  Details can be found [here](https://github.com/galacticcouncil/HydraDX-security/)
- **Documentation:** [HydraDX Docs](https://docs.hydradx.io/)
- **Website:** [hydradx.io](https://hydradx.io/)
- **Twitter:** [hydra_dx](https://twitter.com/hydra_dx)
- **Discord:** 


# Scope

| Contract                                                                                                                                   | SLOC | Purpose                                | Libraries used |  
|--------------------------------------------------------------------------------------------------------------------------------------------|------|----------------------------------------|----------------|
| [Omnipool](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/pallets/omnipool)                                          |      | Omnipool pallet                        | |
| [omnipool/src/lib.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/omnipool/src/lib.rs)               | 1367 | Omnipool pallet - main pallet's file   |                |
| [omnipool/src/types.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/omnipool/src/types.rs)           | 233  | Omnipool pallet - types                |                |
| [omnipool/src/traits.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/omnipool/src/traits.rs)         | 162  | Omnipool pallet - traits               |                |
| [Omnipool Math](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/math/src/omnipool)                                    |      | Omnipool math                          |                |
| [math/src/omnipool/math.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/math/src/omnipool/math.rs)           | 409  | Omnipool math - math implementation    |                |
| [math/src/omnipool/types.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/math/src/omnipool/types.rs)         | 226  | Omnipool math - types                  |                |
| [Stableswap](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/pallets/stableswap)                                      |      | Stableswap pallet                      |                |
| [stableswap/src/lib.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/stableswap/src/lib.rs)           | 871  | Stableswap pallet - main pallet's file |                |
| [stableswap/src/types.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/stableswap/src/types.rs)       | 136  | Stableswap pallet - types              |                |
| [Stableswap Math](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/math/src/stableswap)                                |      | Stableswap Math                        |                |
| [math/src/stableswap/math.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/math/src/stableswap/math.rs)       | 670  | Stableswap Math - math implementation  |                |
| [math/src/stableswap/types.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/math/src/stableswap/types.rs)     | 25   | Stableswap Math - types                |                |
| [EMA Oracle](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/pallets/ema-oracle)                                      |      | Ema on-chain oracle                    |                |
| [ema-oracle/src/lib.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/ema-oracle/src/lib.rs)           | 395  | Ema oracle pallet - main pallet's file |                |
| [ema-oracle/src/types.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/ema-oracle/src/types.rs)       | 154  | Ema oracle pallet - types              |                |
| [Ema Oracle Math](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/math/src/ema)                                       |      | Omnipool math                          |                |
| [math/src/ema/math.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/math/src/ema/math.rs)                     | 174  | Omnipool math - math implementation    |                |
| [Circuit breaker](https://github.com/code-423n4/2024-02-hydradx/tree/main/HydraDX-node/pallets/circuit-breaker)                            |      | Circuit breaker                        |                |
| [circuit-breaker/src/lib.rs](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/pallets/circuit-breaker/src/lib.rs) | 451  | Circuit breaker- main pallet's file    |                |

Total SLOC: 5273

## Out of scope

Only files listed above are in scope. Everything else is out of scope.

# Additional Context
Refer to documentation in each pallet for further details.


## Runtime configuration
Note that each pallet is integrated into HydraDX runtime and can interact with other pallets. 
Each pallet is configured in runtime and this configuration should be taken into account.

Wardens can assume that runtime is configured correctly and that pallets are configured correctly.

Example:
Omnipool has `AuthorityOrigin` parameter which allows only configured origin to perform certain actions.
That means reports such as `if origin is not configured correctly, it can lead to ...` are not valid.

However, findings on possible misconfiguration of pallets in scope will be considered.


## Attack ideas (Where to look for bugs)
Refer to HydraDX security repository for possible attack vectors [Here](https://github.com/galacticcouncil/HydraDX-security/blob/main/threat_modelling.md)

## Main invariants
Refer to HydraDX security repository that describes omnipool's and stableswap invariants [Here](https://github.com/galacticcouncil/HydraDX-security/tree/main/invariants)

# Running a local test node
Refer to HydraDX-node [readme](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/README.md) for details.

# Tests

## Setting up Rust/Substrate environment
Details in [readme](https://github.com/code-423n4/2024-02-hydradx/blob/tree/main/HydraDX-node/README.md)

## Getting the code
Clone this repository
```bash
git clone https://github.com/code-423n4/2024-02-hydradx/
```

Enter into the directory
```bash
cd HydraDX-node
````

## Running pallet tests
Omnipool
```bash
cargo test -p pallet-omnipool 
```

Stableswap
```bash
cargo test -p pallet-stableswap 
```

EMA Oracle
```bash
cargo test -p pallet-ema-oracle 
```

Circuit breaker
```bash
cargo test -p pallet-circuit-breaker 
```

## Running math tests
You can focus on math for each pallet separately.

Omnipool math
```bash
cargo test omnipool -p hydra-dx-math
```

Stableswap math
```bash
cargo test stableswap -p hydra-dx-math
```

EMA Oracle math
```bash
cargo test ema -p hydra-dx-math
```

## Running integration tests
These tests focus on integration of a pallet in HydraDX runtime, interactions with other pallets and configuration.
```bash
cargo test -p runtime-integration-tests
```


## Miscellaneous

Employees of HydraDX and employees' family members are ineligible to participate in this audit.
