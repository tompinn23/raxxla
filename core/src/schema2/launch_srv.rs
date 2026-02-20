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
#[doc = "When written: deploying the SRV from a ship onto planet surface"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: deploying the SRV from a ship onto planet surface\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ID\","]
#[doc = "    \"Loadout\","]
#[doc = "    \"PlayerControlled\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ID\": {"]
#[doc = "      \"title\": \"ID\","]
#[doc = "      \"examples\": ["]
#[doc = "        33,"]
#[doc = "        35,"]
#[doc = "        37"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Loadout\": {"]
#[doc = "      \"title\": \"Loadout\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"default\","]
#[doc = "        \"starter\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PlayerControlled\": {"]
#[doc = "      \"title\": \"PlayerControlled\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"SRVType\": {"]
#[doc = "      \"title\": \"SRVType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"combat_multicrew_srv_01\","]
#[doc = "        \"testbuggy\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SRVType_Localised\": {"]
#[doc = "      \"title\": \"SRVType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"SRV Scorpion\","]
#[doc = "        \"SRV Scarab\""]
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
pub struct LaunchSRV {
    #[serde(skip_deserializing, default = "LaunchSRV::event_value")]
    pub event: String,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Loadout")]
    pub loadout: ::std::string::String,
    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
    #[serde(
        rename = "SRVType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub srv_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SRVType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub srv_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&LaunchSRV> for LaunchSRV {
    fn from(value: &LaunchSRV) -> Self {
        value.clone()
    }
}

impl LaunchSRV {
    pub fn event_value() -> ::std::string::String {
        "LaunchSRV".to_string()
    }
}
