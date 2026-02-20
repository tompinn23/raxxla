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
#[doc = "`RenameSuitLoadout`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"LoadoutID\","]
#[doc = "    \"LoadoutName\","]
#[doc = "    \"SuitID\","]
#[doc = "    \"SuitName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"LoadoutID\": {"]
#[doc = "      \"title\": \"LoadoutID\","]
#[doc = "      \"examples\": ["]
#[doc = "        4293000004,"]
#[doc = "        4293000002,"]
#[doc = "        4293000000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LoadoutName\": {"]
#[doc = "      \"title\": \"LoadoutName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Discoverer\","]
#[doc = "        \"Scavenger\","]
#[doc = "        \"Tanky\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SuitID\": {"]
#[doc = "      \"title\": \"SuitID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1702007358392257,"]
#[doc = "        1703514026864984,"]
#[doc = "        1705338124035131"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitName\": {"]
#[doc = "      \"title\": \"SuitName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"explorationsuit_class3\","]
#[doc = "        \"tacticalsuit_class5\","]
#[doc = "        \"explorationsuit_class5\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SuitName_Localised\": {"]
#[doc = "      \"title\": \"SuitName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$ExplorationSuit_Class1_Name;\","]
#[doc = "        \"$TacticalSuit_Class1_Name;\","]
#[doc = "        \"$UtilitySuit_Class1_Name;\""]
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
pub struct RenameSuitLoadout {
    #[serde(skip_deserializing, default = "RenameSuitLoadout::event_value")]
    pub event: String,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: i64,
    #[serde(rename = "LoadoutName")]
    pub loadout_name: ::std::string::String,
    #[serde(rename = "SuitID")]
    pub suit_id: i64,
    #[serde(rename = "SuitName")]
    pub suit_name: ::std::string::String,
    #[serde(
        rename = "SuitName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub suit_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&RenameSuitLoadout> for RenameSuitLoadout {
    fn from(value: &RenameSuitLoadout) -> Self {
        value.clone()
    }
}

impl RenameSuitLoadout {
    pub fn event_value() -> ::std::string::String {
        "RenameSuitLoadout".to_string()
    }
}
