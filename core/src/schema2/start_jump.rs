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
#[doc = "When written: at the start of a Hyperspace or Supercruise jump (start of countdown)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: at the start of a Hyperspace or Supercruise jump (start of countdown)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"JumpType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"JumpType\": {"]
#[doc = "      \"title\": \"JumpType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Hyperspace\","]
#[doc = "        \"Supercruise\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StarClass\": {"]
#[doc = "      \"title\": \"StarClass\","]
#[doc = "      \"description\": \"only for a hyperspace jump\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"B\","]
#[doc = "        \"TTS\","]
#[doc = "        \"M\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno\","]
#[doc = "        \"Tascheter Sector EL-Y b5\","]
#[doc = "        \"LHS 1443\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        198875014308,"]
#[doc = "        11665802143105,"]
#[doc = "        5068732442009"]
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
pub struct StartJump {
    #[serde(skip_deserializing, default = "StartJump::event_value")]
    pub event: String,
    #[serde(rename = "JumpType")]
    pub jump_type: ::std::string::String,
    #[doc = "only for a hyperspace jump"]
    #[serde(
        rename = "StarClass",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub star_class: ::std::option::Option<::std::string::String>,
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
impl ::std::convert::From<&StartJump> for StartJump {
    fn from(value: &StartJump) -> Self {
        value.clone()
    }
}

impl StartJump {
    pub fn event_value() -> ::std::string::String {
        "StartJump".to_string()
    }
}
