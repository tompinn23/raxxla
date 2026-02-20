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
#[doc = "`Interdiction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Interdicted\","]
#[doc = "    \"IsPlayer\","]
#[doc = "    \"Success\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CombatRank\": {"]
#[doc = "      \"title\": \"CombatRank\","]
#[doc = "      \"description\": \"If a player\","]
#[doc = "      \"examples\": ["]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"description\": \"If npc\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Royal Phoenix Corporation\","]
#[doc = "        \"The Ant Hill Mob\","]
#[doc = "        \"LFT 926 Blue Hand Gang\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Interdicted\": {"]
#[doc = "      \"title\": \"Interdicted\","]
#[doc = "      \"description\": \"Victim pilot name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Fred Flintstone\","]
#[doc = "        \"$ShipName_PassengerLiner_Cruise;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Interdicted_Localised\": {"]
#[doc = "      \"title\": \"Interdicted_Localised\","]
#[doc = "      \"description\": \"Victim pilot name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Kreuzfahrtschiff\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"IsPlayer\": {"]
#[doc = "      \"title\": \"IsPlayer\","]
#[doc = "      \"description\": \"Whether player or npc\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Power\": {"]
#[doc = "      \"title\": \"Power\","]
#[doc = "      \"description\": \"If npc is working for a power\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Independent\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Submitted\": {"]
#[doc = "      \"title\": \"Submitted\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Success\": {"]
#[doc = "      \"title\": \"Success\","]
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
pub struct Interdiction {
    #[doc = "If a player"]
    #[serde(
        rename = "CombatRank",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub combat_rank: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "Interdiction::event_value")]
    pub event: String,
    #[doc = "If npc"]
    #[serde(
        rename = "Faction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction: ::std::option::Option<::std::string::String>,
    #[doc = "Victim pilot name"]
    #[serde(rename = "Interdicted")]
    pub interdicted: ::std::string::String,
    #[doc = "Victim pilot name"]
    #[serde(
        rename = "Interdicted_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub interdicted_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Whether player or npc"]
    #[serde(rename = "IsPlayer")]
    pub is_player: bool,
    #[doc = "If npc is working for a power"]
    #[serde(
        rename = "Power",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub power: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Submitted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub submitted: ::std::option::Option<bool>,
    #[serde(rename = "Success")]
    pub success: bool,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Interdiction> for Interdiction {
    fn from(value: &Interdiction) -> Self {
        value.clone()
    }
}

impl Interdiction {
    pub fn event_value() -> ::std::string::String {
        "Interdiction".to_string()
    }
}
