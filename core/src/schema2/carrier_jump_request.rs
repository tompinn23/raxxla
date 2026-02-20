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
#[doc = "At the time the player requests the jump, not the jump itself"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"At the time the player requests the jump, not the jump itself\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyID\","]
#[doc = "    \"CarrierID\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"SystemName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno 2\","]
#[doc = "        \"Synuefai UY-J b54-1 A\","]
#[doc = "        \"Swoiwns ZD-B d1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        10,"]
#[doc = "        11,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierID\": {"]
#[doc = "      \"title\": \"CarrierID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3707430144,"]
#[doc = "        3705124096"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierType\": {"]
#[doc = "      \"title\": \"CarrierType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FleetCarrier\","]
#[doc = "        \"SquadronCarrier\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"DepartureTime\": {"]
#[doc = "      \"title\": \"DepartureTime\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"2022-11-29T23:48:28Z\","]
#[doc = "        \"2022-12-01T10:01:24Z\","]
#[doc = "        \"2022-12-02T01:16:22Z\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        198875014308,"]
#[doc = "        3657265287866,"]
#[doc = "        2867023521233"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SystemName\": {"]
#[doc = "      \"title\": \"SystemName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno\","]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Synuefai UY-J b54-1\""]
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
pub struct CarrierJumpRequest {
    #[serde(
        rename = "Body",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "CarrierID")]
    pub carrier_id: i64,
    #[serde(skip_deserializing)]
    pub callsign: ::std::string::String,
    #[serde(
        rename = "CarrierType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub carrier_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "DepartureTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub departure_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_deserializing, default = "CarrierJumpRequest::event_value")]
    pub event: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[serde(rename = "SystemName")]
    pub system_name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CarrierJumpRequest> for CarrierJumpRequest {
    fn from(value: &CarrierJumpRequest) -> Self {
        value.clone()
    }
}

impl CarrierJumpRequest {
    pub fn event_value() -> ::std::string::String {
        "CarrierJumpRequest".to_string()
    }
}
