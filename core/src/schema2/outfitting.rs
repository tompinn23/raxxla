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
#[doc = "`ItemsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BuyPrice\","]
#[doc = "    \"Name\","]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BuyPrice\": {"]
#[doc = "      \"title\": \"BuyPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        16731,"]
#[doc = "        66924,"]
#[doc = "        6275"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"hpt_pulselaser_fixed_medium\","]
#[doc = "        \"hpt_pulselaser_fixed_large\","]
#[doc = "        \"hpt_pulselaser_gimbal_small\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"id\","]
#[doc = "      \"examples\": ["]
#[doc = "        128049382,"]
#[doc = "        128049383,"]
#[doc = "        128049385"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ItemsItem {
    #[serde(rename = "BuyPrice")]
    pub buy_price: i64,
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
}
impl ::std::convert::From<&ItemsItem> for ItemsItem {
    fn from(value: &ItemsItem) -> Self {
        value.clone()
    }
}
#[doc = "Written when accessing the outfitting menu. The full parts pricelist is written to a separate file Outfitting.json."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Written when accessing the outfitting menu. The full parts pricelist is written to a separate file Outfitting.json.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"StationName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Horizons\": {"]
#[doc = "      \"title\": \"Horizons\","]
#[doc = "      \"description\": \"Only written in Outfitting.json\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Items\": {"]
#[doc = "      \"title\": \"Items\","]
#[doc = "      \"description\": \"Only written in Outfitting.json\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"BuyPrice\","]
#[doc = "          \"Name\","]
#[doc = "          \"id\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"BuyPrice\": {"]
#[doc = "            \"title\": \"BuyPrice\","]
#[doc = "            \"examples\": ["]
#[doc = "              16731,"]
#[doc = "              66924,"]
#[doc = "              6275"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"hpt_pulselaser_fixed_medium\","]
#[doc = "              \"hpt_pulselaser_fixed_large\","]
#[doc = "              \"hpt_pulselaser_gimbal_small\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"id\","]
#[doc = "            \"examples\": ["]
#[doc = "              128049382,"]
#[doc = "              128049383,"]
#[doc = "              128049385"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3225943808,"]
#[doc = "        3700367104"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Swoiwns ZD-B d1\","]
#[doc = "        \"Synuefai LW-N b52-1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationName\": {"]
#[doc = "      \"title\": \"StationName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"A1A-B2B\","]
#[doc = "        \"Smith Enterprise\""]
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
pub struct Outfitting {
    #[serde(skip_deserializing, default = "Outfitting::event_value")]
    pub event: String,
    #[doc = "Only written in Outfitting.json"]
    #[serde(
        rename = "Horizons",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub horizons: ::std::option::Option<bool>,
    #[doc = "Only written in Outfitting.json"]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub items: ::std::vec::Vec<ItemsItem>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "StationName")]
    pub station_name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Outfitting> for Outfitting {
    fn from(value: &Outfitting) -> Self {
        value.clone()
    }
}

impl Outfitting {
    pub fn event_value() -> ::std::string::String {
        "Outfitting".to_string()
    }
}
