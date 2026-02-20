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
#[doc = "`Fileheader`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Odyssey\","]
#[doc = "    \"build\","]
#[doc = "    \"event\","]
#[doc = "    \"gameversion\","]
#[doc = "    \"language\","]
#[doc = "    \"part\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Odyssey\": {"]
#[doc = "      \"title\": \"Odyssey\","]
#[doc = "      \"description\": \"True for gameversions >= 4.0, both Horizons and Odyssey.\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"build\": {"]
#[doc = "      \"title\": \"build\","]
#[doc = "      \"description\": \"Game build number\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"r282108/r0 \","]
#[doc = "        \"r282352/r0 \","]
#[doc = "        \"r284072/r0 \""]
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
#[doc = "    \"gameversion\": {"]
#[doc = "      \"title\": \"gameversion\","]
#[doc = "      \"description\": \"Which version of the game produced the log (will indicate if beta)\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"4.0.0.1200\","]
#[doc = "        \"4.0.0.1201\","]
#[doc = "        \"4.0.0.1300\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"language\": {"]
#[doc = "      \"title\": \"language\","]
#[doc = "      \"description\": \"The language code\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"English/UK\","]
#[doc = "        \"Russian/RU\","]
#[doc = "        \"French/FR\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"part\": {"]
#[doc = "      \"title\": \"part\","]
#[doc = "      \"description\": \"The file part number\","]
#[doc = "      \"examples\": ["]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
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
pub struct Fileheader {
    #[doc = "Game build number"]
    pub build: ::std::string::String,
    #[serde(skip_deserializing, default = "Fileheader::event_value")]
    pub event: String,
    #[doc = "Which version of the game produced the log (will indicate if beta)"]
    pub gameversion: ::std::string::String,
    #[doc = "The language code"]
    pub language: ::std::string::String,
    #[doc = "True for gameversions >= 4.0, both Horizons and Odyssey."]
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,
    #[doc = "The file part number"]
    pub part: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Fileheader> for Fileheader {
    fn from(value: &Fileheader) -> Self {
        value.clone()
    }
}

impl Fileheader {
    pub fn event_value() -> ::std::string::String {
        "Fileheader".to_string()
    }
}
