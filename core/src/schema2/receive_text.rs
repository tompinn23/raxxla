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
#[doc = "When written: when a text message is received from another player or npc"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when a text message is received from another player or npc\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Channel\","]
#[doc = "    \"From\","]
#[doc = "    \"Message\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Channel\": {"]
#[doc = "      \"title\": \"Channel\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"npc\","]
#[doc = "        \"starsystem\","]
#[doc = "        \"local\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"From\": {"]
#[doc = "      \"title\": \"From\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"\","]
#[doc = "        \"FleetCarrierName A1A-B2B\","]
#[doc = "        \"$ShipName_Police_Independent;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"From_Localised\": {"]
#[doc = "      \"title\": \"From_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"System Authority Vessel\","]
#[doc = "        \"System\","]
#[doc = "        \"Cruise Ship\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Message\": {"]
#[doc = "      \"title\": \"Message\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$COMMS_entered:#name=Asterope;\","]
#[doc = "        \"$COMMS_entered:#name=Celaeno;\","]
#[doc = "        \"$STATION_NoFireZone_entered;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Message_Localised\": {"]
#[doc = "      \"title\": \"Message_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Entered Channel: Asterope\","]
#[doc = "        \"Entered Channel: Celaeno\","]
#[doc = "        \"No fire zone entered.\""]
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
pub struct ReceiveText {
    #[serde(rename = "Channel")]
    pub channel: ::std::string::String,
    #[serde(skip_deserializing, default = "ReceiveText::event_value")]
    pub event: String,
    #[serde(rename = "From")]
    pub from: ::std::string::String,
    #[serde(
        rename = "From_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub from_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Message")]
    pub message: ::std::string::String,
    #[serde(
        rename = "Message_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ReceiveText> for ReceiveText {
    fn from(value: &ReceiveText) -> Self {
        value.clone()
    }
}

impl ReceiveText {
    pub fn event_value() -> ::std::string::String {
        "ReceiveText".to_string()
    }
}
