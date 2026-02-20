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
#[doc = "When written: when a text message is sent to another player"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when a text message is sent to another player\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Message\","]
#[doc = "    \"Sent\","]
#[doc = "    \"To\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Message\": {"]
#[doc = "      \"title\": \"Message\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"CMDR Alice waved at CMDR Mallory\","]
#[doc = "        \"CMDR Bob waved at CMDR Alice\","]
#[doc = "        \"CMDR Mallory waved at CMDR Bob\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Sent\": {"]
#[doc = "      \"title\": \"Sent\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"To\": {"]
#[doc = "      \"title\": \"To\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Alice\","]
#[doc = "        \"Bob\","]
#[doc = "        \"Mallory\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"To_Localised\": {"]
#[doc = "      \"title\": \"To_Localised\","]
#[doc = "      \"description\": \"Unclear under which conditions this field is present, but it sometimes is.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"CMDR Alice\","]
#[doc = "        \"CMDR Bob\","]
#[doc = "        \"CMDR Mallory\""]
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
pub struct SendText {
    #[serde(skip_deserializing, default = "SendText::event_value")]
    pub event: String,
    #[serde(rename = "Message")]
    pub message: ::std::string::String,
    #[serde(rename = "Sent")]
    pub sent: bool,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "To")]
    pub to: ::std::string::String,
    #[doc = "Unclear under which conditions this field is present, but it sometimes is."]
    #[serde(
        rename = "To_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub to_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SendText> for SendText {
    fn from(value: &SendText) -> Self {
        value.clone()
    }
}

impl SendText {
    pub fn event_value() -> ::std::string::String {
        "SendText".to_string()
    }
}
