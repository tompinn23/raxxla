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
#[doc = "When written: when the player’s rank increases. Only the relevant ranks are included."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when the player’s rank increases. Only the relevant ranks are included.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Combat\": {"]
#[doc = "      \"title\": \"Combat\","]
#[doc = "      \"examples\": ["]
#[doc = "        7,"]
#[doc = "        6,"]
#[doc = "        9"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Empire\": {"]
#[doc = "      \"title\": \"Empire\","]
#[doc = "      \"examples\": ["]
#[doc = "        14"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Exobiologist\": {"]
#[doc = "      \"title\": \"Exobiologist\","]
#[doc = "      \"examples\": ["]
#[doc = "        2,"]
#[doc = "        9,"]
#[doc = "        11"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Explore\": {"]
#[doc = "      \"title\": \"Explore\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        10,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Federation\": {"]
#[doc = "      \"title\": \"Federation\","]
#[doc = "      \"examples\": ["]
#[doc = "        24,"]
#[doc = "        12,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Soldier\": {"]
#[doc = "      \"title\": \"Soldier\","]
#[doc = "      \"examples\": ["]
#[doc = "        6,"]
#[doc = "        2,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Trade\": {"]
#[doc = "      \"title\": \"Trade\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        10,"]
#[doc = "        11"]
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
pub struct Promotion {
    #[serde(
        rename = "Combat",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub combat: ::std::option::Option<i64>,
    #[serde(
        rename = "Empire",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub empire: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "Promotion::event_value")]
    pub event: String,
    #[serde(
        rename = "Exobiologist",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exobiologist: ::std::option::Option<i64>,
    #[serde(
        rename = "Explore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub explore: ::std::option::Option<i64>,
    #[serde(
        rename = "Federation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation: ::std::option::Option<i64>,
    #[serde(
        rename = "Soldier",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub soldier: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "Trade",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trade: ::std::option::Option<i64>,
}
impl ::std::convert::From<&Promotion> for Promotion {
    fn from(value: &Promotion) -> Self {
        value.clone()
    }
}

impl Promotion {
    pub fn event_value() -> ::std::string::String {
        "Promotion".to_string()
    }
}
