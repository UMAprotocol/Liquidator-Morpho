// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity 0.8.25;

import {IERC4626} from "morpho-blue-oracles/morpho-chainlink/libraries/VaultLib.sol";
import {AggregatorV3Interface} from "morpho-blue-oracles/morpho-chainlink/libraries/ChainlinkDataFeedLib.sol";

interface IOracle {
    function price() external view returns (uint256);

    function SCALE_FACTOR() external view returns (uint256);

    function BASE_VAULT() external view returns (IERC4626);

    function BASE_VAULT_CONVERSION_SAMPLE() external view returns (uint256);

    function QUOTE_VAULT() external view returns (IERC4626);

    function QUOTE_VAULT_CONVERSION_SAMPLE() external view returns (uint256);

    function BASE_FEED_1() external view returns (AggregatorV3Interface);

    function BASE_FEED_2() external view returns (AggregatorV3Interface);

    function QUOTE_FEED_1() external view returns (AggregatorV3Interface);

    function QUOTE_FEED_2() external view returns (AggregatorV3Interface);
}
