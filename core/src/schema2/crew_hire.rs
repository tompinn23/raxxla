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
#[doc = "When written: when engaging a new member of crew"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when engaging a new member of crew\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CombatRank\","]
#[doc = "    \"Cost\","]
#[doc = "    \"CrewID\","]
#[doc = "    \"Faction\","]
#[doc = "    \"Name\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CombatRank\": {"]
#[doc = "      \"title\": \"CombatRank\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        4,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Cost\": {"]
#[doc = "      \"title\": \"Cost\","]
#[doc = "      \"examples\": ["]
#[doc = "        15000,"]
#[doc = "        150000,"]
#[doc = "        55000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CrewID\": {"]
#[doc = "      \"title\": \"CrewID\","]
#[doc = "      \"examples\": ["]
#[doc = "        216828288,"]
#[doc = "        218204548,"]
#[doc = "        230047520"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"LTT 4487 Industry\","]
#[doc = "        \"League of Azaladshu Defence Party\","]
#[doc = "        \"Neche Inc\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Alice\","]
#[doc = "        \"Bob\","]
#[doc = "        \"Mallory\""]
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
pub struct CrewHire {
    #[serde(rename = "CombatRank")]
    pub combat_rank: i64,
    #[serde(rename = "Cost")]
    pub cost: i64,
    #[serde(rename = "CrewID")]
    pub crew_id: i64,
    #[serde(skip_deserializing, default = "CrewHire::event_value")]
    pub event: String,
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CrewHire> for CrewHire {
    fn from(value: &CrewHire) -> Self {
        value.clone()
    }
}

impl CrewHire {
    pub fn event_value() -> ::std::string::String {
        "CrewHire".to_string()
    }
}
