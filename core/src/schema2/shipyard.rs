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
#[doc = "`PriceListItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ShipPrice\","]
#[doc = "    \"ShipType\","]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ShipPrice\": {"]
#[doc = "      \"title\": \"ShipPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        24336,"]
#[doc = "        34071,"]
#[doc = "        40094"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipType\": {"]
#[doc = "      \"title\": \"ShipType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"sidewinder\","]
#[doc = "        \"eagle\","]
#[doc = "        \"hauler\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipType_Localised\": {"]
#[doc = "      \"title\": \"ShipType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Imperial Eagle\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"id\","]
#[doc = "      \"examples\": ["]
#[doc = "        128049249,"]
#[doc = "        128049255,"]
#[doc = "        128049261"]
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
pub struct PriceListItem {
    pub id: i64,
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
}
impl ::std::convert::From<&PriceListItem> for PriceListItem {
    fn from(value: &PriceListItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: when accessing shipyard in a station. The full price list is written to a separate file, in the same folder as the journal, Shipyard.json"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when accessing shipyard in a station. The full price list is written to a separate file, in the same folder as the journal, Shipyard.json\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"StationName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AllowCobraMkIV\": {"]
#[doc = "      \"title\": \"AllowCobraMkIV\","]
#[doc = "      \"description\": \"Only written in Shipyard.json\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Horizons\": {"]
#[doc = "      \"title\": \"Horizons\","]
#[doc = "      \"description\": \"Only written in Shipyard.json\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3700367104,"]
#[doc = "        128666762"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"PriceList\": {"]
#[doc = "      \"title\": \"PriceList\","]
#[doc = "      \"description\": \"Only written in Shipyard.json\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"ShipPrice\","]
#[doc = "          \"ShipType\","]
#[doc = "          \"id\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"ShipPrice\": {"]
#[doc = "            \"title\": \"ShipPrice\","]
#[doc = "            \"examples\": ["]
#[doc = "              24336,"]
#[doc = "              34071,"]
#[doc = "              40094"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"ShipType\": {"]
#[doc = "            \"title\": \"ShipType\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"sidewinder\","]
#[doc = "              \"eagle\","]
#[doc = "              \"hauler\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"ShipType_Localised\": {"]
#[doc = "            \"title\": \"ShipType_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Imperial Eagle\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"id\","]
#[doc = "            \"examples\": ["]
#[doc = "              128049249,"]
#[doc = "              128049255,"]
#[doc = "              128049261"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno\","]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Swoiwns ZD-B d1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationName\": {"]
#[doc = "      \"title\": \"StationName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"A1A-B2B\","]
#[doc = "        \"Jameson Memorial\""]
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
pub struct Shipyard {
    #[doc = "Only written in Shipyard.json"]
    #[serde(
        rename = "AllowCobraMkIV",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_cobra_mk_iv: ::std::option::Option<bool>,
    #[serde(skip_deserializing, default = "Shipyard::event_value")]
    pub event: String,
    #[doc = "Only written in Shipyard.json"]
    #[serde(
        rename = "Horizons",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub horizons: ::std::option::Option<bool>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "Only written in Shipyard.json"]
    #[serde(
        rename = "PriceList",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub price_list: ::std::vec::Vec<PriceListItem>,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "StationName")]
    pub station_name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Shipyard> for Shipyard {
    fn from(value: &Shipyard) -> Self {
        value.clone()
    }
}

impl Shipyard {
    pub fn event_value() -> ::std::string::String {
        "Shipyard".to_string()
    }
}
