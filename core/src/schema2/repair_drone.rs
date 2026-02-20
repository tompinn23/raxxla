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
#[doc = "When written: when the player's ship has been repaired by a repair drone. Each of the values indicate the amount of damage that has been repaired"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when the player's ship has been repaired by a repair drone. Each of the values indicate the amount of damage that has been repaired\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CockpitRepaired\": {"]
#[doc = "      \"title\": \"CockpitRepaired\","]
#[doc = "      \"examples\": ["]
#[doc = "        4.391763,"]
#[doc = "        0.02048,"]
#[doc = "        0.2048"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"CorrosionRepaired\": {"]
#[doc = "      \"title\": \"CorrosionRepaired\","]
#[doc = "      \"examples\": ["]
#[doc = "        24.0,"]
#[doc = "        10.0,"]
#[doc = "        30.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"HullRepaired\": {"]
#[doc = "      \"title\": \"HullRepaired\","]
#[doc = "      \"examples\": ["]
#[doc = "        310.007385,"]
#[doc = "        310.007324,"]
#[doc = "        309.998169"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
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
pub struct RepairDrone {
    #[serde(
        rename = "CockpitRepaired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cockpit_repaired: ::std::option::Option<f64>,
    #[serde(
        rename = "CorrosionRepaired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub corrosion_repaired: ::std::option::Option<f64>,
    #[serde(skip_deserializing, default = "RepairDrone::event_value")]
    pub event: String,
    #[serde(
        rename = "HullRepaired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hull_repaired: ::std::option::Option<f64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&RepairDrone> for RepairDrone {
    fn from(value: &RepairDrone) -> Self {
        value.clone()
    }
}

impl RepairDrone {
    pub fn event_value() -> ::std::string::String {
        "RepairDrone".to_string()
    }
}
