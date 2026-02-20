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
#[doc = "`DiscoveredItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"NumBodies\","]
#[doc = "    \"SystemName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"NumBodies\": {"]
#[doc = "      \"title\": \"NumBodies\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        7,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SystemName\": {"]
#[doc = "      \"title\": \"SystemName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HD 111990\","]
#[doc = "        \"NGC 4755 BK 125\","]
#[doc = "        \"HR 4876\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemName_Localised\": {"]
#[doc = "      \"title\": \"SystemName_Localised\","]
#[doc = "      \"description\": \"This field sometime appears in this event containing random data (bug)\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Random stuff\""]
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
pub struct DiscoveredItem {
    #[serde(rename = "NumBodies")]
    pub num_bodies: i64,
    #[serde(rename = "SystemName")]
    pub system_name: ::std::string::String,
    #[doc = "This field sometime appears in this event containing random data (bug)"]
    #[serde(
        rename = "SystemName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&DiscoveredItem> for DiscoveredItem {
    fn from(value: &DiscoveredItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: when selling exploration data in Cartographics, a page at a time"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when selling exploration data in Cartographics, a page at a time\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BaseValue\","]
#[doc = "    \"Bonus\","]
#[doc = "    \"Discovered\","]
#[doc = "    \"TotalEarnings\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BaseValue\": {"]
#[doc = "      \"title\": \"BaseValue\","]
#[doc = "      \"examples\": ["]
#[doc = "        2842803,"]
#[doc = "        2415839,"]
#[doc = "        3663580"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Bonus\": {"]
#[doc = "      \"title\": \"Bonus\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        72000,"]
#[doc = "        66804"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Discovered\": {"]
#[doc = "      \"title\": \"Discovered\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"NumBodies\","]
#[doc = "          \"SystemName\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"NumBodies\": {"]
#[doc = "            \"title\": \"NumBodies\","]
#[doc = "            \"examples\": ["]
#[doc = "              9,"]
#[doc = "              7,"]
#[doc = "              4"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"SystemName\": {"]
#[doc = "            \"title\": \"SystemName\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"HD 111990\","]
#[doc = "              \"NGC 4755 BK 125\","]
#[doc = "              \"HR 4876\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"SystemName_Localised\": {"]
#[doc = "            \"title\": \"SystemName_Localised\","]
#[doc = "            \"description\": \"This field sometime appears in this event containing random data (bug)\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Random stuff\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"TotalEarnings\": {"]
#[doc = "      \"title\": \"TotalEarnings\","]
#[doc = "      \"examples\": ["]
#[doc = "        2842803,"]
#[doc = "        2487839,"]
#[doc = "        3663580"]
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
pub struct MultiSellExplorationData {
    #[serde(rename = "BaseValue")]
    pub base_value: i64,
    #[serde(rename = "Bonus")]
    pub bonus: i64,
    #[serde(rename = "Discovered")]
    pub discovered: ::std::vec::Vec<DiscoveredItem>,
    #[serde(skip_deserializing, default = "MultiSellExplorationData::event_value")]
    pub event: String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "TotalEarnings")]
    pub total_earnings: i64,
}
impl ::std::convert::From<&MultiSellExplorationData> for MultiSellExplorationData {
    fn from(value: &MultiSellExplorationData) -> Self {
        value.clone()
    }
}

impl MultiSellExplorationData {
    pub fn event_value() -> ::std::string::String {
        "MultiSellExplorationData".to_string()
    }
}
