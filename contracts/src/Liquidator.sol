// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity 0.8.25;

import {IMorphoLiquidateCallback} from "./MorphoCallbacks.sol";
import {IMorpho, MarketParams} from "./IMorpho.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import {Address} from "openzeppelin-contracts/contracts/utils/Address.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/// @author etherhood
/// @notice Liquidator contract which sells collateral for debt tokens
/// @notice Tokens should never be left over in this contract
contract Liquidator is IMorphoLiquidateCallback, Ownable {
    using SafeERC20 for IERC20;
    using Address for address;
    using Address for address payable;

    struct CallbackParams {
        address swapper;
        address collateral;
        address debt;
        bytes swapData;
    }

    struct LiquidationParams {
        uint256 debtQuote;
        uint256 builderPaymentPercent;
        address swapper;
        bytes swapData;
    }

    IMorpho internal constant morpho = IMorpho(0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb);

    constructor() Ownable(msg.sender) {}

    function liquidateUser(
        MarketParams memory marketParams,
        address user,
        uint256 seizedAssets,
        LiquidationParams memory liquidationParams
    ) external onlyOwner {
        IERC20 debt = IERC20(marketParams.loanToken);
        uint256 startBalance = debt.balanceOf(address(this));

        morpho.liquidate(
            marketParams,
            user,
            seizedAssets,
            0,
            abi.encode(
                CallbackParams(
                    liquidationParams.swapper,
                    marketParams.collateralToken,
                    marketParams.loanToken,
                    liquidationParams.swapData
                )
            )
        );

        uint256 endBalance = debt.balanceOf(address(this));
        require(endBalance > startBalance, "Liquidator: Not profitable");

        uint256 grossProfit = endBalance - startBalance;
        uint256 builderPayment =
            grossProfit * liquidationParams.debtQuote / 1e18 * liquidationParams.builderPaymentPercent / 1e18;
        block.coinbase.sendValue(builderPayment);
    }

    function onMorphoLiquidate(uint256 repaidAssets, bytes calldata data) external {
        require(address(morpho) == msg.sender, "Liquidator: Invalid address");

        CallbackParams memory callbackParams = abi.decode(data, (CallbackParams));

        uint256 collateralSeized = IERC20(callbackParams.collateral).balanceOf(address(this));

        IERC20(callbackParams.collateral).safeIncreaseAllowance(callbackParams.swapper, collateralSeized);

        callbackParams.swapper.functionCall(callbackParams.swapData);

        IERC20(callbackParams.debt).safeIncreaseAllowance(msg.sender, repaidAssets);
    }

    function swapProfit(address token, uint256 amount, address swapper, bytes calldata swapData) external onlyOwner {
        IERC20(token).safeIncreaseAllowance(swapper, amount);

        swapper.functionCall(swapData);
    }

    receive() external payable {}
}
