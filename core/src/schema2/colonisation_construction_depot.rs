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
#[doc = "When Written: Every 15 seconds while docked at a construction depot"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: Every 15 seconds while docked at a construction depot\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ConstructionComplete\","]
#[doc = "    \"ConstructionFailed\","]
#[doc = "    \"ConstructionProgress\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"ResourcesRequired\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ConstructionComplete\": {"]
#[doc = "      \"title\": \"ConstructionComplete\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"ConstructionFailed\": {"]
#[doc = "      \"title\": \"ConstructionFailed\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"ConstructionProgress\": {"]
#[doc = "      \"title\": \"ConstructionProgress\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.141462"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        4217038595"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ResourcesRequired\": {"]
#[doc = "      \"title\": \"ResourcesRequired\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Name\","]
#[doc = "          \"Name_Localised\","]
#[doc = "          \"Payment\","]
#[doc = "          \"ProvidedAmount\","]
#[doc = "          \"RequiredAmount\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$aluminium_name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Aluminium\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Payment\": {"]
#[doc = "            \"title\": \"Payment\","]
#[doc = "            \"examples\": ["]
#[doc = "              3239"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"ProvidedAmount\": {"]
#[doc = "            \"title\": \"ProvidedAmount\","]
#[doc = "            \"examples\": ["]
#[doc = "              621"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"RequiredAmount\": {"]
#[doc = "            \"title\": \"RequiredAmount\","]
#[doc = "            \"examples\": ["]
#[doc = "              43618"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
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
pub struct ColonisationConstructionDepot {
    #[serde(rename = "ConstructionComplete")]
    pub construction_complete: bool,
    #[serde(rename = "ConstructionFailed")]
    pub construction_failed: bool,
    #[serde(rename = "ConstructionProgress")]
    pub construction_progress: f64,
    #[serde(
        skip_deserializing,
        default = "ColonisationConstructionDepot::event_value"
    )]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "ResourcesRequired")]
    pub resources_required: ::std::vec::Vec<ResourcesRequiredItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ColonisationConstructionDepot> for ColonisationConstructionDepot {
    fn from(value: &ColonisationConstructionDepot) -> Self {
        value.clone()
    }
}
#[doc = "`ResourcesRequiredItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"Name_Localised\","]
#[doc = "    \"Payment\","]
#[doc = "    \"ProvidedAmount\","]
#[doc = "    \"RequiredAmount\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$aluminium_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Aluminium\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Payment\": {"]
#[doc = "      \"title\": \"Payment\","]
#[doc = "      \"examples\": ["]
#[doc = "        3239"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ProvidedAmount\": {"]
#[doc = "      \"title\": \"ProvidedAmount\","]
#[doc = "      \"examples\": ["]
#[doc = "        621"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"RequiredAmount\": {"]
#[doc = "      \"title\": \"RequiredAmount\","]
#[doc = "      \"examples\": ["]
#[doc = "        43618"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourcesRequiredItem {
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: ::std::string::String,
    #[serde(rename = "Payment")]
    pub payment: i64,
    #[serde(rename = "ProvidedAmount")]
    pub provided_amount: i64,
    #[serde(rename = "RequiredAmount")]
    pub required_amount: i64,
}
impl ::std::convert::From<&ResourcesRequiredItem> for ResourcesRequiredItem {
    fn from(value: &ResourcesRequiredItem) -> Self {
        value.clone()
    }
}

impl ColonisationConstructionDepot {
    pub fn event_value() -> ::std::string::String {
        "ColonisationConstructionDepot".to_string()
    }
}
