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
#[doc = "This event is logged when buying microresources"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when buying microresources\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"Price\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Consumable\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"description\": \"Old format (used for example at supplies vendor)\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        8,"]
#[doc = "        7"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3221924608,"]
#[doc = "        128927917,"]
#[doc = "        3227846912"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MicroResources\": {"]
#[doc = "      \"title\": \"MicroResources\","]
#[doc = "      \"description\": \"New format (eg at Fleet Carrier bartender)\","]
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
#[doc = "            \"examples\": ["]
#[doc = "              \"Data\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              2"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"opinionpolls\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Opinion Polls\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"description\": \"Old format (used for example at supplies vendor)\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"bypass\","]
#[doc = "        \"amm_grenade_frag\","]
#[doc = "        \"amm_grenade_emp\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"Old format (used for example at supplies vendor)\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"E-Breach\","]
#[doc = "        \"Frag Grenade\","]
#[doc = "        \"Shield Disruptor\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Price\": {"]
#[doc = "      \"title\": \"Price\","]
#[doc = "      \"examples\": ["]
#[doc = "        25000,"]
#[doc = "        16000,"]
#[doc = "        14000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TotalCount\": {"]
#[doc = "      \"title\": \"TotalCount\","]
#[doc = "      \"description\": \"New format (eg at Fleet Carrier bartender)\","]
#[doc = "      \"examples\": ["]
#[doc = "        2"]
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
pub struct BuyMicroResources {
    #[serde(
        rename = "Category",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub category: ::std::option::Option<::std::string::String>,
    #[doc = "Old format (used for example at supplies vendor)"]
    #[serde(
        rename = "Count",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub count: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "BuyMicroResources::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "New format (eg at Fleet Carrier bartender)"]
    #[serde(
        rename = "MicroResources",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub micro_resources: ::std::vec::Vec<MicroResourcesItem>,
    #[doc = "Old format (used for example at supplies vendor)"]
    #[serde(
        rename = "Name",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Old format (used for example at supplies vendor)"]
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Price")]
    pub price: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "New format (eg at Fleet Carrier bartender)"]
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_count: ::std::option::Option<i64>,
}
impl ::std::convert::From<&BuyMicroResources> for BuyMicroResources {
    fn from(value: &BuyMicroResources) -> Self {
        value.clone()
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
#[doc = "      \"examples\": ["]
#[doc = "        \"Data\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"opinionpolls\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Opinion Polls\""]
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
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
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

impl BuyMicroResources {
    pub fn event_value() -> ::std::string::String {
        "BuyMicroResources".to_string()
    }
}
