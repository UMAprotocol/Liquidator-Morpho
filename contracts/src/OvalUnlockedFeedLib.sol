// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.25;

import {IChainlinkSourceAdapter} from "./IChainlinkSourceAdapter.sol";
import {Math} from "openzeppelin-contracts/contracts/utils/math/Math.sol";
import {
    AggregatorV3Interface,
    ChainlinkDataFeedLib
} from "morpho-blue-oracles/morpho-chainlink/libraries/ChainlinkDataFeedLib.sol";

/// @title OvalUnlockedFeedLib
/// @notice Library exposing unlocked price from Oval Oracle
library OvalUnlockedFeedLib {
    using ChainlinkDataFeedLib for AggregatorV3Interface;

    function tryGetSourceOracle(AggregatorV3Interface feed) internal view returns (AggregatorV3Interface) {
        if (address(feed) == address(0)) return feed;

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
        return convertDecimals(source.getPrice(), sourceDecimals, feedDecimals);
    }

    function convertDecimals(uint256 answer, uint8 iDecimals, uint8 oDecimals) internal pure returns (uint256) {
        if (iDecimals == oDecimals) return answer;
        if (iDecimals < oDecimals) return answer * 10 ** (oDecimals - iDecimals);
        return answer / 10 ** (iDecimals - oDecimals);
    }
}
