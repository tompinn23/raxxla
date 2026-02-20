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
#[doc = "When written: Player has escaped interdiction"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: Player has escaped interdiction\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Interdictor\","]
#[doc = "    \"IsPlayer\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Interdictor\": {"]
#[doc = "      \"title\": \"Interdictor\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$ShipName_Police_Independent;\","]
#[doc = "        \"Markus Klaus\","]
#[doc = "        \"Bob Royall\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Interdictor_Localised\": {"]
#[doc = "      \"title\": \"Interdictor_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"System Authority Vessel\","]
#[doc = "        \"Federal Security Service\","]
#[doc = "        \"Internal Security Service\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"IsPlayer\": {"]
#[doc = "      \"title\": \"IsPlayer\","]
#[doc = "      \"description\": \"Whether player or npc\","]
#[doc = "      \"examples\": ["]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"IsThargoid\": {"]
#[doc = "      \"title\": \"IsThargoid\","]
#[doc = "      \"description\": \"Whether thargoid\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
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
pub struct EscapeInterdiction {
    #[serde(skip_deserializing, default = "EscapeInterdiction::event_value")]
    pub event: String,
    #[serde(rename = "Interdictor")]
    pub interdictor: ::std::string::String,
    #[serde(
        rename = "Interdictor_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub interdictor_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Whether player or npc"]
    #[serde(rename = "IsPlayer")]
    pub is_player: bool,
    #[doc = "Whether thargoid"]
    #[serde(
        rename = "IsThargoid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_thargoid: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&EscapeInterdiction> for EscapeInterdiction {
    fn from(value: &EscapeInterdiction) -> Self {
        value.clone()
    }
}

impl EscapeInterdiction {
    pub fn event_value() -> ::std::string::String {
        "EscapeInterdiction".to_string()
    }
}
