// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.25;

/// @dev Includes only the methods needed for UnlockedOvalOracle
interface IAggregatorV3 {
    function decimals() external view returns (uint8);

    function latestRoundData()
        external
        view
        returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound);
}
