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
#[doc = "`SquadronStartup`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CurrentRank\","]
#[doc = "    \"SquadronName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CurrentRank\": {"]
#[doc = "      \"title\": \"CurrentRank\","]
#[doc = "      \"examples\": ["]
#[doc = "        3,"]
#[doc = "        0,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CurrentRankName\": {"]
#[doc = "      \"title\": \"CurrentRankName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Pilot\","]
#[doc = "        \"$Squadron_DefaultRankName_Rank0;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"CurrentRankName_Localised\": {"]
#[doc = "      \"title\": \"CurrentRankName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Pilot\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SquadronID\": {"]
#[doc = "      \"title\": \"SquadronID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3,"]
#[doc = "        0,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SquadronName\": {"]
#[doc = "      \"title\": \"SquadronName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"TestSquadron\""]
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
pub struct SquadronStartup {
    #[serde(rename = "CurrentRank")]
    pub current_rank: i64,
    #[serde(
        rename = "CurrentRankName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_rank_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "CurrentRankName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_rank_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "SquadronStartup::event_value")]
    pub event: String,
    #[serde(
        rename = "SquadronID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub squadron_id: ::std::option::Option<i64>,
    #[serde(rename = "SquadronName")]
    pub squadron_name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&SquadronStartup> for SquadronStartup {
    fn from(value: &SquadronStartup) -> Self {
        value.clone()
    }
}

impl SquadronStartup {
    pub fn event_value() -> ::std::string::String {
        "SquadronStartup".to_string()
    }
}
