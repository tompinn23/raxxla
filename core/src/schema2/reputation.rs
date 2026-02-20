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
#[doc = "When written: at startup (after Rank and Progress). This gives the player's reputation (on a scale of -100..+100) with the superpowers"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: at startup (after Rank and Progress). This gives the player's reputation (on a scale of -100..+100) with the superpowers\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Alliance\": {"]
#[doc = "      \"title\": \"Alliance\","]
#[doc = "      \"examples\": ["]
#[doc = "        71.8815,"]
#[doc = "        90.176201,"]
#[doc = "        89.642403"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": -100.0"]
#[doc = "    },"]
#[doc = "    \"Empire\": {"]
#[doc = "      \"title\": \"Empire\","]
#[doc = "      \"examples\": ["]
#[doc = "        5.42526,"]
#[doc = "        11.6653,"]
#[doc = "        12.0872"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": -100.0"]
#[doc = "    },"]
#[doc = "    \"Federation\": {"]
#[doc = "      \"title\": \"Federation\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.56348,"]
#[doc = "        -0.83652,"]
#[doc = "        -6.02652"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": -100.0"]
#[doc = "    },"]
#[doc = "    \"Independent\": {"]
#[doc = "      \"title\": \"Independent\","]
#[doc = "      \"examples\": ["]
#[doc = "        -38.199299,"]
#[doc = "        74.849998,"]
#[doc = "        50.513199"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": -100.0"]
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
pub struct Reputation {
    #[serde(
        rename = "Alliance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alliance: ::std::option::Option<f64>,
    #[serde(
        rename = "Empire",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub empire: ::std::option::Option<f64>,
    #[serde(skip_deserializing, default = "Reputation::event_value")]
    pub event: String,
    #[serde(
        rename = "Federation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub federation: ::std::option::Option<f64>,
    #[serde(
        rename = "Independent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub independent: ::std::option::Option<f64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Reputation> for Reputation {
    fn from(value: &Reputation) -> Self {
        value.clone()
    }
}

impl Reputation {
    pub fn event_value() -> ::std::string::String {
        "Reputation".to_string()
    }
}
