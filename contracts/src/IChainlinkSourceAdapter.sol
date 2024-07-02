// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.25;

import {AggregatorV3Interface} from "morpho-blue-oracles/morpho-chainlink/libraries/ChainlinkDataFeedLib.sol";

interface IChainlinkSourceAdapter {
    function CHAINLINK_SOURCE() external view returns (AggregatorV3Interface);
}
