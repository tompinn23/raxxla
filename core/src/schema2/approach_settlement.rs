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
#[doc = "When written: when approaching a planetary settlement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when approaching a planetary settlement\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyID\","]
#[doc = "    \"BodyName\","]
#[doc = "    \"Name\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        35,"]
#[doc = "        40,"]
#[doc = "        38"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyName\": {"]
#[doc = "      \"title\": \"BodyName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485 C 5\","]
#[doc = "        \"Siris 5 c\","]
#[doc = "        \"Vanth\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Latitude\": {"]
#[doc = "      \"title\": \"Latitude\","]
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
#[doc = "      \"examples\": ["]
#[doc = "        -17.250286,"]
#[doc = "        -17.250288,"]
#[doc = "        -44.13446"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 180.0,"]
#[doc = "      \"minimum\": -180.0"]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3814679040,"]
#[doc = "        128973415,"]
#[doc = "        3807125248"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Foster Synthetics Exchange\","]
#[doc = "        \"Nevermore Terrace\","]
#[doc = "        \"Uutoni Extraction Platform\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Ancient Ruins (1)\","]
#[doc = "        \"Ancient Ruins (3)\","]
#[doc = "        \"Ancient Ruins (2)\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationAllegiance\": {"]
#[doc = "      \"title\": \"StationAllegiance\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Federation\","]
#[doc = "        \"Alliance\","]
#[doc = "        \"PilotsFederation\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationEconomies\": {"]
#[doc = "      \"title\": \"StationEconomies\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Name\","]
#[doc = "          \"Proportion\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$economy_Carrier;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Private Enterprise\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Proportion\": {"]
#[doc = "            \"title\": \"Proportion\","]
#[doc = "            \"examples\": ["]
#[doc = "              1.0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"StationEconomy\": {"]
#[doc = "      \"title\": \"StationEconomy\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$economy_Carrier;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationEconomy_Localised\": {"]
#[doc = "      \"title\": \"StationEconomy_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Private Enterprise\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationFaction\": {"]
#[doc = "      \"title\": \"StationFaction\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"FactionState\": {"]
#[doc = "          \"title\": \"FactionState\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Investment\","]
#[doc = "            \"War\","]
#[doc = "            \"Election\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Name\": {"]
#[doc = "          \"title\": \"Name\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"FleetCarrier\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"StationGovernment\": {"]
#[doc = "      \"title\": \"StationGovernment\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$government_Carrier;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationGovernment_Localised\": {"]
#[doc = "      \"title\": \"StationGovernment_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Private Ownership\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationServices\": {"]
#[doc = "      \"title\": \"StationServices\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"dock\","]
#[doc = "          \"autodock\","]
#[doc = "          \"commodities\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        3657265287866,"]
#[doc = "        7269634614689,"]
#[doc = "        10477373803"]
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
pub struct ApproachSettlement {
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "BodyName")]
    pub body_name: ::std::string::String,
    #[serde(skip_deserializing, default = "ApproachSettlement::event_value")]
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
        rename = "MarketID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub market_id: ::std::option::Option<i64>,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StationAllegiance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_allegiance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StationEconomies",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub station_economies: ::std::vec::Vec<StationEconomiesItem>,
    #[serde(
        rename = "StationEconomy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_economy: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StationEconomy_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_economy_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StationFaction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_faction: ::std::option::Option<StationFaction>,
    #[serde(
        rename = "StationGovernment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_government: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StationGovernment_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_government_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StationServices",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub station_services: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ApproachSettlement> for ApproachSettlement {
    fn from(value: &ApproachSettlement) -> Self {
        value.clone()
    }
}
#[doc = "`StationEconomiesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"Proportion\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$economy_Carrier;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Private Enterprise\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Proportion\": {"]
#[doc = "      \"title\": \"Proportion\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StationEconomiesItem {
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Proportion")]
    pub proportion: f64,
}
impl ::std::convert::From<&StationEconomiesItem> for StationEconomiesItem {
    fn from(value: &StationEconomiesItem) -> Self {
        value.clone()
    }
}
#[doc = "`StationFaction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"StationFaction\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"FactionState\": {"]
#[doc = "      \"title\": \"FactionState\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Investment\","]
#[doc = "        \"War\","]
#[doc = "        \"Election\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FleetCarrier\""]
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
pub struct StationFaction {
    #[serde(
        rename = "FactionState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
}
impl ::std::convert::From<&StationFaction> for StationFaction {
    fn from(value: &StationFaction) -> Self {
        value.clone()
    }
}

impl ApproachSettlement {
    pub fn event_value() -> ::std::string::String {
        "ApproachSettlement".to_string()
    }
}
