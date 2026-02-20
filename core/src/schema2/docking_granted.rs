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
#[doc = "When written: when a docking request is granted"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when a docking request is granted\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"LandingPad\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"StationName\","]
#[doc = "    \"StationType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"LandingPad\": {"]
#[doc = "      \"title\": \"LandingPad\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        3,"]
#[doc = "        11"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3701165824,"]
#[doc = "        3222025216"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StationName\": {"]
#[doc = "      \"title\": \"StationName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"A1A-B2B\","]
#[doc = "        \"Shinn Enterprise\","]
#[doc = "        \"$EXT_PANEL_ColonisationShip:#index=1;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationName_Localised\": {"]
#[doc = "      \"title\": \"StationName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"System Colonisation Ship\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationType\": {"]
#[doc = "      \"title\": \"StationType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FleetCarrier\","]
#[doc = "        \"Coriolis\","]
#[doc = "        \"CraterOutpost\""]
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
pub struct DockingGranted {
    #[serde(skip_deserializing, default = "DockingGranted::event_value")]
    pub event: String,
    #[serde(rename = "LandingPad")]
    pub landing_pad: i64,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "StationName")]
    pub station_name: ::std::string::String,
    #[serde(
        rename = "StationName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "StationType")]
    pub station_type: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&DockingGranted> for DockingGranted {
    fn from(value: &DockingGranted) -> Self {
        value.clone()
    }
}

impl DockingGranted {
    pub fn event_value() -> ::std::string::String {
        "DockingGranted".to_string()
    }
}
