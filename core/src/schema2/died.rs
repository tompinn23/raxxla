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
#[doc = "When written: player was killed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: player was killed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"KillerName\": {"]
#[doc = "      \"title\": \"KillerName\","]
#[doc = "      \"description\": \"When killed by a single entity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Cmdr banana\","]
#[doc = "        \"$UNKNOWN;\","]
#[doc = "        \"Cmdr apple\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillerName_Localised\": {"]
#[doc = "      \"title\": \"KillerName_Localised\","]
#[doc = "      \"description\": \"When killed by a single entity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Unknown\","]
#[doc = "        \"Internal Security Service\","]
#[doc = "        \"System Authority Vessel\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillerRank\": {"]
#[doc = "      \"title\": \"KillerRank\","]
#[doc = "      \"description\": \"When killed by a single entity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Master\","]
#[doc = "        \"Elite\","]
#[doc = "        \"Deadly\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillerShip\": {"]
#[doc = "      \"title\": \"KillerShip\","]
#[doc = "      \"description\": \"When killed by a single entity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"utilitysuit_class5\","]
#[doc = "        \"ps_turretbasemedium02_6m\","]
#[doc = "        \"scout_q\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Killers\": {"]
#[doc = "      \"title\": \"Killers\","]
#[doc = "      \"description\": \"When killed by a wing\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"KillerName\","]
#[doc = "          \"KillerRank\","]
#[doc = "          \"KillerShip\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"KillerName\": {"]
#[doc = "            \"title\": \"KillerName\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Cmdr Alice\","]
#[doc = "              \"$UNKNOWN;\","]
#[doc = "              \"Cmdr Bob\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"KillerName_Localised\": {"]
#[doc = "            \"title\": \"KillerName_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Unknown\","]
#[doc = "              \"Internal Security Service\","]
#[doc = "              \"System Authority Vessel\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"KillerRank\": {"]
#[doc = "            \"title\": \"KillerRank\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Master\","]
#[doc = "              \"Elite\","]
#[doc = "              \"Deadly\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"KillerShip\": {"]
#[doc = "            \"title\": \"KillerShip\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"utilitysuit_class5\","]
#[doc = "              \"ps_turretbasemedium02_6m\","]
#[doc = "              \"scout_q\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
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
pub struct Died {
    #[serde(skip_deserializing, default = "Died::event_value")]
    pub event: String,
    #[doc = "When killed by a single entity"]
    #[serde(
        rename = "KillerName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub killer_name: ::std::option::Option<::std::string::String>,
    #[doc = "When killed by a single entity"]
    #[serde(
        rename = "KillerName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub killer_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "When killed by a single entity"]
    #[serde(
        rename = "KillerRank",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub killer_rank: ::std::option::Option<::std::string::String>,
    #[doc = "When killed by a single entity"]
    #[serde(
        rename = "KillerShip",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub killer_ship: ::std::option::Option<::std::string::String>,
    #[doc = "When killed by a wing"]
    #[serde(
        rename = "Killers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub killers: ::std::vec::Vec<KillersItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Died> for Died {
    fn from(value: &Died) -> Self {
        value.clone()
    }
}
#[doc = "`KillersItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"KillerName\","]
#[doc = "    \"KillerRank\","]
#[doc = "    \"KillerShip\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"KillerName\": {"]
#[doc = "      \"title\": \"KillerName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Cmdr Alice\","]
#[doc = "        \"$UNKNOWN;\","]
#[doc = "        \"Cmdr Bob\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillerName_Localised\": {"]
#[doc = "      \"title\": \"KillerName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Unknown\","]
#[doc = "        \"Internal Security Service\","]
#[doc = "        \"System Authority Vessel\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillerRank\": {"]
#[doc = "      \"title\": \"KillerRank\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Master\","]
#[doc = "        \"Elite\","]
#[doc = "        \"Deadly\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillerShip\": {"]
#[doc = "      \"title\": \"KillerShip\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"utilitysuit_class5\","]
#[doc = "        \"ps_turretbasemedium02_6m\","]
#[doc = "        \"scout_q\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct KillersItem {
    #[serde(rename = "KillerName")]
    pub killer_name: ::std::string::String,
    #[serde(
        rename = "KillerName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub killer_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "KillerRank")]
    pub killer_rank: ::std::string::String,
    #[serde(rename = "KillerShip")]
    pub killer_ship: ::std::string::String,
}
impl ::std::convert::From<&KillersItem> for KillersItem {
    fn from(value: &KillersItem) -> Self {
        value.clone()
    }
}

impl Died {
    pub fn event_value() -> ::std::string::String {
        "Died".to_string()
    }
}
