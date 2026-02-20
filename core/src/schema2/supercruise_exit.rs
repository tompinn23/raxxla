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
#[doc = "When written: leaving supercruise for normal space"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: leaving supercruise for normal space\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Body\","]
#[doc = "    \"BodyID\","]
#[doc = "    \"BodyType\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno 2\","]
#[doc = "        \"Panoi\","]
#[doc = "        \"HIP 20485 A 1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        10,"]
#[doc = "        0,"]
#[doc = "        11"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyType\": {"]
#[doc = "      \"title\": \"BodyType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Planet\","]
#[doc = "        \"Star\","]
#[doc = "        \"Station\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Multicrew\": {"]
#[doc = "      \"title\": \"Multicrew\","]
#[doc = "      \"examples\": ["]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno\","]
#[doc = "        \"Panoi\","]
#[doc = "        \"HIP 20485\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        198875014308,"]
#[doc = "        6955800204002,"]
#[doc = "        3657265287866"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Taxi\": {"]
#[doc = "      \"title\": \"Taxi\","]
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
pub struct SupercruiseExit {
    #[serde(rename = "Body")]
    pub body: ::std::string::String,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "BodyType")]
    pub body_type: ::std::string::String,
    #[serde(skip_deserializing, default = "SupercruiseExit::event_value")]
    pub event: String,
    #[serde(
        rename = "Multicrew",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub multicrew: ::std::option::Option<bool>,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[serde(
        rename = "Taxi",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taxi: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&SupercruiseExit> for SupercruiseExit {
    fn from(value: &SupercruiseExit) -> Self {
        value.clone()
    }
}

impl SupercruiseExit {
    pub fn event_value() -> ::std::string::String {
        "SupercruiseExit".to_string()
    }
}
