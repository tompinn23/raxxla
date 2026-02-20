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
#[doc = "`CrewItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"Role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Alice\","]
#[doc = "        \"Bob\","]
#[doc = "        \"Mallory\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Role\": {"]
#[doc = "      \"title\": \"Role\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Helm\","]
#[doc = "        \"Idle\","]
#[doc = "        \"FireCon\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CrewItem {
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(rename = "Role")]
    pub role: ::std::string::String,
}
impl ::std::convert::From<&CrewItem> for CrewItem {
    fn from(value: &CrewItem) -> Self {
        value.clone()
    }
}
#[doc = "This event is logged when a player (on foot) gets into a ship or SRV"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when a player (on foot) gets into a ship or SRV\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Body\","]
#[doc = "    \"BodyID\","]
#[doc = "    \"Multicrew\","]
#[doc = "    \"OnPlanet\","]
#[doc = "    \"OnStation\","]
#[doc = "    \"SRV\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"Taxi\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485 A 1\","]
#[doc = "        \"Panoi\","]
#[doc = "        \"HIP 20485 C 5\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        11,"]
#[doc = "        0,"]
#[doc = "        35"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Crew\": {"]
#[doc = "      \"title\": \"Crew\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Name\","]
#[doc = "          \"Role\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Alice\","]
#[doc = "              \"Bob\","]
#[doc = "              \"Mallory\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Role\": {"]
#[doc = "            \"title\": \"Role\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Helm\","]
#[doc = "              \"Idle\","]
#[doc = "              \"FireCon\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ID\": {"]
#[doc = "      \"title\": \"ID\","]
#[doc = "      \"description\": \"player’s ship ID (if players own vessel)\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        33,"]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3222025216,"]
#[doc = "        3221924608,"]
#[doc = "        128831127"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Multicrew\": {"]
#[doc = "      \"title\": \"Multicrew\","]
#[doc = "      \"description\": \"true when boarding another player’s vessel\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"OnPlanet\": {"]
#[doc = "      \"title\": \"OnPlanet\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"OnStation\": {"]
#[doc = "      \"title\": \"OnStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"SRV\": {"]
#[doc = "      \"title\": \"SRV\","]
#[doc = "      \"description\": \"true if getting into SRV, false if getting into a ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Panoi\","]
#[doc = "        \"Siris\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationName\": {"]
#[doc = "      \"title\": \"StationName\","]
#[doc = "      \"description\": \"if at a station\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Shinn Enterprise\","]
#[doc = "        \"Bowersox Terminal\","]
#[doc = "        \"Penal Ship Y-32\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationType\": {"]
#[doc = "      \"title\": \"StationType\","]
#[doc = "      \"description\": \"if at a station\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Coriolis\","]
#[doc = "        \"Outpost\","]
#[doc = "        \"MegaShip\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        3657265287866,"]
#[doc = "        6955800204002,"]
#[doc = "        7269634614689"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Taxi\": {"]
#[doc = "      \"title\": \"Taxi\","]
#[doc = "      \"description\": \"true when boarding a taxi transport ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
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
pub struct Embark {
    #[serde(rename = "Body")]
    pub body: ::std::string::String,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(
        rename = "Crew",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub crew: ::std::vec::Vec<CrewItem>,
    #[serde(skip_deserializing, default = "Embark::event_value")]
    pub event: String,
    #[doc = "player’s ship ID (if players own vessel)"]
    #[serde(
        rename = "ID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<i64>,
    #[serde(
        rename = "MarketID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub market_id: ::std::option::Option<i64>,
    #[doc = "true when boarding another player’s vessel"]
    #[serde(rename = "Multicrew")]
    pub multicrew: bool,
    #[serde(rename = "OnPlanet")]
    pub on_planet: bool,
    #[serde(rename = "OnStation")]
    pub on_station: bool,
    #[doc = "true if getting into SRV, false if getting into a ship"]
    #[serde(rename = "SRV")]
    pub srv: bool,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[doc = "if at a station"]
    #[serde(
        rename = "StationName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_name: ::std::option::Option<::std::string::String>,
    #[doc = "if at a station"]
    #[serde(
        rename = "StationType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "true when boarding a taxi transport ship"]
    #[serde(rename = "Taxi")]
    pub taxi: bool,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Embark> for Embark {
    fn from(value: &Embark) -> Self {
        value.clone()
    }
}

impl Embark {
    pub fn event_value() -> ::std::string::String {
        "Embark".to_string()
    }
}
