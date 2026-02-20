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
#[doc = "When written: when a crime is recorded against the player"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when a crime is recorded against the player\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CrimeType\","]
#[doc = "    \"Faction\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Bounty\": {"]
#[doc = "      \"title\": \"Bounty\","]
#[doc = "      \"examples\": ["]
#[doc = "        1000,"]
#[doc = "        1500,"]
#[doc = "        2000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CrimeType\": {"]
#[doc = "      \"title\": \"CrimeType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"onFoot_murder\","]
#[doc = "        \"onFoot_recklessEndangerment\","]
#[doc = "        \"onFoot_detectionOfWeapon\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Pilot Syndicate 4\","]
#[doc = "        \"United Latorioson Labour\","]
#[doc = "        \"Galileo Corporation\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Fine\": {"]
#[doc = "      \"title\": \"Fine\","]
#[doc = "      \"examples\": ["]
#[doc = "        250,"]
#[doc = "        500,"]
#[doc = "        100"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Victim\": {"]
#[doc = "      \"title\": \"Victim\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Zara Alford\","]
#[doc = "        \"Blaine Ayers\","]
#[doc = "        \"Johnson Witt\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Victim_Localised\": {"]
#[doc = "      \"title\": \"Victim_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Charge Port\","]
#[doc = "        \"Sentry Skimmer\","]
#[doc = "        \"Ground Turret\""]
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
pub struct CommitCrime {
    #[serde(
        rename = "Bounty",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bounty: ::std::option::Option<i64>,
    #[serde(rename = "CrimeType")]
    pub crime_type: ::std::string::String,
    #[serde(skip_deserializing, default = "CommitCrime::event_value")]
    pub event: String,
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
    #[serde(
        rename = "Fine",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fine: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "Victim",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub victim: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Victim_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub victim_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CommitCrime> for CommitCrime {
    fn from(value: &CommitCrime) -> Self {
        value.clone()
    }
}

impl CommitCrime {
    pub fn event_value() -> ::std::string::String {
        "CommitCrime".to_string()
    }
}
