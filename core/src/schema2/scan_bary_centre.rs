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
#[doc = "When scanning one body of a binary pair, you will get an event detailing the orbital parameters of their BaryCentre"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When scanning one body of a binary pair, you will get an event detailing the orbital parameters of their BaryCentre\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"AscendingNode\","]
#[doc = "    \"BodyID\","]
#[doc = "    \"Eccentricity\","]
#[doc = "    \"MeanAnomaly\","]
#[doc = "    \"OrbitalInclination\","]
#[doc = "    \"OrbitalPeriod\","]
#[doc = "    \"Periapsis\","]
#[doc = "    \"SemiMajorAxis\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AscendingNode\": {"]
#[doc = "      \"title\": \"AscendingNode\","]
#[doc = "      \"examples\": ["]
#[doc = "        -120.920443,"]
#[doc = "        131.83321,"]
#[doc = "        80.449766"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        2,"]
#[doc = "        36"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Eccentricity\": {"]
#[doc = "      \"title\": \"Eccentricity\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.059955,"]
#[doc = "        0.192351,"]
#[doc = "        0.113812"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"MeanAnomaly\": {"]
#[doc = "      \"title\": \"MeanAnomaly\","]
#[doc = "      \"examples\": ["]
#[doc = "        50.230625,"]
#[doc = "        334.799505,"]
#[doc = "        111.542564"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"OrbitalInclination\": {"]
#[doc = "      \"title\": \"OrbitalInclination\","]
#[doc = "      \"examples\": ["]
#[doc = "        -40.157093,"]
#[doc = "        -5.839172,"]
#[doc = "        93.686874"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"OrbitalPeriod\": {"]
#[doc = "      \"title\": \"OrbitalPeriod\","]
#[doc = "      \"examples\": ["]
#[doc = "        7033498.644829,"]
#[doc = "        738302832841.8732,"]
#[doc = "        181543731.689453"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Periapsis\": {"]
#[doc = "      \"title\": \"Periapsis\","]
#[doc = "      \"examples\": ["]
#[doc = "        65.170837,"]
#[doc = "        332.975557,"]
#[doc = "        88.077519"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"SemiMajorAxis\": {"]
#[doc = "      \"title\": \"SemiMajorAxis\","]
#[doc = "      \"examples\": ["]
#[doc = "        13380128741.264343,"]
#[doc = "        45344798564910.89,"]
#[doc = "        112775021791.45811"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Turdet\","]
#[doc = "        \"Arietis Sector ZF-O b6-1\","]
#[doc = "        \"Burigpa\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        7269097743753,"]
#[doc = "        2868635248009,"]
#[doc = "        669880427913"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
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
pub struct ScanBaryCentre {
    #[serde(rename = "AscendingNode")]
    pub ascending_node: f64,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "Eccentricity")]
    pub eccentricity: f64,
    #[serde(skip_deserializing, default = "ScanBaryCentre::event_value")]
    pub event: String,
    #[serde(rename = "MeanAnomaly")]
    pub mean_anomaly: f64,
    #[serde(rename = "OrbitalInclination")]
    pub orbital_inclination: f64,
    #[serde(rename = "OrbitalPeriod")]
    pub orbital_period: f64,
    #[serde(rename = "Periapsis")]
    pub periapsis: f64,
    #[serde(rename = "SemiMajorAxis")]
    pub semi_major_axis: f64,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ScanBaryCentre> for ScanBaryCentre {
    fn from(value: &ScanBaryCentre) -> Self {
        value.clone()
    }
}

impl ScanBaryCentre {
    pub fn event_value() -> ::std::string::String {
        "ScanBaryCentre".to_string()
    }
}
