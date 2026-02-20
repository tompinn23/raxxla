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
#[doc = "When dropping out of supercruise at a targeted destination"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When dropping out of supercruise at a targeted destination\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Threat\","]
#[doc = "    \"Type\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3701165824,"]
#[doc = "        3222025216"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Threat\": {"]
#[doc = "      \"title\": \"Threat\","]
#[doc = "      \"examples\": ["]
#[doc = "        0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$USS_Type_MissionTarget;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type_Localised\": {"]
#[doc = "      \"title\": \"Type_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Target\""]
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
pub struct SupercruiseDestinationDrop {
    #[serde(
        skip_deserializing,
        default = "SupercruiseDestinationDrop::event_value"
    )]
    pub event: String,
    #[serde(
        rename = "MarketID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub market_id: ::std::option::Option<i64>,
    #[serde(rename = "Threat")]
    pub threat: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[serde(
        rename = "Type_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SupercruiseDestinationDrop> for SupercruiseDestinationDrop {
    fn from(value: &SupercruiseDestinationDrop) -> Self {
        value.clone()
    }
}

impl SupercruiseDestinationDrop {
    pub fn event_value() -> ::std::string::String {
        "SupercruiseDestinationDrop".to_string()
    }
}
