pub use liquidator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod liquidator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_morpho"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IMorpho"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("liquidateUser"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidateUser"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("seizedAssets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Liquidator.LiquidationParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onMorphoLiquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onMorphoLiquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repaidAssets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@Rs\xBB\xBB\xBB\xBB\xBB\x9C\xC5\xE9\x0E;:\xF6K\xDA\xF6,7\xEE\xFF\xCB`\x80R4\x80\x15a\0'W_\x80\xFD[P`@Qa\x12i8\x03\x80a\x12i\x839\x81\x01`@\x81\x90Ra\0F\x91a\0\xE7V[3\x80a\0kW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0t\x81a\0\x98V[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\x92W`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R[Pa\x01\x14V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_` \x82\x84\x03\x12\x15a\0\xF7W_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\rW_\x80\xFD[\x93\x92PPPV[`\x80Qa\x116a\x013_9_\x81\x81a\x01\xF7\x01Ra\x04\xBC\x01Ra\x116_\xF3\xFE`\x80`@R`\x046\x10a\0|W_5`\xE0\x1C\x80c\xA1\xDB\x97\x82\x11a\0LW\x80c\xA1\xDB\x97\x82\x14a\x01\x05W\x80c\xCF~\xA1\x96\x14a\x01$W\x80c\xF1B\x10\xA6\x14a\x01CW\x80c\xF2\xFD\xE3\x8B\x14a\x01bW_\x80\xFD[\x80cM\x0E\xC7\x05\x14a\0\x87W\x80cYT?\xA6\x14a\0\xA8W\x80cqP\x18\xA6\x14a\0\xC7W\x80c\x8D\xA5\xCB[\x14a\0\xDBW_\x80\xFD[6a\0\x83W\0[_\x80\xFD[4\x80\x15a\0\x92W_\x80\xFD[Pa\0\xA6a\0\xA16`\x04a\x0C\x8EV[a\x01\x81V[\0[4\x80\x15a\0\xB3W_\x80\xFD[Pa\0\xA6a\0\xC26`\x04a\r\x8EV[a\x04\x1AV[4\x80\x15a\0\xD2W_\x80\xFD[Pa\0\xA6a\x04\x87V[4\x80\x15a\0\xE6W_\x80\xFD[P_T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x01\x10W_\x80\xFD[Pa\0\xA6a\x01\x1F6`\x04a\r\xF8V[a\x04\x9AV[4\x80\x15a\x01/W_\x80\xFD[Pa\0\xA6a\x01>6`\x04a\x0E V[a\x04\xBAV[4\x80\x15a\x01NW_\x80\xFD[Pa\0\xA6a\x01]6`\x04a\x0EhV[a\x06\x0BV[4\x80\x15a\x01mW_\x80\xFD[Pa\0\xA6a\x01|6`\x04a\x0E\x7FV[a\x06 V[a\x01\x89a\x06ZV[\x83Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xCFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF3\x91\x90a\x0E\x98V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD8\xEA\xBC\xB8\x87\x87\x87_`@Q\x80`\x80\x01`@R\x80\x8A`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A``\x01Q\x81RP`@Q` \x01a\x02\x83\x91\x90a\x0E\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xB2\x95\x94\x93\x92\x91\x90a\x0F+V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xF1\x91\x90a\x0F\x9EV[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x037W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03[\x91\x90a\x0E\x98V[\x90P\x81\x81\x11a\x03\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FLiquidator: Not profitable\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x03\xBC\x83\x83a\x0F\xD4V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0\x86` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0\x88_\x01Q\x85a\x03\xE5\x91\x90a\x0F\xE7V[a\x03\xEF\x91\x90a\x0F\xFEV[a\x03\xF9\x91\x90a\x0F\xE7V[a\x04\x03\x91\x90a\x0F\xFEV[\x90Pa\x04\x0FA\x82a\x06\x86V[PPPPPPPPPV[a\x04\"a\x06ZV[a\x046`\x01`\x01`\xA0\x1B\x03\x86\x16\x84\x86a\x07\x1EV[a\x04\x7F\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x01`\x01`\xA0\x1B\x03\x87\x16\x92\x91PPa\x07\xABV[PPPPPPV[a\x04\x8Fa\x06ZV[a\x04\x98_a\x07\xC1V[V[a\x04\xA2a\x06ZV[a\x04\xB6`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x08\x10V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x14a\x052W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FLiquidator: Invalid address\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xA8V[_a\x05?\x82\x84\x01\x84a\x10\x1DV[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P_\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xB0\x91\x90a\x0E\x98V[\x82Q` \x84\x01Q\x91\x92Pa\x05\xCE\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a\x07\x1EV[``\x82\x01Q\x82Qa\x05\xEA\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x07\xABV[P`@\x82\x01Qa\x06\x04\x90`\x01`\x01`\xA0\x1B\x03\x163\x87a\x07\x1EV[PPPPPV[a\x06\x13a\x06ZV[a\x06\x1D3\x82a\x06\x86V[PV[a\x06(a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06QW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\xA8V[a\x06\x1D\x81a\x07\xC1V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\x98W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x03\xA8V[\x80G\x10\x15a\x06\xA9W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x03\xA8V[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x06\xF2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\xF7V[``\x91P[PP\x90P\x80a\x07\x19W`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8F\x91\x90a\x0E\x98V[\x90Pa\x07\xA5\x84\x84a\x07\xA0\x85\x85a\x10\xB8V[a\x08oV[PPPPV[``a\x07\xB8\x83\x83_a\x08\xFEV[\x90P[\x92\x91PPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\x07\x19\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\t\x99V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08\xC0\x84\x82a\t\xFAV[a\x07\xA5W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R_`D\x83\x01Ra\x08\xF4\x91\x86\x91\x82\x16\x90c\t^\xA7\xB3\x90`d\x01a\x08=V[a\x07\xA5\x84\x82a\t\x99V[``\x81G\x10\x15a\t#W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x03\xA8V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\t>\x91\x90a\x10\xCBV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\txW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\t}V[``\x91P[P\x91P\x91Pa\t\x8D\x86\x83\x83a\n\x9BV[\x92PPP[\x93\x92PPPV[_a\t\xAD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x07\xABV[\x90P\x80Q_\x14\x15\x80\x15a\t\xD1WP\x80\x80` \x01\x90Q\x81\x01\x90a\t\xCF\x91\x90a\x10\xE1V[\x15[\x15a\x07\x19W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x03\xA8V[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\n\x15\x91\x90a\x10\xCBV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\nNW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\nSV[``\x91P[P\x91P\x91P\x81\x80\x15a\n}WP\x80Q\x15\x80a\n}WP\x80\x80` \x01\x90Q\x81\x01\x90a\n}\x91\x90a\x10\xE1V[\x80\x15a\n\x92WP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[``\x82a\n\xB0Wa\n\xAB\x82a\n\xF7V[a\t\x92V[\x81Q\x15\x80\x15a\n\xC7WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n\xF0W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\xA8V[P\x80a\t\x92V[\x80Q\x15a\x0B\x07W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BWWa\x0BWa\x0B V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BWWa\x0BWa\x0B V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x96W_\x80\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x0B\xAAW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xC5Wa\x0B\xC5a\x0B V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xEDWa\x0B\xEDa\x0B V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\x05W_\x80\xFD[\x83` \x87\x01` \x83\x017_` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a\x0C4W_\x80\xFD[a\x0C<a\x0B4V[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x0CX`@\x83\x01a\x0B\x80V[`@\x82\x01R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CvW_\x80\xFD[a\x0C\x82\x84\x82\x85\x01a\x0B\x9BV[``\x83\x01RP\x92\x91PPV[_\x80_\x80\x84\x86\x03a\x01\0\x81\x12\x15a\x0C\xA3W_\x80\xFD[`\xA0\x81\x12\x15a\x0C\xB0W_\x80\xFD[Pa\x0C\xB9a\x0B]V[a\x0C\xC2\x86a\x0B\x80V[\x81Ra\x0C\xD0` \x87\x01a\x0B\x80V[` \x82\x01Ra\x0C\xE1`@\x87\x01a\x0B\x80V[`@\x82\x01Ra\x0C\xF2``\x87\x01a\x0B\x80V[``\x82\x01R`\x80\x86\x81\x015\x90\x82\x01R\x93Pa\r\x0F`\xA0\x86\x01a\x0B\x80V[\x92P`\xC0\x85\x015\x91P`\xE0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r1W_\x80\xFD[a\r=\x87\x82\x88\x01a\x0C$V[\x91PP\x92\x95\x91\x94P\x92PV[_\x80\x83`\x1F\x84\x01\x12a\rYW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rpW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x87W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a\r\xA2W_\x80\xFD[a\r\xAB\x86a\x0B\x80V[\x94P` \x86\x015\x93Pa\r\xC0`@\x87\x01a\x0B\x80V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xDBW_\x80\xFD[a\r\xE7\x88\x82\x89\x01a\rIV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x0E\tW_\x80\xFD[a\x0E\x12\x83a\x0B\x80V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_`@\x84\x86\x03\x12\x15a\x0E2W_\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EOW_\x80\xFD[a\x0E[\x86\x82\x87\x01a\rIV[\x94\x97\x90\x96P\x93\x94PPPPV[_` \x82\x84\x03\x12\x15a\x0ExW_\x80\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0E\x8FW_\x80\xFD[a\x07\xB8\x82a\x0B\x80V[_` \x82\x84\x03\x12\x15a\x0E\xA8W_\x80\xFD[PQ\x91\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_`\x01\x80`\xA0\x1B\x03\x80\x84Q\x16` \x84\x01R\x80` \x85\x01Q\x16`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Q`\x80\x80\x84\x01Ra\x0F#`\xA0\x84\x01\x82a\x0E\xAFV[\x94\x93PPPPV[_a\x01 `\x01\x80`\xA0\x1B\x03\x80\x89Q\x16\x84R\x80` \x8A\x01Q\x16` \x85\x01R\x80`@\x8A\x01Q\x16`@\x85\x01R\x80``\x8A\x01Q\x16``\x85\x01R`\x80\x89\x01Q`\x80\x85\x01R\x80\x88\x16`\xA0\x85\x01RP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra\x0F\x92\x81\x84\x01\x85a\x0E\xAFV[\x98\x97PPPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x0F\xAFW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07\xBBWa\x07\xBBa\x0F\xC0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07\xBBWa\x07\xBBa\x0F\xC0V[_\x82a\x10\x18WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x10-W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10DW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15a\x10WW_\x80\xFD[a\x10_a\x0B4V[a\x10h\x83a\x0B\x80V[\x81Ra\x10v` \x84\x01a\x0B\x80V[` \x82\x01Ra\x10\x87`@\x84\x01a\x0B\x80V[`@\x82\x01R``\x83\x015\x82\x81\x11\x15a\x10\x9DW_\x80\xFD[a\x10\xA9\x87\x82\x86\x01a\x0B\x9BV[``\x83\x01RP\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\xBBWa\x07\xBBa\x0F\xC0V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x10\xF1W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\t\x92W_\x80\xFD\xFE\xA2dipfsX\"\x12 \xDF\x04\xE1\xC4\xE1\xE3\xE1\xAA\x8D\x04\x82\x17\x89E!\xAD\xCF>L\xD4\xFEOe\x06\xA2\x8D\xAE+;\x14{odsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0|W_5`\xE0\x1C\x80c\xA1\xDB\x97\x82\x11a\0LW\x80c\xA1\xDB\x97\x82\x14a\x01\x05W\x80c\xCF~\xA1\x96\x14a\x01$W\x80c\xF1B\x10\xA6\x14a\x01CW\x80c\xF2\xFD\xE3\x8B\x14a\x01bW_\x80\xFD[\x80cM\x0E\xC7\x05\x14a\0\x87W\x80cYT?\xA6\x14a\0\xA8W\x80cqP\x18\xA6\x14a\0\xC7W\x80c\x8D\xA5\xCB[\x14a\0\xDBW_\x80\xFD[6a\0\x83W\0[_\x80\xFD[4\x80\x15a\0\x92W_\x80\xFD[Pa\0\xA6a\0\xA16`\x04a\x0C\x8EV[a\x01\x81V[\0[4\x80\x15a\0\xB3W_\x80\xFD[Pa\0\xA6a\0\xC26`\x04a\r\x8EV[a\x04\x1AV[4\x80\x15a\0\xD2W_\x80\xFD[Pa\0\xA6a\x04\x87V[4\x80\x15a\0\xE6W_\x80\xFD[P_T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[4\x80\x15a\x01\x10W_\x80\xFD[Pa\0\xA6a\x01\x1F6`\x04a\r\xF8V[a\x04\x9AV[4\x80\x15a\x01/W_\x80\xFD[Pa\0\xA6a\x01>6`\x04a\x0E V[a\x04\xBAV[4\x80\x15a\x01NW_\x80\xFD[Pa\0\xA6a\x01]6`\x04a\x0EhV[a\x06\x0BV[4\x80\x15a\x01mW_\x80\xFD[Pa\0\xA6a\x01|6`\x04a\x0E\x7FV[a\x06 V[a\x01\x89a\x06ZV[\x83Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xCFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF3\x91\x90a\x0E\x98V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD8\xEA\xBC\xB8\x87\x87\x87_`@Q\x80`\x80\x01`@R\x80\x8A`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A``\x01Q\x81RP`@Q` \x01a\x02\x83\x91\x90a\x0E\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xB2\x95\x94\x93\x92\x91\x90a\x0F+V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xF1\x91\x90a\x0F\x9EV[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x037W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03[\x91\x90a\x0E\x98V[\x90P\x81\x81\x11a\x03\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FLiquidator: Not profitable\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x03\xBC\x83\x83a\x0F\xD4V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0\x86` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0\x88_\x01Q\x85a\x03\xE5\x91\x90a\x0F\xE7V[a\x03\xEF\x91\x90a\x0F\xFEV[a\x03\xF9\x91\x90a\x0F\xE7V[a\x04\x03\x91\x90a\x0F\xFEV[\x90Pa\x04\x0FA\x82a\x06\x86V[PPPPPPPPPV[a\x04\"a\x06ZV[a\x046`\x01`\x01`\xA0\x1B\x03\x86\x16\x84\x86a\x07\x1EV[a\x04\x7F\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x01`\x01`\xA0\x1B\x03\x87\x16\x92\x91PPa\x07\xABV[PPPPPPV[a\x04\x8Fa\x06ZV[a\x04\x98_a\x07\xC1V[V[a\x04\xA2a\x06ZV[a\x04\xB6`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x08\x10V[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x14a\x052W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FLiquidator: Invalid address\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xA8V[_a\x05?\x82\x84\x01\x84a\x10\x1DV[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P_\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xB0\x91\x90a\x0E\x98V[\x82Q` \x84\x01Q\x91\x92Pa\x05\xCE\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a\x07\x1EV[``\x82\x01Q\x82Qa\x05\xEA\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x07\xABV[P`@\x82\x01Qa\x06\x04\x90`\x01`\x01`\xA0\x1B\x03\x163\x87a\x07\x1EV[PPPPPV[a\x06\x13a\x06ZV[a\x06\x1D3\x82a\x06\x86V[PV[a\x06(a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06QW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\xA8V[a\x06\x1D\x81a\x07\xC1V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\x98W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x03\xA8V[\x80G\x10\x15a\x06\xA9W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x03\xA8V[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x06\xF2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\xF7V[``\x91P[PP\x90P\x80a\x07\x19W`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8F\x91\x90a\x0E\x98V[\x90Pa\x07\xA5\x84\x84a\x07\xA0\x85\x85a\x10\xB8V[a\x08oV[PPPPV[``a\x07\xB8\x83\x83_a\x08\xFEV[\x90P[\x92\x91PPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\x07\x19\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\t\x99V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08\xC0\x84\x82a\t\xFAV[a\x07\xA5W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R_`D\x83\x01Ra\x08\xF4\x91\x86\x91\x82\x16\x90c\t^\xA7\xB3\x90`d\x01a\x08=V[a\x07\xA5\x84\x82a\t\x99V[``\x81G\x10\x15a\t#W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x03\xA8V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\t>\x91\x90a\x10\xCBV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\txW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\t}V[``\x91P[P\x91P\x91Pa\t\x8D\x86\x83\x83a\n\x9BV[\x92PPP[\x93\x92PPPV[_a\t\xAD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x07\xABV[\x90P\x80Q_\x14\x15\x80\x15a\t\xD1WP\x80\x80` \x01\x90Q\x81\x01\x90a\t\xCF\x91\x90a\x10\xE1V[\x15[\x15a\x07\x19W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x03\xA8V[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\n\x15\x91\x90a\x10\xCBV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\nNW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\nSV[``\x91P[P\x91P\x91P\x81\x80\x15a\n}WP\x80Q\x15\x80a\n}WP\x80\x80` \x01\x90Q\x81\x01\x90a\n}\x91\x90a\x10\xE1V[\x80\x15a\n\x92WP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[``\x82a\n\xB0Wa\n\xAB\x82a\n\xF7V[a\t\x92V[\x81Q\x15\x80\x15a\n\xC7WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n\xF0W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\xA8V[P\x80a\t\x92V[\x80Q\x15a\x0B\x07W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BWWa\x0BWa\x0B V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BWWa\x0BWa\x0B V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x96W_\x80\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x0B\xAAW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xC5Wa\x0B\xC5a\x0B V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xEDWa\x0B\xEDa\x0B V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\x05W_\x80\xFD[\x83` \x87\x01` \x83\x017_` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a\x0C4W_\x80\xFD[a\x0C<a\x0B4V[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x0CX`@\x83\x01a\x0B\x80V[`@\x82\x01R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CvW_\x80\xFD[a\x0C\x82\x84\x82\x85\x01a\x0B\x9BV[``\x83\x01RP\x92\x91PPV[_\x80_\x80\x84\x86\x03a\x01\0\x81\x12\x15a\x0C\xA3W_\x80\xFD[`\xA0\x81\x12\x15a\x0C\xB0W_\x80\xFD[Pa\x0C\xB9a\x0B]V[a\x0C\xC2\x86a\x0B\x80V[\x81Ra\x0C\xD0` \x87\x01a\x0B\x80V[` \x82\x01Ra\x0C\xE1`@\x87\x01a\x0B\x80V[`@\x82\x01Ra\x0C\xF2``\x87\x01a\x0B\x80V[``\x82\x01R`\x80\x86\x81\x015\x90\x82\x01R\x93Pa\r\x0F`\xA0\x86\x01a\x0B\x80V[\x92P`\xC0\x85\x015\x91P`\xE0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r1W_\x80\xFD[a\r=\x87\x82\x88\x01a\x0C$V[\x91PP\x92\x95\x91\x94P\x92PV[_\x80\x83`\x1F\x84\x01\x12a\rYW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rpW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r\x87W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a\r\xA2W_\x80\xFD[a\r\xAB\x86a\x0B\x80V[\x94P` \x86\x015\x93Pa\r\xC0`@\x87\x01a\x0B\x80V[\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xDBW_\x80\xFD[a\r\xE7\x88\x82\x89\x01a\rIV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x0E\tW_\x80\xFD[a\x0E\x12\x83a\x0B\x80V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_`@\x84\x86\x03\x12\x15a\x0E2W_\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EOW_\x80\xFD[a\x0E[\x86\x82\x87\x01a\rIV[\x94\x97\x90\x96P\x93\x94PPPPV[_` \x82\x84\x03\x12\x15a\x0ExW_\x80\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x0E\x8FW_\x80\xFD[a\x07\xB8\x82a\x0B\x80V[_` \x82\x84\x03\x12\x15a\x0E\xA8W_\x80\xFD[PQ\x91\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_`\x01\x80`\xA0\x1B\x03\x80\x84Q\x16` \x84\x01R\x80` \x85\x01Q\x16`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Q`\x80\x80\x84\x01Ra\x0F#`\xA0\x84\x01\x82a\x0E\xAFV[\x94\x93PPPPV[_a\x01 `\x01\x80`\xA0\x1B\x03\x80\x89Q\x16\x84R\x80` \x8A\x01Q\x16` \x85\x01R\x80`@\x8A\x01Q\x16`@\x85\x01R\x80``\x8A\x01Q\x16``\x85\x01R`\x80\x89\x01Q`\x80\x85\x01R\x80\x88\x16`\xA0\x85\x01RP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra\x0F\x92\x81\x84\x01\x85a\x0E\xAFV[\x98\x97PPPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x0F\xAFW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07\xBBWa\x07\xBBa\x0F\xC0V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07\xBBWa\x07\xBBa\x0F\xC0V[_\x82a\x10\x18WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x10-W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10DW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15a\x10WW_\x80\xFD[a\x10_a\x0B4V[a\x10h\x83a\x0B\x80V[\x81Ra\x10v` \x84\x01a\x0B\x80V[` \x82\x01Ra\x10\x87`@\x84\x01a\x0B\x80V[`@\x82\x01R``\x83\x015\x82\x81\x11\x15a\x10\x9DW_\x80\xFD[a\x10\xA9\x87\x82\x86\x01a\x0B\x9BV[``\x83\x01RP\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\xBBWa\x07\xBBa\x0F\xC0V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x10\xF1W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\t\x92W_\x80\xFD\xFE\xA2dipfsX\"\x12 \xDF\x04\xE1\xC4\xE1\xE3\xE1\xAA\x8D\x04\x82\x17\x89E!\xAD\xCF>L\xD4\xFEOe\x06\xA2\x8D\xAE+;\x14{odsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `liquidateUser` (0x4d0ec705) function
        pub fn liquidate_user(
            &self,
            market_params: MarketParams,
            user: ::ethers::core::types::Address,
            seized_assets: ::ethers::core::types::U256,
            liquidation_params: LiquidationParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [77, 14, 199, 5],
                    (market_params, user, seized_assets, liquidation_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onMorphoLiquidate` (0xcf7ea196) function
        pub fn on_morpho_liquidate(
            &self,
            repaid_assets: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 126, 161, 150], (repaid_assets, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapERC20` (0x59543fa6) function
        pub fn swap_erc20(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
            swap_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 84, 63, 166], (token, amount, swapper, swap_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC20` (0xa1db9782) function
        pub fn withdraw_erc20(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 219, 151, 130], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawETH` (0xf14210a6) function
        pub fn withdraw_eth(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 66, 16, 166], amount)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LiquidatorErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LiquidatorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LiquidatorErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for LiquidatorErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LiquidatorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for LiquidatorErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for LiquidatorErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for LiquidatorErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidateUser` function with signature `liquidateUser((address,address,address,address,uint256),address,uint256,(uint256,uint256,address,bytes))` and selector `0x4d0ec705`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "liquidateUser",
        abi = "liquidateUser((address,address,address,address,uint256),address,uint256,(uint256,uint256,address,bytes))"
    )]
    pub struct LiquidateUserCall {
        pub market_params: MarketParams,
        pub user: ::ethers::core::types::Address,
        pub seized_assets: ::ethers::core::types::U256,
        pub liquidation_params: LiquidationParams,
    }
    ///Container type for all input parameters for the `onMorphoLiquidate` function with signature `onMorphoLiquidate(uint256,bytes)` and selector `0xcf7ea196`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "onMorphoLiquidate", abi = "onMorphoLiquidate(uint256,bytes)")]
    pub struct OnMorphoLiquidateCall {
        pub repaid_assets: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `swapERC20` function with signature `swapERC20(address,uint256,address,bytes)` and selector `0x59543fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swapERC20", abi = "swapERC20(address,uint256,address,bytes)")]
    pub struct SwapERC20Call {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
        pub swap_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawERC20` function with signature `withdrawERC20(address,uint256)` and selector `0xa1db9782`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawERC20", abi = "withdrawERC20(address,uint256)")]
    pub struct WithdrawERC20Call {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawETH` function with signature `withdrawETH(uint256)` and selector `0xf14210a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawETH", abi = "withdrawETH(uint256)")]
    pub struct WithdrawETHCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LiquidatorCalls {
        LiquidateUser(LiquidateUserCall),
        OnMorphoLiquidate(OnMorphoLiquidateCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SwapERC20(SwapERC20Call),
        TransferOwnership(TransferOwnershipCall),
        WithdrawERC20(WithdrawERC20Call),
        WithdrawETH(WithdrawETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LiquidateUserCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidateUser(decoded));
            }
            if let Ok(decoded) = <OnMorphoLiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnMorphoLiquidate(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SwapERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapERC20(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <WithdrawERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawERC20(decoded));
            }
            if let Ok(decoded) = <WithdrawETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LiquidateUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnMorphoLiquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LiquidateUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnMorphoLiquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LiquidateUserCall> for LiquidatorCalls {
        fn from(value: LiquidateUserCall) -> Self {
            Self::LiquidateUser(value)
        }
    }
    impl ::core::convert::From<OnMorphoLiquidateCall> for LiquidatorCalls {
        fn from(value: OnMorphoLiquidateCall) -> Self {
            Self::OnMorphoLiquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LiquidatorCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SwapERC20Call> for LiquidatorCalls {
        fn from(value: SwapERC20Call) -> Self {
            Self::SwapERC20(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawERC20Call> for LiquidatorCalls {
        fn from(value: WithdrawERC20Call) -> Self {
            Self::WithdrawERC20(value)
        }
    }
    impl ::core::convert::From<WithdrawETHCall> for LiquidatorCalls {
        fn from(value: WithdrawETHCall) -> Self {
            Self::WithdrawETH(value)
        }
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///`LiquidationParams(uint256,uint256,address,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidationParams {
        pub debt_quote: ::ethers::core::types::U256,
        pub builder_payment_percent: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
        pub swap_data: ::ethers::core::types::Bytes,
    }
}
