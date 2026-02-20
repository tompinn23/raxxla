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
#[doc = "When Written: when selling a ship stored in the shipyard"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when selling a ship stored in the shipyard\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"SellShipID\","]
#[doc = "    \"ShipPrice\","]
#[doc = "    \"ShipType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3226858240,"]
#[doc = "        128666762"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SellShipID\": {"]
#[doc = "      \"title\": \"SellShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        36,"]
#[doc = "        68,"]
#[doc = "        6"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipMarketID\": {"]
#[doc = "      \"title\": \"ShipMarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128110840,"]
#[doc = "        3227808512,"]
#[doc = "        3706338816"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipPrice\": {"]
#[doc = "      \"title\": \"ShipPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        28080,"]
#[doc = "        12730985,"]
#[doc = "        104262846"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipType\": {"]
#[doc = "      \"title\": \"ShipType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"sidewinder\","]
#[doc = "        \"type7\","]
#[doc = "        \"anaconda\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipType_Localised\": {"]
#[doc = "      \"title\": \"ShipType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Type-7 Transporter\","]
#[doc = "        \"Asp Explorer\","]
#[doc = "        \"Fer-de-Lance\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"System\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"description\": \"If ship is in another system\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Zeaex\","]
#[doc = "        \"Wiliajuk\","]
#[doc = "        \"Harow\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
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
pub struct ShipyardSell {
    #[serde(skip_deserializing, default = "ShipyardSell::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "SellShipID")]
    pub sell_ship_id: i64,
    #[serde(
        rename = "ShipMarketID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_market_id: ::std::option::Option<i64>,
    #[serde(rename = "ShipPrice")]
    pub ship_price: i64,
    #[serde(rename = "ShipType")]
    pub ship_type: ::std::string::String,
    #[serde(
        rename = "ShipType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "If ship is in another system"]
    #[serde(
        rename = "System",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ShipyardSell> for ShipyardSell {
    fn from(value: &ShipyardSell) -> Self {
        value.clone()
    }
}

impl ShipyardSell {
    pub fn event_value() -> ::std::string::String {
        "ShipyardSell".to_string()
    }
}
