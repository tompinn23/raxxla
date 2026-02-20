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
#[doc = "When written: when the captain in multicrew disbands the crew"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when the captain in multicrew disbands the crew\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"OnCrime\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"OnCrime\": {"]
#[doc = "      \"title\": \"OnCrime\","]
#[doc = "      \"description\": \"true if crew disbanded as a result of a crime in a lawful session\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Telepresence\": {"]
#[doc = "      \"title\": \"Telepresence\","]
#[doc = "      \"description\": \"only from Odyssey build\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
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
pub struct EndCrewSession {
    #[serde(skip_deserializing, default = "EndCrewSession::event_value")]
    pub event: String,
    #[doc = "true if crew disbanded as a result of a crime in a lawful session"]
    #[serde(rename = "OnCrime")]
    pub on_crime: bool,
    #[doc = "only from Odyssey build"]
    #[serde(
        rename = "Telepresence",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub telepresence: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&EndCrewSession> for EndCrewSession {
    fn from(value: &EndCrewSession) -> Self {
        value.clone()
    }
}

impl EndCrewSession {
    pub fn event_value() -> ::std::string::String {
        "EndCrewSession".to_string()
    }
}
