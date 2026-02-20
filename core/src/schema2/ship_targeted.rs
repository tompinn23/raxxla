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
#[doc = "When written: when the current player selects a new target"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when the current player selects a new target\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"TargetLocked\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Bounty\": {"]
#[doc = "      \"title\": \"Bounty\","]
#[doc = "      \"description\": \"If Scan stage >= 3\","]
#[doc = "      \"examples\": ["]
#[doc = "        87714,"]
#[doc = "        805840,"]
#[doc = "        290251"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"description\": \"If Scan stage >= 3\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Pilot Syndicate 4\","]
#[doc = "        \"Sol Workers' Party\","]
#[doc = "        \"V886 Centauri Future\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"HullHealth\": {"]
#[doc = "      \"title\": \"HullHealth\","]
#[doc = "      \"description\": \"If Scan stage >= 2\","]
#[doc = "      \"examples\": ["]
#[doc = "        100.0,"]
#[doc = "        99.974396,"]
#[doc = "        18.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"LegalStatus\": {"]
#[doc = "      \"title\": \"LegalStatus\","]
#[doc = "      \"description\": \"If Scan stage >= 3\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Clean\","]
#[doc = "        \"Lawless\","]
#[doc = "        \"Hunter\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PilotName\": {"]
#[doc = "      \"title\": \"PilotName\","]
#[doc = "      \"description\": \"If Scan stage >= 1\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$ShipName_Police_Independent;\","]
#[doc = "        \"$ShipName_Police_Federation;\","]
#[doc = "        \"$npc_name_decorate:#name=GutBuster;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PilotName_Localised\": {"]
#[doc = "      \"title\": \"PilotName_Localised\","]
#[doc = "      \"description\": \"If Scan stage >= 1\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"System Authority Vessel\","]
#[doc = "        \"Federal Security Service\","]
#[doc = "        \"GutBuster\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PilotRank\": {"]
#[doc = "      \"title\": \"PilotRank\","]
#[doc = "      \"description\": \"If Scan stage >= 1\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Expert\","]
#[doc = "        \"Competent\","]
#[doc = "        \"Dangerous\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Power\": {"]
#[doc = "      \"title\": \"Power\","]
#[doc = "      \"description\": \"If the player is aligned in powerplay and the target is also aligned to a power\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Li Yong-Rui\","]
#[doc = "        \"Aisling Duval\","]
#[doc = "        \"Edmund Mahon\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ScanStage\": {"]
#[doc = "      \"title\": \"ScanStage\","]
#[doc = "      \"examples\": ["]
#[doc = "        3,"]
#[doc = "        0,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShieldHealth\": {"]
#[doc = "      \"title\": \"ShieldHealth\","]
#[doc = "      \"description\": \"If Scan stage >= 2\","]
#[doc = "      \"examples\": ["]
#[doc = "        100.0,"]
#[doc = "        82.548676,"]
#[doc = "        82.78862"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"anaconda\","]
#[doc = "        \"viper_mkiv\","]
#[doc = "        \"krait_mkii\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Ship_Localised\": {"]
#[doc = "      \"title\": \"Ship_Localised\","]
#[doc = "      \"description\": \"If target locked\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Viper Mk IV\","]
#[doc = "        \"Krait Mk II\","]
#[doc = "        \"$VIPER_NAME;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SquadronID\": {"]
#[doc = "      \"title\": \"SquadronID\","]
#[doc = "      \"description\": \"If target in a squadron\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"ODMH\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Subsystem\": {"]
#[doc = "      \"title\": \"Subsystem\","]
#[doc = "      \"description\": \"If Scan stage >= 3\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$modularcargobaydoor_name;\","]
#[doc = "        \"$int_dronecontrol_decontamination_size1_class1_name;\","]
#[doc = "        \"$ext_drive_class7_a_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SubsystemHealth\": {"]
#[doc = "      \"title\": \"SubsystemHealth\","]
#[doc = "      \"description\": \"If Scan stage >= 3\","]
#[doc = "      \"examples\": ["]
#[doc = "        100.0,"]
#[doc = "        98.854958,"]
#[doc = "        98.80484"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Subsystem_Localised\": {"]
#[doc = "      \"title\": \"Subsystem_Localised\","]
#[doc = "      \"description\": \"If Scan stage >= 3\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Cargo Hatch\","]
#[doc = "        \"Decontamination\","]
#[doc = "        \"Drive\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetLocked\": {"]
#[doc = "      \"title\": \"TargetLocked\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
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
pub struct ShipTargeted {
    #[doc = "If Scan stage >= 3"]
    #[serde(
        rename = "Bounty",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bounty: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "ShipTargeted::event_value")]
    pub event: String,
    #[doc = "If Scan stage >= 3"]
    #[serde(
        rename = "Faction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "HullHealth",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hull_health: ::std::option::Option<f64>,
    #[doc = "If Scan stage >= 3"]
    #[serde(
        rename = "LegalStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_status: ::std::option::Option<::std::string::String>,
    #[doc = "If Scan stage >= 1"]
    #[serde(
        rename = "PilotName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pilot_name: ::std::option::Option<::std::string::String>,
    #[doc = "If Scan stage >= 1"]
    #[serde(
        rename = "PilotName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pilot_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "If Scan stage >= 1"]
    #[serde(
        rename = "PilotRank",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pilot_rank: ::std::option::Option<::std::string::String>,
    #[doc = "If the player is aligned in powerplay and the target is also aligned to a power"]
    #[serde(
        rename = "Power",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub power: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ScanStage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scan_stage: ::std::option::Option<i64>,
    #[serde(
        rename = "ShieldHealth",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shield_health: ::std::option::Option<f64>,
    #[serde(
        rename = "Ship",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship: ::std::option::Option<::std::string::String>,
    #[doc = "If target locked"]
    #[serde(
        rename = "Ship_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_localised: ::std::option::Option<::std::string::String>,
    #[doc = "If target in a squadron"]
    #[serde(
        rename = "SquadronID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub squadron_id: ::std::option::Option<::std::string::String>,
    #[doc = "If Scan stage >= 3"]
    #[serde(
        rename = "Subsystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subsystem: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SubsystemHealth",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subsystem_health: ::std::option::Option<f64>,
    #[doc = "If Scan stage >= 3"]
    #[serde(
        rename = "Subsystem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subsystem_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "TargetLocked")]
    pub target_locked: bool,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ShipTargeted> for ShipTargeted {
    fn from(value: &ShipTargeted) -> Self {
        value.clone()
    }
}

impl ShipTargeted {
    pub fn event_value() -> ::std::string::String {
        "ShipTargeted".to_string()
    }
}
