pub use i_oracle::*;
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
pub mod i_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BASE_FEED_1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_FEED_1"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BASE_FEED_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_FEED_2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BASE_VAULT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_VAULT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC4626"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BASE_VAULT_CONVERSION_SAMPLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BASE_VAULT_CONVERSION_SAMPLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QUOTE_FEED_1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("QUOTE_FEED_1"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QUOTE_FEED_2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("QUOTE_FEED_2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QUOTE_VAULT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("QUOTE_VAULT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC4626"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QUOTE_VAULT_CONVERSION_SAMPLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "QUOTE_VAULT_CONVERSION_SAMPLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SCALE_FACTOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SCALE_FACTOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IOracle)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `BASE_FEED_1` (0xf50a4718) function
        pub fn base_feed_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([245, 10, 71, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE_FEED_2` (0xdc53858c) function
        pub fn base_feed_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([220, 83, 133, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE_VAULT` (0xeaa2d7b4) function
        pub fn base_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 162, 215, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE_VAULT_CONVERSION_SAMPLE` (0x054f7ac0) function
        pub fn base_vault_conversion_sample(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([5, 79, 122, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `QUOTE_FEED_1` (0x56095e11) function
        pub fn quote_feed_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([86, 9, 94, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `QUOTE_FEED_2` (0xacfbd39e) function
        pub fn quote_feed_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([172, 251, 211, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `QUOTE_VAULT` (0x2e6f20a6) function
        pub fn quote_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([46, 111, 32, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `QUOTE_VAULT_CONVERSION_SAMPLE` (0x461739d2) function
        pub fn quote_vault_conversion_sample(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 23, 57, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SCALE_FACTOR` (0xce4b5bbe) function
        pub fn scale_factor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 75, 91, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price` (0xa035b1fe) function
        pub fn price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 53, 177, 254], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `BASE_FEED_1` function with signature `BASE_FEED_1()` and selector `0xf50a4718`
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
    #[ethcall(name = "BASE_FEED_1", abi = "BASE_FEED_1()")]
    pub struct BaseFeed1Call;
    ///Container type for all input parameters for the `BASE_FEED_2` function with signature `BASE_FEED_2()` and selector `0xdc53858c`
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
    #[ethcall(name = "BASE_FEED_2", abi = "BASE_FEED_2()")]
    pub struct BaseFeed2Call;
    ///Container type for all input parameters for the `BASE_VAULT` function with signature `BASE_VAULT()` and selector `0xeaa2d7b4`
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
    #[ethcall(name = "BASE_VAULT", abi = "BASE_VAULT()")]
    pub struct BaseVaultCall;
    ///Container type for all input parameters for the `BASE_VAULT_CONVERSION_SAMPLE` function with signature `BASE_VAULT_CONVERSION_SAMPLE()` and selector `0x054f7ac0`
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
        name = "BASE_VAULT_CONVERSION_SAMPLE",
        abi = "BASE_VAULT_CONVERSION_SAMPLE()"
    )]
    pub struct BaseVaultConversionSampleCall;
    ///Container type for all input parameters for the `QUOTE_FEED_1` function with signature `QUOTE_FEED_1()` and selector `0x56095e11`
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
    #[ethcall(name = "QUOTE_FEED_1", abi = "QUOTE_FEED_1()")]
    pub struct QuoteFeed1Call;
    ///Container type for all input parameters for the `QUOTE_FEED_2` function with signature `QUOTE_FEED_2()` and selector `0xacfbd39e`
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
    #[ethcall(name = "QUOTE_FEED_2", abi = "QUOTE_FEED_2()")]
    pub struct QuoteFeed2Call;
    ///Container type for all input parameters for the `QUOTE_VAULT` function with signature `QUOTE_VAULT()` and selector `0x2e6f20a6`
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
    #[ethcall(name = "QUOTE_VAULT", abi = "QUOTE_VAULT()")]
    pub struct QuoteVaultCall;
    ///Container type for all input parameters for the `QUOTE_VAULT_CONVERSION_SAMPLE` function with signature `QUOTE_VAULT_CONVERSION_SAMPLE()` and selector `0x461739d2`
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
        name = "QUOTE_VAULT_CONVERSION_SAMPLE",
        abi = "QUOTE_VAULT_CONVERSION_SAMPLE()"
    )]
    pub struct QuoteVaultConversionSampleCall;
    ///Container type for all input parameters for the `SCALE_FACTOR` function with signature `SCALE_FACTOR()` and selector `0xce4b5bbe`
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
    #[ethcall(name = "SCALE_FACTOR", abi = "SCALE_FACTOR()")]
    pub struct ScaleFactorCall;
    ///Container type for all input parameters for the `price` function with signature `price()` and selector `0xa035b1fe`
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
    #[ethcall(name = "price", abi = "price()")]
    pub struct PriceCall;
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
    pub enum IOracleCalls {
        BaseFeed1(BaseFeed1Call),
        BaseFeed2(BaseFeed2Call),
        BaseVault(BaseVaultCall),
        BaseVaultConversionSample(BaseVaultConversionSampleCall),
        QuoteFeed1(QuoteFeed1Call),
        QuoteFeed2(QuoteFeed2Call),
        QuoteVault(QuoteVaultCall),
        QuoteVaultConversionSample(QuoteVaultConversionSampleCall),
        ScaleFactor(ScaleFactorCall),
        Price(PriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BaseFeed1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseFeed1(decoded));
            }
            if let Ok(decoded) = <BaseFeed2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseFeed2(decoded));
            }
            if let Ok(decoded) = <BaseVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseVault(decoded));
            }
            if let Ok(decoded) = <BaseVaultConversionSampleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseVaultConversionSample(decoded));
            }
            if let Ok(decoded) = <QuoteFeed1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteFeed1(decoded));
            }
            if let Ok(decoded) = <QuoteFeed2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteFeed2(decoded));
            }
            if let Ok(decoded) = <QuoteVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteVault(decoded));
            }
            if let Ok(decoded) = <QuoteVaultConversionSampleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteVaultConversionSample(decoded));
            }
            if let Ok(decoded) = <ScaleFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ScaleFactor(decoded));
            }
            if let Ok(decoded) = <PriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Price(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BaseFeed1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseFeed2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseVaultConversionSample(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteFeed1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteFeed2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteVaultConversionSample(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScaleFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseFeed1(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseFeed2(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseVaultConversionSample(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteFeed1(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteFeed2(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteVaultConversionSample(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ScaleFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Price(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BaseFeed1Call> for IOracleCalls {
        fn from(value: BaseFeed1Call) -> Self {
            Self::BaseFeed1(value)
        }
    }
    impl ::core::convert::From<BaseFeed2Call> for IOracleCalls {
        fn from(value: BaseFeed2Call) -> Self {
            Self::BaseFeed2(value)
        }
    }
    impl ::core::convert::From<BaseVaultCall> for IOracleCalls {
        fn from(value: BaseVaultCall) -> Self {
            Self::BaseVault(value)
        }
    }
    impl ::core::convert::From<BaseVaultConversionSampleCall> for IOracleCalls {
        fn from(value: BaseVaultConversionSampleCall) -> Self {
            Self::BaseVaultConversionSample(value)
        }
    }
    impl ::core::convert::From<QuoteFeed1Call> for IOracleCalls {
        fn from(value: QuoteFeed1Call) -> Self {
            Self::QuoteFeed1(value)
        }
    }
    impl ::core::convert::From<QuoteFeed2Call> for IOracleCalls {
        fn from(value: QuoteFeed2Call) -> Self {
            Self::QuoteFeed2(value)
        }
    }
    impl ::core::convert::From<QuoteVaultCall> for IOracleCalls {
        fn from(value: QuoteVaultCall) -> Self {
            Self::QuoteVault(value)
        }
    }
    impl ::core::convert::From<QuoteVaultConversionSampleCall> for IOracleCalls {
        fn from(value: QuoteVaultConversionSampleCall) -> Self {
            Self::QuoteVaultConversionSample(value)
        }
    }
    impl ::core::convert::From<ScaleFactorCall> for IOracleCalls {
        fn from(value: ScaleFactorCall) -> Self {
            Self::ScaleFactor(value)
        }
    }
    impl ::core::convert::From<PriceCall> for IOracleCalls {
        fn from(value: PriceCall) -> Self {
            Self::Price(value)
        }
    }
    ///Container type for all return fields from the `BASE_FEED_1` function with signature `BASE_FEED_1()` and selector `0xf50a4718`
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
    pub struct BaseFeed1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE_FEED_2` function with signature `BASE_FEED_2()` and selector `0xdc53858c`
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
    pub struct BaseFeed2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE_VAULT` function with signature `BASE_VAULT()` and selector `0xeaa2d7b4`
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
    pub struct BaseVaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE_VAULT_CONVERSION_SAMPLE` function with signature `BASE_VAULT_CONVERSION_SAMPLE()` and selector `0x054f7ac0`
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
    pub struct BaseVaultConversionSampleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `QUOTE_FEED_1` function with signature `QUOTE_FEED_1()` and selector `0x56095e11`
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
    pub struct QuoteFeed1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `QUOTE_FEED_2` function with signature `QUOTE_FEED_2()` and selector `0xacfbd39e`
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
    pub struct QuoteFeed2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `QUOTE_VAULT` function with signature `QUOTE_VAULT()` and selector `0x2e6f20a6`
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
    pub struct QuoteVaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `QUOTE_VAULT_CONVERSION_SAMPLE` function with signature `QUOTE_VAULT_CONVERSION_SAMPLE()` and selector `0x461739d2`
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
    pub struct QuoteVaultConversionSampleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `SCALE_FACTOR` function with signature `SCALE_FACTOR()` and selector `0xce4b5bbe`
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
    pub struct ScaleFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `price` function with signature `price()` and selector `0xa035b1fe`
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
    pub struct PriceReturn(pub ::ethers::core::types::U256);
}
