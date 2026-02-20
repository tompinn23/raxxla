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
#[doc = "When written: when zooming in on a signal using the FSS scanner"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when zooming in on a signal using the FSS scanner\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"SignalName\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"IsStation\": {"]
#[doc = "      \"title\": \"IsStation\","]
#[doc = "      \"description\": \"If it is a station\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"OpposingPower\": {"]
#[doc = "      \"title\": \"OpposingPower\","]
#[doc = "      \"description\": \"the opposing power, if relevant\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Archon Delaine\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SignalName\": {"]
#[doc = "      \"title\": \"SignalName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$MULTIPLAYER_SCENARIO42_TITLE;\","]
#[doc = "        \"$ListeningPost:#index=1;\","]
#[doc = "        \"BLUE TYPHOON Q1M-83F\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SignalName_Localised\": {"]
#[doc = "      \"title\": \"SignalName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Nav Beacon\","]
#[doc = "        \"Listening Post\","]
#[doc = "        \"Conflict Zone [Low Intensity]\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SignalType\": {"]
#[doc = "      \"title\": \"SignalType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"USS\","]
#[doc = "        \"TouristBeacon\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SpawningFaction\": {"]
#[doc = "      \"title\": \"SpawningFaction\","]
#[doc = "      \"description\": \"the minor faction, if relevant\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$faction_none;\","]
#[doc = "        \"I.P.S.A.L Project\","]
#[doc = "        \"Janus Incorporated\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SpawningFaction_Localised\": {"]
#[doc = "      \"title\": \"SpawningFaction_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"None\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SpawningPower\": {"]
#[doc = "      \"title\": \"SpawningPower\","]
#[doc = "      \"description\": \"the spawning power, if relevant\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Archon Delaine\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SpawningState\": {"]
#[doc = "      \"title\": \"SpawningState\","]
#[doc = "      \"description\": \"the BGS state that triggered this event, if relevant\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$FactionState_None;\","]
#[doc = "        \"$FactionState_CivilUnrest_desc;\","]
#[doc = "        \"\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SpawningState_Localised\": {"]
#[doc = "      \"title\": \"SpawningState_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"None\","]
#[doc = "        \"A period of civil unrest represents a failure in local security leading to riots, looting and terrorist activity.\","]
#[doc = "        \"Expansion represents a factions intention to spread into a nearby system in the near future.\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        198875014308,"]
#[doc = "        3657265287866,"]
#[doc = "        6955800204002"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ThreatLevel\": {"]
#[doc = "      \"title\": \"ThreatLevel\","]
#[doc = "      \"description\": \"If a Unidentified Signal Source(USS) Event\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        5,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TimeRemaining\": {"]
#[doc = "      \"title\": \"TimeRemaining\","]
#[doc = "      \"description\": \"remaining lifetime in seconds, if relevant\","]
#[doc = "      \"examples\": ["]
#[doc = "        342.042999,"]
#[doc = "        95.250122,"]
#[doc = "        1762.713013"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"USSType\": {"]
#[doc = "      \"title\": \"USSType\","]
#[doc = "      \"description\": \"If a Unidentified Signal Source(USS) Event\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$USS_Type_Salvage;\","]
#[doc = "        \"$USS_Type_NonHuman;\","]
#[doc = "        \"$USS_Type_WeaponsFire;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"USSType_Localised\": {"]
#[doc = "      \"title\": \"USSType_Localised\","]
#[doc = "      \"description\": \"If a Unidentified Signal Source(USS) Event\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Degraded emissions\","]
#[doc = "        \"Nonhuman signal source\","]
#[doc = "        \"Weapons fire\""]
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
pub struct FSSSignalDiscovered {
    #[serde(skip_deserializing, default = "FSSSignalDiscovered::event_value")]
    pub event: String,
    #[doc = "If it is a station"]
    #[serde(
        rename = "IsStation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_station: ::std::option::Option<bool>,
    #[doc = "the opposing power, if relevant"]
    #[serde(
        rename = "OpposingPower",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub opposing_power: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SignalName")]
    pub signal_name: ::std::string::String,
    #[serde(
        rename = "SignalName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub signal_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SignalType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub signal_type: ::std::option::Option<::std::string::String>,
    #[doc = "the minor faction, if relevant"]
    #[serde(
        rename = "SpawningFaction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub spawning_faction: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SpawningFaction_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub spawning_faction_localised: ::std::option::Option<::std::string::String>,
    #[doc = "the spawning power, if relevant"]
    #[serde(
        rename = "SpawningPower",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub spawning_power: ::std::option::Option<::std::string::String>,
    #[doc = "the BGS state that triggered this event, if relevant"]
    #[serde(
        rename = "SpawningState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub spawning_state: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SpawningState_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub spawning_state_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "If a Unidentified Signal Source(USS) Event"]
    #[serde(
        rename = "ThreatLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub threat_level: ::std::option::Option<i64>,
    #[serde(
        rename = "TimeRemaining",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_remaining: ::std::option::Option<f64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "If a Unidentified Signal Source(USS) Event"]
    #[serde(
        rename = "USSType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub uss_type: ::std::option::Option<::std::string::String>,
    #[doc = "If a Unidentified Signal Source(USS) Event"]
    #[serde(
        rename = "USSType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub uss_type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FSSSignalDiscovered> for FSSSignalDiscovered {
    fn from(value: &FSSSignalDiscovered) -> Self {
        value.clone()
    }
}

impl FSSSignalDiscovered {
    pub fn event_value() -> ::std::string::String {
        "FSSSignalDiscovered".to_string()
    }
}
