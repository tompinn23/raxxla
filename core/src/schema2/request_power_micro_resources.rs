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
#[doc = "`MicroResourcesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Category\","]
#[doc = "    \"Count\","]
#[doc = "    \"Name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"description\": \"The category of the micro-resource.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Data\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"description\": \"The count of this type of micro-resource.\","]
#[doc = "      \"examples\": ["]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"description\": \"The internal name of the micro-resource.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"powerfinancialrecords\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localized name of the micro-resource.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Power Industrial Data\""]
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
pub struct MicroResourcesItem {
    #[doc = "The category of the micro-resource."]
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[doc = "The count of this type of micro-resource."]
    #[serde(rename = "Count")]
    pub count: i64,
    #[doc = "The internal name of the micro-resource."]
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[doc = "The localized name of the micro-resource."]
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&MicroResourcesItem> for MicroResourcesItem {
    fn from(value: &MicroResourcesItem) -> Self {
        value.clone()
    }
}
#[doc = "When Written: when requesting power micro-resources."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when requesting power micro-resources.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"MicroResources\","]
#[doc = "    \"TotalCount\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"description\": \"The ID of the market where the request took place.\","]
#[doc = "      \"examples\": ["]
#[doc = "        3223182848"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MicroResources\": {"]
#[doc = "      \"title\": \"MicroResources\","]
#[doc = "      \"description\": \"Details of the micro-resources requested.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Category\","]
#[doc = "          \"Count\","]
#[doc = "          \"Name\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Category\": {"]
#[doc = "            \"title\": \"Category\","]
#[doc = "            \"description\": \"The category of the micro-resource.\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Data\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"description\": \"The count of this type of micro-resource.\","]
#[doc = "            \"examples\": ["]
#[doc = "              1"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"description\": \"The internal name of the micro-resource.\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"powerfinancialrecords\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localized name of the micro-resource.\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Power Industrial Data\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"TotalCount\": {"]
#[doc = "      \"title\": \"TotalCount\","]
#[doc = "      \"description\": \"The total number of micro-resources requested.\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        2,"]
#[doc = "        3"]
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
pub struct RequestPowerMicroResources {
    #[serde(
        skip_deserializing,
        default = "RequestPowerMicroResources::event_value"
    )]
    pub event: String,
    #[doc = "The ID of the market where the request took place."]
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "Details of the micro-resources requested."]
    #[serde(rename = "MicroResources")]
    pub micro_resources: ::std::vec::Vec<MicroResourcesItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The total number of micro-resources requested."]
    #[serde(rename = "TotalCount")]
    pub total_count: i64,
}
impl ::std::convert::From<&RequestPowerMicroResources> for RequestPowerMicroResources {
    fn from(value: &RequestPowerMicroResources) -> Self {
        value.clone()
    }
}

impl RequestPowerMicroResources {
    pub fn event_value() -> ::std::string::String {
        "RequestPowerMicroResources".to_string()
    }
}
