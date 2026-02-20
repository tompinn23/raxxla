#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "The carrier owner has requested the carrier buys or sells goods (or cancels such an order)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The carrier owner has requested the carrier buys or sells goods (or cancels such an order)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BlackMarket\","]
#[doc = "    \"CarrierID\","]
#[doc = "    \"Commodity\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BlackMarket\": {"]
#[doc = "      \"title\": \"BlackMarket\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"CancelTrade\": {"]
#[doc = "      \"title\": \"CancelTrade\","]
#[doc = "      \"description\": \"PurchaseOrder or SaleOrder or CancelTrade\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"CarrierID\": {"]
#[doc = "      \"title\": \"CarrierID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3707430144,"]
#[doc = "        3705124096"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierType\": {"]
#[doc = "      \"title\": \"CarrierType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FleetCarrier\","]
#[doc = "        \"SquadronCarrier\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Commodity\": {"]
#[doc = "      \"title\": \"Commodity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"largecapacitypowerregulator\","]
#[doc = "        \"buildingschematic\","]
#[doc = "        \"powergridassembly\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Commodity_Localised\": {"]
#[doc = "      \"title\": \"Commodity_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Power Regulator\","]
#[doc = "        \"Building Schematic\","]
#[doc = "        \"Energy Grid Assembly\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Price\": {"]
#[doc = "      \"title\": \"Price\","]
#[doc = "      \"examples\": ["]
#[doc = "        500000,"]
#[doc = "        250000,"]
#[doc = "        2500"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"PurchaseOrder\": {"]
#[doc = "      \"title\": \"PurchaseOrder\","]
#[doc = "      \"description\": \"PurchaseOrder or SaleOrder or CancelTrade\","]
#[doc = "      \"examples\": ["]
#[doc = "        100,"]
#[doc = "        206,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SaleOrder\": {"]
#[doc = "      \"title\": \"SaleOrder\","]
#[doc = "      \"description\": \"PurchaseOrder or SaleOrder or CancelTrade\","]
#[doc = "      \"examples\": ["]
#[doc = "        100,"]
#[doc = "        5,"]
#[doc = "        21000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"event\": {"]
#[doc = "      \"title\": \"event\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"AfmuRepairs\","]
#[doc = "        \"FSDJump\","]
#[doc = "        \"Location\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"title\": \"timestamp\","]
#[doc = "      \"description\": \"Timestamp in UTC, ISO 8601\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"2022-09-11T22:00:45Z\","]
#[doc = "        \"2022-09-12T08:54:21Z\","]
#[doc = "        \"2022-09-12T08:54:24Z\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CarrierTradeOrder {
    #[serde(rename = "BlackMarket")]
    pub black_market: bool,
    #[doc = "PurchaseOrder or SaleOrder or CancelTrade"]
    #[serde(
        rename = "CancelTrade",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cancel_trade: ::std::option::Option<bool>,
    #[serde(rename = "CarrierID")]
    pub carrier_id: i64,
    #[serde(
        rename = "CarrierType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub carrier_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Commodity")]
    pub commodity: ::std::string::String,
    #[serde(
        rename = "Commodity_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity_localised: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "CarrierTradeOrder::event_value")]
    pub event: String,
    #[serde(
        rename = "Price",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub price: ::std::option::Option<i64>,
    #[doc = "PurchaseOrder or SaleOrder or CancelTrade"]
    #[serde(
        rename = "PurchaseOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub purchase_order: ::std::option::Option<i64>,
    #[doc = "PurchaseOrder or SaleOrder or CancelTrade"]
    #[serde(
        rename = "SaleOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sale_order: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CarrierTradeOrder> for CarrierTradeOrder {
    fn from(value: &CarrierTradeOrder) -> Self {
        value.clone()
    }
}

impl CarrierTradeOrder {
    pub fn event_value() -> ::std::string::String {
        "CarrierTradeOrder".to_string()
    }
}
