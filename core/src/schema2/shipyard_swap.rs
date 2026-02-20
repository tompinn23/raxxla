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
#[doc = "When Written: when switching to another ship already stored at this station"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when switching to another ship already stored at this station\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"ShipType\","]
#[doc = "    \"StoreOldShip\","]
#[doc = "    \"StoreShipID\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        128666762,"]
#[doc = "        3228917248"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        25,"]
#[doc = "        11"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipType\": {"]
#[doc = "      \"title\": \"ShipType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"asp\","]
#[doc = "        \"cutter\","]
#[doc = "        \"empire_trader\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipType_Localised\": {"]
#[doc = "      \"title\": \"ShipType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asp Explorer\","]
#[doc = "        \"Imperial Cutter\","]
#[doc = "        \"Imperial Clipper\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoreOldShip\": {"]
#[doc = "      \"title\": \"StoreOldShip\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Krait_MkII\","]
#[doc = "        \"Asp\","]
#[doc = "        \"Cutter\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoreShipID\": {"]
#[doc = "      \"title\": \"StoreShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        18,"]
#[doc = "        5,"]
#[doc = "        25"]
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
pub struct ShipyardSwap {
    #[serde(skip_deserializing, default = "ShipyardSwap::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[serde(rename = "ShipType")]
    pub ship_type: ::std::string::String,
    #[serde(
        rename = "ShipType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_type_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "StoreOldShip")]
    pub store_old_ship: ::std::string::String,
    #[serde(rename = "StoreShipID")]
    pub store_ship_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ShipyardSwap> for ShipyardSwap {
    fn from(value: &ShipyardSwap) -> Self {
        value.clone()
    }
}

impl ShipyardSwap {
    pub fn event_value() -> ::std::string::String {
        "ShipyardSwap".to_string()
    }
}
