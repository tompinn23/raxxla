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
#[doc = "When Written: when starting a mission"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when starting a mission\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Faction\","]
#[doc = "    \"Influence\","]
#[doc = "    \"LocalisedName\","]
#[doc = "    \"MissionID\","]
#[doc = "    \"Name\","]
#[doc = "    \"Reputation\","]
#[doc = "    \"Wing\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Commodity\": {"]
#[doc = "      \"title\": \"Commodity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$SurveilleanceLogs_Name;\","]
#[doc = "        \"$MedicalRecords_Name;\","]
#[doc = "        \"$EmployeeGeneticData_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Commodity_Localised\": {"]
#[doc = "      \"title\": \"Commodity_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Surveillance Logs\","]
#[doc = "        \"Medical Records\","]
#[doc = "        \"Employee Genetic Data\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"description\": \"Number required to deliver\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        202,"]
#[doc = "        195"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"DestinationSettlement\": {"]
#[doc = "      \"title\": \"DestinationSettlement\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Skoropadsky Boarding Site\","]
#[doc = "        \"Cataldo Excavation Hub\","]
#[doc = "        \"Degefa Extraction Platform\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"DestinationStation\": {"]
#[doc = "      \"title\": \"DestinationStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Ryazanski Dock\","]
#[doc = "        \"Jameson Memorial\","]
#[doc = "        \"Ockels Relay\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"DestinationSystem\": {"]
#[doc = "      \"title\": \"DestinationSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Nuenets\","]
#[doc = "        \"Shinrarta Dezhra\","]
#[doc = "        \"LDS 413\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Donation\": {"]
#[doc = "      \"title\": \"Donation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"575000\","]
#[doc = "        \"1000000\","]
#[doc = "        \"450000\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Expiry\": {"]
#[doc = "      \"title\": \"Expiry\","]
#[doc = "      \"description\": \"Mission expiry time, in ISO 8601\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"2022-05-22T17:15:36Z\","]
#[doc = "        \"2022-05-22T17:27:50Z\","]
#[doc = "        \"2022-05-22T17:26:20Z\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"description\": \"Faction offering mission\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Leavism's Ending\","]
#[doc = "        \"Simbad Regime\","]
#[doc = "        \"Latorioson Blue Universal Ex\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Influence\": {"]
#[doc = "      \"title\": \"Influence\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"++\","]
#[doc = "        \"+\","]
#[doc = "        \"None\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"KillCount\": {"]
#[doc = "      \"title\": \"KillCount\","]
#[doc = "      \"description\": \"Number of targets\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        13,"]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LocalisedName\": {"]
#[doc = "      \"title\": \"LocalisedName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Download Surveillance Logs from a data port\","]
#[doc = "        \"Download Medical Records from a data port\","]
#[doc = "        \"Download Employee Genetic Data from a data port\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        868354915,"]
#[doc = "        868354949,"]
#[doc = "        868354970"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Mission_OnFoot_Hack_Download_Covert_MB\","]
#[doc = "        \"Mission_OnFoot_Hack_Download_MB\","]
#[doc = "        \"Mission_OnFoot_Sabotage_Power_MB\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewDestinationStation\": {"]
#[doc = "      \"title\": \"NewDestinationStation\","]
#[doc = "      \"description\": \"If it has been redirected\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Ryazanski Dock\","]
#[doc = "        \"Jameson Memorial\","]
#[doc = "        \"Ockels Relay\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewDestinationSystem\": {"]
#[doc = "      \"title\": \"NewDestinationSystem\","]
#[doc = "      \"description\": \"If it has been redirected\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Nuenets\","]
#[doc = "        \"Shinrarta Dezhra\","]
#[doc = "        \"LDS 413\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PassengerCount\": {"]
#[doc = "      \"title\": \"PassengerCount\","]
#[doc = "      \"examples\": ["]
#[doc = "        4,"]
#[doc = "        3,"]
#[doc = "        16"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"PassengerType\": {"]
#[doc = "      \"title\": \"PassengerType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Tourist\","]
#[doc = "        \"Criminal\","]
#[doc = "        \"Explorer\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PassengerVIPs\": {"]
#[doc = "      \"title\": \"PassengerVIPs\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"PassengerWanted\": {"]
#[doc = "      \"title\": \"PassengerWanted\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Reputation\": {"]
#[doc = "      \"title\": \"Reputation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"++\","]
#[doc = "        \"+\","]
#[doc = "        \"None\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Reward\": {"]
#[doc = "      \"title\": \"Reward\","]
#[doc = "      \"examples\": ["]
#[doc = "        298488,"]
#[doc = "        189591,"]
#[doc = "        320312"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Target\": {"]
#[doc = "      \"title\": \"Target\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Jeremy Rayner\","]
#[doc = "        \"$MissionUtil_FactionTag_Datalink;\","]
#[doc = "        \"Karissa Kennedy\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetFaction\": {"]
#[doc = "      \"title\": \"TargetFaction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Nuenets Blue Crew\","]
#[doc = "        \"Kipsigines Vision Partners\","]
#[doc = "        \"44th Vulture Syndicate\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetType\": {"]
#[doc = "      \"title\": \"TargetType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$MissionUtil_FactionTag_PirateLord;\","]
#[doc = "        \"$MissionUtil_FactionTag_Skimmer;\","]
#[doc = "        \"$MissionUtil_FactionTag_AIHumanoid;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetType_Localised\": {"]
#[doc = "      \"title\": \"TargetType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Known Pirate\","]
#[doc = "        \"Sentry Skimmer\","]
#[doc = "        \"Faction Members\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Target_Localised\": {"]
#[doc = "      \"title\": \"Target_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Data Link\","]
#[doc = "        \"Hub Access Terminal\","]
#[doc = "        \"Turret\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Wing\": {"]
#[doc = "      \"title\": \"Wing\","]
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
pub struct MissionAccepted {
    #[serde(
        rename = "Commodity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Commodity_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Number required to deliver"]
    #[serde(
        rename = "Count",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub count: ::std::option::Option<i64>,
    #[serde(
        rename = "DestinationSettlement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination_settlement: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "DestinationStation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination_station: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "DestinationSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination_system: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Donation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub donation: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "MissionAccepted::event_value")]
    pub event: String,
    #[doc = "Mission expiry time, in ISO 8601"]
    #[serde(
        rename = "Expiry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expiry: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Faction offering mission"]
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
    #[serde(rename = "Influence")]
    pub influence: ::std::string::String,
    #[doc = "Number of targets"]
    #[serde(
        rename = "KillCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub kill_count: ::std::option::Option<i64>,
    #[serde(rename = "LocalisedName")]
    pub localised_name: ::std::string::String,
    #[serde(rename = "MissionID")]
    pub mission_id: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[doc = "If it has been redirected"]
    #[serde(
        rename = "NewDestinationStation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_destination_station: ::std::option::Option<::std::string::String>,
    #[doc = "If it has been redirected"]
    #[serde(
        rename = "NewDestinationSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_destination_system: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "PassengerCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub passenger_count: ::std::option::Option<i64>,
    #[serde(
        rename = "PassengerType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub passenger_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "PassengerVIPs",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub passenger_vi_ps: ::std::option::Option<bool>,
    #[serde(
        rename = "PassengerWanted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub passenger_wanted: ::std::option::Option<bool>,
    #[serde(rename = "Reputation")]
    pub reputation: ::std::string::String,
    #[serde(
        rename = "Reward",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reward: ::std::option::Option<i64>,
    #[serde(
        rename = "Target",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "TargetFaction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_faction: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Target_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "TargetType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "TargetType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Wing")]
    pub wing: bool,
}
impl ::std::convert::From<&MissionAccepted> for MissionAccepted {
    fn from(value: &MissionAccepted) -> Self {
        value.clone()
    }
}

impl MissionAccepted {
    pub fn event_value() -> ::std::string::String {
        "MissionAccepted".to_string()
    }
}
