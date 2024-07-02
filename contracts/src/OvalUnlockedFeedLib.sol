// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.25;

import {IChainlinkSourceAdapter} from "./IChainlinkSourceAdapter.sol";
import {
    AggregatorV3Interface,
    ChainlinkDataFeedLib
} from "morpho-blue-oracles/morpho-chainlink/libraries/ChainlinkDataFeedLib.sol";

/// @title OvalUnlockedFeedLib
/// @notice Library exposing unlocked price from Oval Oracle
library OvalUnlockedFeedLib {
    using ChainlinkDataFeedLib for AggregatorV3Interface;

    function tryGetSourceOracle(AggregatorV3Interface feed) internal view returns (AggregatorV3Interface) {
        try IChainlinkSourceAdapter(address(feed)).CHAINLINK_SOURCE() returns (AggregatorV3Interface source) {
            return source;
        } catch {
            return feed;
        }
    }

    function getUnlockedPrice(AggregatorV3Interface feed) internal view returns (uint256) {
        AggregatorV3Interface source = tryGetSourceOracle(feed);

        if (source == feed) {
            return feed.getPrice();
        }

        uint8 sourceDecimals = source.decimals();
        uint8 feedDecimals = feed.decimals();
        return source.getPrice();
    }
}
