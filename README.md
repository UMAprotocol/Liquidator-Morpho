# Morpho Liquidator
![Rust](https://github.com/etherhood/Liquidator-Morpho/workflows/Rust/badge.svg)
![Solidity](https://github.com/etherhood/Liquidator-Morpho/workflows/Solidity/badge.svg)

## App
`app/` folder contains rust repo which index all events from morpho contracts and write it to local morpho.json file and then start listening to new events and new block. Whenever it encounters new positions which are unhealthy, it triggers liquidation for that position using `Liquidator` contract 

## Instructions
- Install dependencies with `make setup`
- Build the project with `make build-release`
- Deploy `contracts/src/UnlockedOvalOracle.sol`
- Deploy `contracts/src/Liquidator.sol`
- Fund the deployed `Liquidator` contract with ETH to cover expected builder payments
- Create `.env` file using `.env.example` as reference and fill in the required values
- Run the bot with `cargo run --release` (export `RUST_LOG` to required log level)
- Monitor Liquidator contract balances and manually swap accrued gross profits (ERC-20 debt tokens) into ETH that has been consumed in builder payments and gas

# Todo
- [x] Abstract out constants to `env` args which can be passed at start time
- [x] Integrate 1inch or other solver APIs to fetch swap route when liquidating
- [x] Add signer support for provider which send liquidator transaction
- [ ] Add flashbots support to not allow frontrunning
- [ ] Add support for minimum value of collateral to filter liquidation opportunities
- [ ] Add batching of liquidation transaction at end of block processing
- [ ] Add gas fee calculation to ascertain profitability
- [ ] Add transaction caching to not send multiple transactions for same position

This is not yet ready for production use. PRs welcome.