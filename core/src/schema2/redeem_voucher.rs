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
#[doc = "`FactionsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Amount\","]
#[doc = "    \"Faction\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Amount\": {"]
#[doc = "      \"title\": \"Amount\","]
#[doc = "      \"examples\": ["]
#[doc = "        1400,"]
#[doc = "        5062,"]
#[doc = "        8800"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"United HIP 20485 Nationalists\","]
#[doc = "        \"\","]
#[doc = "        \"Aristocrats of Latorioson\""]
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
pub struct FactionsItem {
    #[serde(rename = "Amount")]
    pub amount: i64,
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
}
impl ::std::convert::From<&FactionsItem> for FactionsItem {
    fn from(value: &FactionsItem) -> Self {
        value.clone()
    }
}
#[doc = "When Written: when claiming payment for combat bounties and bonds"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when claiming payment for combat bounties and bonds\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Amount\","]
#[doc = "    \"Type\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Amount\": {"]
#[doc = "      \"title\": \"Amount\","]
#[doc = "      \"description\": \"Net amount received, after any broker fee\","]
#[doc = "      \"examples\": ["]
#[doc = "        1400,"]
#[doc = "        5062,"]
#[doc = "        22100"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BrokerPercentage\": {"]
#[doc = "      \"title\": \"BrokerPercentage\","]
#[doc = "      \"examples\": ["]
#[doc = "        25.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"PilotsFederation\","]
#[doc = "        \"Wandrama Purple Council\","]
#[doc = "        \"Materia Auxiliatores\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Factions\": {"]
#[doc = "      \"title\": \"Factions\","]
#[doc = "      \"description\": \"For type bounty\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Amount\","]
#[doc = "          \"Faction\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Amount\": {"]
#[doc = "            \"title\": \"Amount\","]
#[doc = "            \"examples\": ["]
#[doc = "              1400,"]
#[doc = "              5062,"]
#[doc = "              8800"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Faction\": {"]
#[doc = "            \"title\": \"Faction\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"United HIP 20485 Nationalists\","]
#[doc = "              \"\","]
#[doc = "              \"Aristocrats of Latorioson\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"bounty\","]
#[doc = "        \"codex\","]
#[doc = "        \"CombatBond\""]
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
pub struct RedeemVoucher {
    #[doc = "Net amount received, after any broker fee"]
    #[serde(rename = "Amount")]
    pub amount: i64,
    #[serde(
        rename = "BrokerPercentage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub broker_percentage: ::std::option::Option<f64>,
    #[serde(skip_deserializing, default = "RedeemVoucher::event_value")]
    pub event: String,
    #[serde(
        rename = "Faction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction: ::std::option::Option<::std::string::String>,
    #[doc = "For type bounty"]
    #[serde(
        rename = "Factions",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub factions: ::std::vec::Vec<FactionsItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&RedeemVoucher> for RedeemVoucher {
    fn from(value: &RedeemVoucher) -> Self {
        value.clone()
    }
}

impl RedeemVoucher {
    pub fn event_value() -> ::std::string::String {
        "RedeemVoucher".to_string()
    }
}
