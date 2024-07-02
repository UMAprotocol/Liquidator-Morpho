// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.25;

import {IERC4626, VaultLib} from "morpho-blue-oracles/morpho-chainlink/libraries/VaultLib.sol";
import {AggregatorV3Interface} from "morpho-blue-oracles/morpho-chainlink/libraries/ChainlinkDataFeedLib.sol";

import {IOracle} from "./IOracle.sol";

/// @notice Contract to return unlocked price data, only for off-chain usage in a liquidator bot
contract UnlockedOvalOracle {
    using VaultLib for IERC4626;

    function getUnlockedPrice(IOracle oracle) external view returns (uint256) {
        IERC4626 baseVault = oracle.BASE_VAULT();
        uint256 baseVaultConversionSample = oracle.BASE_VAULT_CONVERSION_SAMPLE();
        IERC4626 quoteVault = oracle.QUOTE_VAULT();
        uint256 quoteVaultConversionSample = oracle.QUOTE_VAULT_CONVERSION_SAMPLE();
        AggregatorV3Interface baseFeed1 = oracle.BASE_FEED_1();
        AggregatorV3Interface baseFeed2 = oracle.BASE_FEED_2();
        AggregatorV3Interface quoteFeed1 = oracle.QUOTE_FEED_1();
        AggregatorV3Interface quoteFeed2 = oracle.QUOTE_FEED_2();
        uint256 scaleFactor = oracle.SCALE_FACTOR();

        return oracle.price();
    }
}
