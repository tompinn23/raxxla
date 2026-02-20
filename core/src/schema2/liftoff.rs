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
#[doc = "When written: when taking off from planet surface"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when taking off from planet surface\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"PlayerControlled\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485 C 5\","]
#[doc = "        \"Vanth\","]
#[doc = "        \"Actaea\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        35,"]
#[doc = "        38,"]
#[doc = "        11"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Latitude\": {"]
#[doc = "      \"title\": \"Latitude\","]
#[doc = "      \"description\": \"only if player flying in ship)\","]
#[doc = "      \"examples\": ["]
#[doc = "        -1.477483,"]
#[doc = "        -40.646881,"]
#[doc = "        22.613714"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 90.0,"]
#[doc = "      \"minimum\": -90.0"]
#[doc = "    },"]
#[doc = "    \"Longitude\": {"]
#[doc = "      \"title\": \"Longitude\","]
#[doc = "      \"description\": \"only if player flying in ship)\","]
#[doc = "      \"examples\": ["]
#[doc = "        -17.250286,"]
#[doc = "        -17.250288,"]
#[doc = "        -44.13446"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 180.0,"]
#[doc = "      \"minimum\": -180.0"]
#[doc = "    },"]
#[doc = "    \"Multicrew\": {"]
#[doc = "      \"title\": \"Multicrew\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"NearestDestination\": {"]
#[doc = "      \"title\": \"NearestDestination\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Foster Synthetics Exchange\","]
#[doc = "        \"Uutoni Extraction Platform\","]
#[doc = "        \"$SAA_Unknown_Signal:#type=$SAA_SignalType_Human;:#index=0;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NearestDestination_Localised\": {"]
#[doc = "      \"title\": \"NearestDestination_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Human Signal (0)\","]
#[doc = "        \"Ancient Ruins (1)\","]
#[doc = "        \"Crashed Thargoid Ship\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OnPlanet\": {"]
#[doc = "      \"title\": \"OnPlanet\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"OnStation\": {"]
#[doc = "      \"title\": \"OnStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"PlayerControlled\": {"]
#[doc = "      \"title\": \"PlayerControlled\","]
#[doc = "      \"description\": \"false if ship dismissed when player is in SRV, true if player is taking off\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Sol\","]
#[doc = "        \"Latorioson\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        3657265287866,"]
#[doc = "        10477373803,"]
#[doc = "        671222670713"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Taxi\": {"]
#[doc = "      \"title\": \"Taxi\","]
#[doc = "      \"examples\": ["]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
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
pub struct Liftoff {
    #[serde(
        rename = "Body",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "BodyID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body_id: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "Liftoff::event_value")]
    pub event: String,
    #[serde(
        rename = "Latitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latitude: ::std::option::Option<f64>,
    #[serde(
        rename = "Longitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub longitude: ::std::option::Option<f64>,
    #[serde(
        rename = "Multicrew",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub multicrew: ::std::option::Option<bool>,
    #[serde(
        rename = "NearestDestination",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nearest_destination: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "NearestDestination_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nearest_destination_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "OnPlanet",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_planet: ::std::option::Option<bool>,
    #[serde(
        rename = "OnStation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub on_station: ::std::option::Option<bool>,
    #[doc = "false if ship dismissed when player is in SRV, true if player is taking off"]
    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
    #[serde(
        rename = "StarSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub star_system: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SystemAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_address: ::std::option::Option<i64>,
    #[serde(
        rename = "Taxi",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taxi: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Liftoff> for Liftoff {
    fn from(value: &Liftoff) -> Self {
        value.clone()
    }
}

impl Liftoff {
    pub fn event_value() -> ::std::string::String {
        "Liftoff".to_string()
    }
}
