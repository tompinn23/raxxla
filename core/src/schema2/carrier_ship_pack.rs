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
#[doc = "`CarrierShipPack`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CarrierID\","]
#[doc = "    \"Operation\","]
#[doc = "    \"PackTheme\","]
#[doc = "    \"PackTier\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CarrierID\": {"]
#[doc = "      \"title\": \"CarrierID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344"]
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
#[doc = "    \"Cost\": {"]
#[doc = "      \"title\": \"Cost\","]
#[doc = "      \"examples\": ["]
#[doc = "        9554600,"]
#[doc = "        566276585,"]
#[doc = "        19181515"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Operation\": {"]
#[doc = "      \"title\": \"Operation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"SellPack\","]
#[doc = "        \"BuyPack\","]
#[doc = "        \"RestockPack\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PackTheme\": {"]
#[doc = "      \"title\": \"PackTheme\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Zorgon Peterson - Cargo\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PackTier\": {"]
#[doc = "      \"title\": \"PackTier\","]
#[doc = "      \"examples\": ["]
#[doc = "        3,"]
#[doc = "        1,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Refund\": {"]
#[doc = "      \"title\": \"Refund\","]
#[doc = "      \"examples\": ["]
#[doc = "        345554641,"]
#[doc = "        554315009,"]
#[doc = "        19160455"]
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
pub struct CarrierShipPack {
    #[serde(rename = "CarrierID")]
    pub carrier_id: i64,
    #[serde(
        rename = "CarrierType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub carrier_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Cost",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cost: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "CarrierShipPack::event_value")]
    pub event: String,
    #[serde(rename = "Operation")]
    pub operation: ::std::string::String,
    #[serde(rename = "PackTheme")]
    pub pack_theme: ::std::string::String,
    #[serde(rename = "PackTier")]
    pub pack_tier: i64,
    #[serde(
        rename = "Refund",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refund: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CarrierShipPack> for CarrierShipPack {
    fn from(value: &CarrierShipPack) -> Self {
        value.clone()
    }
}

impl CarrierShipPack {
    pub fn event_value() -> ::std::string::String {
        "CarrierShipPack".to_string()
    }
}
