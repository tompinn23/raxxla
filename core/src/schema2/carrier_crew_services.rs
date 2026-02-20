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
#[doc = "Changes to crew"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Changes to crew\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CarrierID\","]
#[doc = "    \"CrewName\","]
#[doc = "    \"CrewRole\","]
#[doc = "    \"Operation\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
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
#[doc = "    \"CrewName\": {"]
#[doc = "      \"title\": \"CrewName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Kareem Pickett\","]
#[doc = "        \"Floyd Hughes\","]
#[doc = "        \"Coleman Sims\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"CrewRole\": {"]
#[doc = "      \"title\": \"CrewRole\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Bartender\","]
#[doc = "        \"VistaGenomics\","]
#[doc = "        \"PioneerSupplies\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Operation\": {"]
#[doc = "      \"title\": \"Operation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Activate\","]
#[doc = "        \"Replace\","]
#[doc = "        \"Deactivate\","]
#[doc = "        \"Resume\","]
#[doc = "        \"Pause\""]
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
pub struct CarrierCrewServices {
    #[serde(rename = "CarrierID")]
    pub carrier_id: i64,
    #[serde(
        rename = "CarrierType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub carrier_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "CrewName")]
    pub crew_name: ::std::string::String,
    #[serde(rename = "CrewRole")]
    pub crew_role: ::std::string::String,
    #[serde(skip_deserializing, default = "CarrierCrewServices::event_value")]
    pub event: String,
    #[serde(rename = "Operation")]
    pub operation: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CarrierCrewServices> for CarrierCrewServices {
    fn from(value: &CarrierCrewServices) -> Self {
        value.clone()
    }
}

impl CarrierCrewServices {
    pub fn event_value() -> ::std::string::String {
        "CarrierCrewServices".to_string()
    }
}
