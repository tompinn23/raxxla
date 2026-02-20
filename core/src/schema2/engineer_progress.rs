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
#[doc = "When Written: when a player increases their access to an engineer"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when a player increases their access to an engineer\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Engineer\": {"]
#[doc = "      \"title\": \"Engineer\","]
#[doc = "      \"description\": \"Update for one engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Zacariah Nemo\","]
#[doc = "        \"Lori Jameson\","]
#[doc = "        \"The Sarge\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"EngineerID\": {"]
#[doc = "      \"title\": \"EngineerID\","]
#[doc = "      \"description\": \"Update for one engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        300050,"]
#[doc = "        300230,"]
#[doc = "        300040"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Engineers\": {"]
#[doc = "      \"title\": \"Engineers\","]
#[doc = "      \"description\": \"Summary at startup\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Engineer\","]
#[doc = "          \"EngineerID\","]
#[doc = "          \"Progress\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Engineer\": {"]
#[doc = "            \"title\": \"Engineer\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Hera Tani\","]
#[doc = "              \"The Sarge\","]
#[doc = "              \"Professor Palin\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"EngineerID\": {"]
#[doc = "            \"title\": \"EngineerID\","]
#[doc = "            \"examples\": ["]
#[doc = "              300090,"]
#[doc = "              300040,"]
#[doc = "              300220"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Progress\": {"]
#[doc = "            \"title\": \"Progress\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Unlocked\","]
#[doc = "              \"Invited\","]
#[doc = "              \"Known\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Rank\": {"]
#[doc = "            \"title\": \"Rank\","]
#[doc = "            \"examples\": ["]
#[doc = "              5,"]
#[doc = "              0,"]
#[doc = "              3"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"RankProgress\": {"]
#[doc = "            \"title\": \"RankProgress\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              36,"]
#[doc = "              3"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Progress\": {"]
#[doc = "      \"title\": \"Progress\","]
#[doc = "      \"description\": \"Update for one engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Unlocked\","]
#[doc = "        \"Invited\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Rank\": {"]
#[doc = "      \"title\": \"Rank\","]
#[doc = "      \"description\": \"Update for one engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        4,"]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"RankProgress\": {"]
#[doc = "      \"title\": \"RankProgress\","]
#[doc = "      \"description\": \"Uncertain whether this is actually logged\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        36,"]
#[doc = "        3"]
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
pub struct EngineerProgress {
    #[doc = "Update for one engineer"]
    #[serde(
        rename = "Engineer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer: ::std::option::Option<::std::string::String>,
    #[doc = "Update for one engineer"]
    #[serde(
        rename = "EngineerID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer_id: ::std::option::Option<i64>,
    #[doc = "Summary at startup"]
    #[serde(
        rename = "Engineers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub engineers: ::std::vec::Vec<EngineersItem>,
    #[serde(skip_deserializing, default = "EngineerProgress::event_value")]
    pub event: String,
    #[doc = "Update for one engineer"]
    #[serde(
        rename = "Progress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress: ::std::option::Option<::std::string::String>,
    #[doc = "Update for one engineer"]
    #[serde(
        rename = "Rank",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rank: ::std::option::Option<i64>,
    #[doc = "Uncertain whether this is actually logged"]
    #[serde(
        rename = "RankProgress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rank_progress: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&EngineerProgress> for EngineerProgress {
    fn from(value: &EngineerProgress) -> Self {
        value.clone()
    }
}
#[doc = "`EngineersItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Engineer\","]
#[doc = "    \"EngineerID\","]
#[doc = "    \"Progress\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Engineer\": {"]
#[doc = "      \"title\": \"Engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Hera Tani\","]
#[doc = "        \"The Sarge\","]
#[doc = "        \"Professor Palin\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"EngineerID\": {"]
#[doc = "      \"title\": \"EngineerID\","]
#[doc = "      \"examples\": ["]
#[doc = "        300090,"]
#[doc = "        300040,"]
#[doc = "        300220"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Progress\": {"]
#[doc = "      \"title\": \"Progress\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Unlocked\","]
#[doc = "        \"Invited\","]
#[doc = "        \"Known\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Rank\": {"]
#[doc = "      \"title\": \"Rank\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        0,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"RankProgress\": {"]
#[doc = "      \"title\": \"RankProgress\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        36,"]
#[doc = "        3"]
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
pub struct EngineersItem {
    #[serde(rename = "Engineer")]
    pub engineer: ::std::string::String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,
    #[serde(rename = "Progress")]
    pub progress: ::std::string::String,
    #[serde(
        rename = "Rank",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rank: ::std::option::Option<i64>,
    #[serde(
        rename = "RankProgress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rank_progress: ::std::option::Option<i64>,
}
impl ::std::convert::From<&EngineersItem> for EngineersItem {
    fn from(value: &EngineersItem) -> Self {
        value.clone()
    }
}

impl EngineerProgress {
    pub fn event_value() -> ::std::string::String {
        "EngineerProgress".to_string()
    }
}
