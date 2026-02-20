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
#[doc = "When Written: when selling exploration data in Cartographics"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when selling exploration data in Cartographics\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BaseValue\","]
#[doc = "    \"Bonus\","]
#[doc = "    \"Discovered\","]
#[doc = "    \"Systems\","]
#[doc = "    \"TotalEarnings\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BaseValue\": {"]
#[doc = "      \"title\": \"BaseValue\","]
#[doc = "      \"examples\": ["]
#[doc = "        8701,"]
#[doc = "        4539,"]
#[doc = "        2387"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Bonus\": {"]
#[doc = "      \"title\": \"Bonus\","]
#[doc = "      \"description\": \"Bonus for first discoveries\","]
#[doc = "      \"examples\": ["]
#[doc = "        6489,"]
#[doc = "        0,"]
#[doc = "        1913"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Discovered\": {"]
#[doc = "      \"title\": \"Discovered\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"Phraa Blao SO-O d7-28 A\","]
#[doc = "          \"Phraa Blao SO-O d7-28 C\","]
#[doc = "          \"Phraa Blao SO-O d7-28 B\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Systems\": {"]
#[doc = "      \"title\": \"Systems\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"Phraa Blao SO-O d7-28\","]
#[doc = "          \"Dryao Ain ZQ-C c26-0\","]
#[doc = "          \"Laso CM-K d9-9\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"TotalEarnings\": {"]
#[doc = "      \"title\": \"TotalEarnings\","]
#[doc = "      \"description\": \"Total credits received (including for example the 200% bonus if rank 5 with Li Yong Rui)\","]
#[doc = "      \"examples\": ["]
#[doc = "        10951,"]
#[doc = "        4539,"]
#[doc = "        3137"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
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
pub struct SellExplorationData {
    #[serde(rename = "BaseValue")]
    pub base_value: i64,
    #[doc = "Bonus for first discoveries"]
    #[serde(rename = "Bonus")]
    pub bonus: i64,
    #[serde(rename = "Discovered")]
    pub discovered: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_deserializing, default = "SellExplorationData::event_value")]
    pub event: String,
    #[serde(rename = "Systems")]
    pub systems: ::std::vec::Vec<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Total credits received (including for example the 200% bonus if rank 5 with Li Yong Rui)"]
    #[serde(rename = "TotalEarnings")]
    pub total_earnings: i64,
}
impl ::std::convert::From<&SellExplorationData> for SellExplorationData {
    fn from(value: &SellExplorationData) -> Self {
        value.clone()
    }
}

impl SellExplorationData {
    pub fn event_value() -> ::std::string::String {
        "SellExplorationData".to_string()
    }
}
