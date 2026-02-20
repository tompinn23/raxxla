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
#[doc = "When written: when scanning a data link"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when scanning a data link\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Message\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Message\": {"]
#[doc = "      \"title\": \"Message\","]
#[doc = "      \"description\": \"Message identifier from data link\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$DATAPOINT_GAMEPLAY_complete;\","]
#[doc = "        \"$DataLink_ScannedInfo_Operations;\","]
#[doc = "        \"$DataLink_ScannedInfo_Power_2;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Message_Localised\": {"]
#[doc = "      \"title\": \"Message_Localised\","]
#[doc = "      \"description\": \"Localized message from data link\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Alert: All Data Point telemetry links established, Intel package created.\","]
#[doc = "        \"Op-Net accessed\","]
#[doc = "        \"Fuel routing system accessed\""]
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
pub struct DatalinkScan {
    #[serde(skip_deserializing, default = "DatalinkScan::event_value")]
    pub event: String,
    #[doc = "Message identifier from data link"]
    #[serde(rename = "Message")]
    pub message: ::std::string::String,
    #[doc = "Localized message from data link"]
    #[serde(
        rename = "Message_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&DatalinkScan> for DatalinkScan {
    fn from(value: &DatalinkScan) -> Self {
        value.clone()
    }
}

impl DatalinkScan {
    pub fn event_value() -> ::std::string::String {
        "DatalinkScan".to_string()
    }
}
