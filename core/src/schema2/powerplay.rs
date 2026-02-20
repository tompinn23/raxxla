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
#[doc = "When written: at startup, if player has pledged to a power"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: at startup, if player has pledged to a power\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Merits\","]
#[doc = "    \"Power\","]
#[doc = "    \"Rank\","]
#[doc = "    \"TimePledged\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Merits\": {"]
#[doc = "      \"title\": \"Merits\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1260"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Power\": {"]
#[doc = "      \"title\": \"Power\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Li Yong-Rui\","]
#[doc = "        \"Aisling Duval\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Rank\": {"]
#[doc = "      \"title\": \"Rank\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        3,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TimePledged\": {"]
#[doc = "      \"title\": \"TimePledged\","]
#[doc = "      \"description\": \"Time pledged in seconds\","]
#[doc = "      \"examples\": ["]
#[doc = "        43809417,"]
#[doc = "        43811142,"]
#[doc = "        43813289"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Votes\": {"]
#[doc = "      \"title\": \"Votes\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        5"]
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
pub struct Powerplay {
    #[serde(skip_deserializing, default = "Powerplay::event_value")]
    pub event: String,
    #[serde(rename = "Merits")]
    pub merits: i64,
    #[serde(rename = "Power")]
    pub power: ::std::string::String,
    #[serde(rename = "Rank")]
    pub rank: i64,
    #[doc = "Time pledged in seconds"]
    #[serde(rename = "TimePledged")]
    pub time_pledged: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "Votes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub votes: ::std::option::Option<i64>,
}
impl ::std::convert::From<&Powerplay> for Powerplay {
    fn from(value: &Powerplay) -> Self {
        value.clone()
    }
}

impl Powerplay {
    pub fn event_value() -> ::std::string::String {
        "Powerplay".to_string()
    }
}
