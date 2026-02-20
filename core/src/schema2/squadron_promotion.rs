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
#[doc = "`SquadronPromotion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"NewRank\","]
#[doc = "    \"OldRank\","]
#[doc = "    \"SquadronName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"NewRank\": {"]
#[doc = "      \"title\": \"NewRank\","]
#[doc = "      \"examples\": ["]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"NewRankName\": {"]
#[doc = "      \"title\": \"NewRankName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Squadron_DefaultRankName_Rank1;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewRankName_Localised\": {"]
#[doc = "      \"title\": \"NewRankName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Senior Officer\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OldRank\": {"]
#[doc = "      \"title\": \"OldRank\","]
#[doc = "      \"examples\": ["]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"OldRankName\": {"]
#[doc = "      \"title\": \"OldRankName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Squadron_DefaultRankName_Rank4;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OldRankName_Localised\": {"]
#[doc = "      \"title\": \"OldRankName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Rookie\""]
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
pub struct SquadronPromotion {
    #[serde(skip_deserializing, default = "SquadronPromotion::event_value")]
    pub event: String,
    #[serde(rename = "NewRank")]
    pub new_rank: i64,
    #[serde(
        rename = "NewRankName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_rank_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "NewRankName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_rank_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "OldRank")]
    pub old_rank: i64,
    #[serde(
        rename = "OldRankName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub old_rank_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "OldRankName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub old_rank_name_localised: ::std::option::Option<::std::string::String>,
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
impl ::std::convert::From<&SquadronPromotion> for SquadronPromotion {
    fn from(value: &SquadronPromotion) -> Self {
        value.clone()
    }
}

impl SquadronPromotion {
    pub fn event_value() -> ::std::string::String {
        "SquadronPromotion".to_string()
    }
}
