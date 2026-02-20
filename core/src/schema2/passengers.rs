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
#[doc = "`ManifestItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"MissionID\","]
#[doc = "    \"Type\","]
#[doc = "    \"VIP\","]
#[doc = "    \"Wanted\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        7,"]
#[doc = "        9,"]
#[doc = "        13"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        799410896,"]
#[doc = "        841578018,"]
#[doc = "        841577937"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Explorer\","]
#[doc = "        \"Tourist\","]
#[doc = "        \"Business\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"VIP\": {"]
#[doc = "      \"title\": \"VIP\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Wanted\": {"]
#[doc = "      \"title\": \"Wanted\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ManifestItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "MissionID")]
    pub mission_id: i64,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[serde(rename = "VIP")]
    pub vip: bool,
    #[serde(rename = "Wanted")]
    pub wanted: bool,
}
impl ::std::convert::From<&ManifestItem> for ManifestItem {
    fn from(value: &ManifestItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: at startup, when loading the saved game file"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: at startup, when loading the saved game file\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Manifest\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Manifest\": {"]
#[doc = "      \"title\": \"Manifest\","]
#[doc = "      \"description\": \"Array of passenger records\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Count\","]
#[doc = "          \"MissionID\","]
#[doc = "          \"Type\","]
#[doc = "          \"VIP\","]
#[doc = "          \"Wanted\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              7,"]
#[doc = "              9,"]
#[doc = "              13"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"MissionID\": {"]
#[doc = "            \"title\": \"MissionID\","]
#[doc = "            \"examples\": ["]
#[doc = "              799410896,"]
#[doc = "              841578018,"]
#[doc = "              841577937"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Type\": {"]
#[doc = "            \"title\": \"Type\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Explorer\","]
#[doc = "              \"Tourist\","]
#[doc = "              \"Business\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"VIP\": {"]
#[doc = "            \"title\": \"VIP\","]
#[doc = "            \"examples\": ["]
#[doc = "              true"]
#[doc = "            ],"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"Wanted\": {"]
#[doc = "            \"title\": \"Wanted\","]
#[doc = "            \"examples\": ["]
#[doc = "              false,"]
#[doc = "              true"]
#[doc = "            ],"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
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
pub struct Passengers {
    #[serde(skip_deserializing, default = "Passengers::event_value")]
    pub event: String,
    #[doc = "Array of passenger records"]
    #[serde(rename = "Manifest")]
    pub manifest: ::std::vec::Vec<ManifestItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Passengers> for Passengers {
    fn from(value: &Passengers) -> Self {
        value.clone()
    }
}

impl Passengers {
    pub fn event_value() -> ::std::string::String {
        "Passengers".to_string()
    }
}
