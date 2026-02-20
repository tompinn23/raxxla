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
#[doc = "When written: player is awarded a bounty for a kill"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: player is awarded a bounty for a kill\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Target\","]
#[doc = "    \"VictimFaction\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Educated Pilots Association\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PilotName\": {"]
#[doc = "      \"title\": \"PilotName\","]
#[doc = "      \"description\": \"Name of the Pilot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$npc_name_decorate:#name=Oriana Miller;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PilotName_Localised\": {"]
#[doc = "      \"title\": \"PilotName_Localised\","]
#[doc = "      \"description\": \"Name of the Pilot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Oriana Miller\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Reward\": {"]
#[doc = "      \"title\": \"Reward\","]
#[doc = "      \"description\": \"Used instead of Rewards when the bounty is for a skimmer\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Rewards\": {"]
#[doc = "      \"title\": \"Rewards\","]
#[doc = "      \"description\": \"An array of Faction names and the Reward values, as the target can have multiple bounties payable by different factions. Not used for skimmers.(See Reward)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Faction\","]
#[doc = "          \"Reward\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Faction\": {"]
#[doc = "            \"title\": \"Faction\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"United HIP 20485 Nationalists\","]
#[doc = "              \"Aristocrats of Latorioson\","]
#[doc = "              \"Latorioson Vision Ltd\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Reward\": {"]
#[doc = "            \"title\": \"Reward\","]
#[doc = "            \"examples\": ["]
#[doc = "              1400,"]
#[doc = "              8800,"]
#[doc = "              13300"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"SharedWithOthers\": {"]
#[doc = "      \"title\": \"SharedWithOthers\","]
#[doc = "      \"description\": \"if credit for the kill is shared with other players, this has the number of other players involved\","]
#[doc = "      \"examples\": ["]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Target\": {"]
#[doc = "      \"title\": \"Target\","]
#[doc = "      \"description\": \"type of ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"citizensuitai_industrial\","]
#[doc = "        \"citizensuitai_scientific\","]
#[doc = "        \"assaultsuitai_class2\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Target_Localised\": {"]
#[doc = "      \"title\": \"Target_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Technician\","]
#[doc = "        \"Researcher\","]
#[doc = "        \"$AssaultSuitAI_Class1_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TotalReward\": {"]
#[doc = "      \"title\": \"TotalReward\","]
#[doc = "      \"description\": \"Not used for skimmers.\","]
#[doc = "      \"examples\": ["]
#[doc = "        1400,"]
#[doc = "        8800,"]
#[doc = "        13300"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"VictimFaction\": {"]
#[doc = "      \"title\": \"VictimFaction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Pilot Syndicate 4\","]
#[doc = "        \"Leavism's Ending\","]
#[doc = "        \"Galileo Corporation\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"VictimFaction_Localised\": {"]
#[doc = "      \"title\": \"VictimFaction_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Pirates\","]
#[doc = "        \"None\""]
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
pub struct Bounty {
    #[serde(skip_deserializing, default = "Bounty::event_value")]
    pub event: String,
    #[serde(
        rename = "Faction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the Pilot"]
    #[serde(
        rename = "PilotName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pilot_name: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the Pilot"]
    #[serde(
        rename = "PilotName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pilot_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Used instead of Rewards when the bounty is for a skimmer"]
    #[serde(
        rename = "Reward",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reward: ::std::option::Option<i64>,
    #[doc = "An array of Faction names and the Reward values, as the target can have multiple bounties payable by different factions. Not used for skimmers.(See Reward)"]
    #[serde(
        rename = "Rewards",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub rewards: ::std::vec::Vec<RewardsItem>,
    #[doc = "if credit for the kill is shared with other players, this has the number of other players involved"]
    #[serde(
        rename = "SharedWithOthers",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_with_others: ::std::option::Option<i64>,
    #[doc = "type of ship"]
    #[serde(rename = "Target")]
    pub target: ::std::string::String,
    #[serde(
        rename = "Target_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Not used for skimmers."]
    #[serde(
        rename = "TotalReward",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_reward: ::std::option::Option<i64>,
    #[serde(rename = "VictimFaction")]
    pub victim_faction: ::std::string::String,
    #[serde(
        rename = "VictimFaction_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub victim_faction_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Bounty> for Bounty {
    fn from(value: &Bounty) -> Self {
        value.clone()
    }
}
#[doc = "`RewardsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Faction\","]
#[doc = "    \"Reward\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"United HIP 20485 Nationalists\","]
#[doc = "        \"Aristocrats of Latorioson\","]
#[doc = "        \"Latorioson Vision Ltd\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Reward\": {"]
#[doc = "      \"title\": \"Reward\","]
#[doc = "      \"examples\": ["]
#[doc = "        1400,"]
#[doc = "        8800,"]
#[doc = "        13300"]
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
pub struct RewardsItem {
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
    #[serde(rename = "Reward")]
    pub reward: i64,
}
impl ::std::convert::From<&RewardsItem> for RewardsItem {
    fn from(value: &RewardsItem) -> Self {
        value.clone()
    }
}

impl Bounty {
    pub fn event_value() -> ::std::string::String {
        "Bounty".to_string()
    }
}
