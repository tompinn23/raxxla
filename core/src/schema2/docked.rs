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
#[doc = "When written: when landing at landing pad in a space station, outpost, or surface settlement"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when landing at landing pad in a space station, outpost, or surface settlement\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"DistFromStarLS\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"StationGovernment\","]
#[doc = "    \"StationName\","]
#[doc = "    \"StationServices\","]
#[doc = "    \"StationType\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ActiveFine\": {"]
#[doc = "      \"title\": \"ActiveFine\","]
#[doc = "      \"description\": \"Only if any fine is active\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"CockpitBreach\": {"]
#[doc = "      \"title\": \"CockpitBreach\","]
#[doc = "      \"description\": \"true (only if landing with breached cockpit)\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"DistFromStarLS\": {"]
#[doc = "      \"title\": \"DistFromStarLS\","]
#[doc = "      \"examples\": ["]
#[doc = "        8370.582637,"]
#[doc = "        0.0,"]
#[doc = "        11.433206"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"LandingPads\": {"]
#[doc = "      \"title\": \"LandingPads\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Large\","]
#[doc = "        \"Medium\","]
#[doc = "        \"Small\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Large\": {"]
#[doc = "          \"title\": \"Large\","]
#[doc = "          \"examples\": ["]
#[doc = "            8,"]
#[doc = "            9,"]
#[doc = "            2"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"Medium\": {"]
#[doc = "          \"title\": \"Medium\","]
#[doc = "          \"examples\": ["]
#[doc = "            4,"]
#[doc = "            18,"]
#[doc = "            1"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"Small\": {"]
#[doc = "          \"title\": \"Small\","]
#[doc = "          \"examples\": ["]
#[doc = "            4,"]
#[doc = "            17,"]
#[doc = "            2"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
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
#[doc = "    \"Multicrew\": {"]
#[doc = "      \"title\": \"Multicrew\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
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
#[doc = "      \"description\": \"Only if station has multiple economies\","]
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
#[doc = "              \"$economy_Carrier;\","]
#[doc = "              \"$economy_Industrial;\","]
#[doc = "              \"$economy_Extraction;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Private Enterprise\","]
#[doc = "              \"Industrial\","]
#[doc = "              \"Extraction\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Proportion\": {"]
#[doc = "            \"title\": \"Proportion\","]
#[doc = "            \"examples\": ["]
#[doc = "              1.0,"]
#[doc = "              0.87,"]
#[doc = "              0.13"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"StationEconomy\": {"]
#[doc = "      \"title\": \"StationEconomy\","]
#[doc = "      \"description\": \"Only if station has a single economy\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$economy_Carrier;\","]
#[doc = "        \"$economy_Industrial;\","]
#[doc = "        \"$economy_Colony;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationEconomy_Localised\": {"]
#[doc = "      \"title\": \"StationEconomy_Localised\","]
#[doc = "      \"description\": \"Only if station has a single economy\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Private Enterprise\","]
#[doc = "        \"Industrial\","]
#[doc = "        \"Colony\""]
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
#[doc = "            \"Expansion\","]
#[doc = "            \"Boom\","]
#[doc = "            \"Investment\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Name\": {"]
#[doc = "          \"title\": \"Name\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"FleetCarrier\","]
#[doc = "            \"Pilot Syndicate 4\","]
#[doc = "            \"Hero Ferrari\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"StationGovernment\": {"]
#[doc = "      \"title\": \"StationGovernment\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$government_Carrier;\","]
#[doc = "        \"$government_Corporate;\","]
#[doc = "        \"$government_Engineer;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationGovernment_Localised\": {"]
#[doc = "      \"title\": \"StationGovernment_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Private Ownership\","]
#[doc = "        \"Corporate\","]
#[doc = "        \"Workshop\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
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
#[doc = "    \"StationServices\": {"]
#[doc = "      \"title\": \"StationServices\","]
#[doc = "      \"description\": \"StationServices can include: Dock, Autodock, BlackMarket, Commodities, Contacts, Exploration, Initiatives, Missions, Outfitting, CrewLounge, Rearm, Refuel, Repair, Shipyard, Tuning, Workshop, MissionsGenerated, Facilitator, Research, FlightController, StationOperations, OnDockMission, Powerplay, SearchAndRescue. New in v3.7: shop, carriermanagement, carrierfuel, carriervendor, livery, modulepacks, voucherredemption\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"dock\","]
#[doc = "          \"autodock\","]
#[doc = "          \"blackmarket\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"StationState\": {"]
#[doc = "      \"title\": \"StationState\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Damaged\","]
#[doc = "        \"UnderRepairs\""]
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
#[doc = "    \"Wanted\": {"]
#[doc = "      \"title\": \"Wanted\","]
#[doc = "      \"description\": \"Only if docking when wanted locally\","]
#[doc = "      \"examples\": ["]
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
pub struct Docked {
    #[doc = "Only if any fine is active"]
    #[serde(
        rename = "ActiveFine",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub active_fine: ::std::option::Option<bool>,
    #[doc = "true (only if landing with breached cockpit)"]
    #[serde(
        rename = "CockpitBreach",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cockpit_breach: ::std::option::Option<bool>,
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,
    #[serde(skip_deserializing, default = "Docked::event_value")]
    pub event: String,
    #[serde(
        rename = "LandingPads",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub landing_pads: ::std::option::Option<LandingPads>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(
        rename = "Multicrew",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub multicrew: ::std::option::Option<bool>,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(
        rename = "StationAllegiance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_allegiance: ::std::option::Option<::std::string::String>,
    #[doc = "Only if station has multiple economies"]
    #[serde(
        rename = "StationEconomies",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub station_economies: ::std::vec::Vec<StationEconomiesItem>,
    #[doc = "Only if station has a single economy"]
    #[serde(
        rename = "StationEconomy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_economy: ::std::option::Option<::std::string::String>,
    #[doc = "Only if station has a single economy"]
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
    #[serde(rename = "StationGovernment")]
    pub station_government: ::std::string::String,
    #[serde(
        rename = "StationGovernment_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_government_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "StationName")]
    pub station_name: ::std::string::String,
    #[serde(
        rename = "StationName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "StationServices can include: Dock, Autodock, BlackMarket, Commodities, Contacts, Exploration, Initiatives, Missions, Outfitting, CrewLounge, Rearm, Refuel, Repair, Shipyard, Tuning, Workshop, MissionsGenerated, Facilitator, Research, FlightController, StationOperations, OnDockMission, Powerplay, SearchAndRescue. New in v3.7: shop, carriermanagement, carrierfuel, carriervendor, livery, modulepacks, voucherredemption"]
    #[serde(rename = "StationServices")]
    pub station_services: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "StationState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub station_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "StationType")]
    pub station_type: ::std::string::String,
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
    #[doc = "Only if docking when wanted locally"]
    #[serde(
        rename = "Wanted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wanted: ::std::option::Option<bool>,
}
impl ::std::convert::From<&Docked> for Docked {
    fn from(value: &Docked) -> Self {
        value.clone()
    }
}
#[doc = "`LandingPads`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"LandingPads\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Large\","]
#[doc = "    \"Medium\","]
#[doc = "    \"Small\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Large\": {"]
#[doc = "      \"title\": \"Large\","]
#[doc = "      \"examples\": ["]
#[doc = "        8,"]
#[doc = "        9,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Medium\": {"]
#[doc = "      \"title\": \"Medium\","]
#[doc = "      \"examples\": ["]
#[doc = "        4,"]
#[doc = "        18,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Small\": {"]
#[doc = "      \"title\": \"Small\","]
#[doc = "      \"examples\": ["]
#[doc = "        4,"]
#[doc = "        17,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LandingPads {
    #[serde(rename = "Large")]
    pub large: i64,
    #[serde(rename = "Medium")]
    pub medium: i64,
    #[serde(rename = "Small")]
    pub small: i64,
}
impl ::std::convert::From<&LandingPads> for LandingPads {
    fn from(value: &LandingPads) -> Self {
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
#[doc = "        \"$economy_Carrier;\","]
#[doc = "        \"$economy_Industrial;\","]
#[doc = "        \"$economy_Extraction;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Private Enterprise\","]
#[doc = "        \"Industrial\","]
#[doc = "        \"Extraction\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Proportion\": {"]
#[doc = "      \"title\": \"Proportion\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0,"]
#[doc = "        0.87,"]
#[doc = "        0.13"]
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
#[doc = "        \"Expansion\","]
#[doc = "        \"Boom\","]
#[doc = "        \"Investment\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FleetCarrier\","]
#[doc = "        \"Pilot Syndicate 4\","]
#[doc = "        \"Hero Ferrari\""]
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

impl Docked {
    pub fn event_value() -> ::std::string::String {
        "Docked".to_string()
    }
}
