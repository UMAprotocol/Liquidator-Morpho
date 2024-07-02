// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.25;

import {IAggregatorV3} from "./IAggregatorV3.sol";
import {IOracle} from "./IOracle.sol";

/// @notice Contract to return unlocked price data, only for off-chain usage in a liquidator bot
contract UnlockedOvalOracle {
    function getUnlockedPrice(IOracle oracle) external view returns (uint256) {
        uint256 scaleFactor = oracle.SCALE_FACTOR();
        return oracle.price();
    }
}