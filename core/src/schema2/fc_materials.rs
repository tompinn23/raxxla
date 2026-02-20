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
#[doc = "When trading with a Fleet Carrier Bartender for materials, a file is written with the pricelist to FCMaterials.json"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When trading with a Fleet Carrier Bartender for materials, a file is written with the pricelist to FCMaterials.json\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CarrierID\","]
#[doc = "    \"CarrierName\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CarrierID\": {"]
#[doc = "      \"title\": \"CarrierID\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"A1A-B2B\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"CarrierName\": {"]
#[doc = "      \"title\": \"CarrierName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Carrier Name\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Items\": {"]
#[doc = "      \"title\": \"Items\","]
#[doc = "      \"description\": \"Only included in FCMaterials.json\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Demand\","]
#[doc = "          \"Name\","]
#[doc = "          \"Price\","]
#[doc = "          \"Stock\","]
#[doc = "          \"id\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Demand\": {"]
#[doc = "            \"title\": \"Demand\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              1,"]
#[doc = "              98"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$californium_name;\","]
#[doc = "              \"$aerogel_name;\","]
#[doc = "              \"$culinaryrecipes_name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Californium\","]
#[doc = "              \"Aerogel\","]
#[doc = "              \"Culinary Recipes\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Price\": {"]
#[doc = "            \"title\": \"Price\","]
#[doc = "            \"examples\": ["]
#[doc = "              74000,"]
#[doc = "              1000,"]
#[doc = "              400"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Stock\": {"]
#[doc = "            \"title\": \"Stock\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              20,"]
#[doc = "              18"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"id\": {"]
#[doc = "            \"title\": \"id\","]
#[doc = "            \"examples\": ["]
#[doc = "              128961556,"]
#[doc = "              128972334,"]
#[doc = "              128961527"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705124096,"]
#[doc = "        3707953408,"]
#[doc = "        3706338816"]
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
pub struct FCMaterials {
    #[serde(rename = "CarrierID")]
    pub carrier_id: ::std::string::String,
    #[serde(rename = "CarrierName")]
    pub carrier_name: ::std::string::String,
    #[serde(skip_deserializing, default = "FCMaterials::event_value")]
    pub event: String,
    #[doc = "Only included in FCMaterials.json"]
    #[serde(
        rename = "Items",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub items: ::std::vec::Vec<ItemsItem>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&FCMaterials> for FCMaterials {
    fn from(value: &FCMaterials) -> Self {
        value.clone()
    }
}
#[doc = "`ItemsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Demand\","]
#[doc = "    \"Name\","]
#[doc = "    \"Price\","]
#[doc = "    \"Stock\","]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Demand\": {"]
#[doc = "      \"title\": \"Demand\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1,"]
#[doc = "        98"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$californium_name;\","]
#[doc = "        \"$aerogel_name;\","]
#[doc = "        \"$culinaryrecipes_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Californium\","]
#[doc = "        \"Aerogel\","]
#[doc = "        \"Culinary Recipes\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Price\": {"]
#[doc = "      \"title\": \"Price\","]
#[doc = "      \"examples\": ["]
#[doc = "        74000,"]
#[doc = "        1000,"]
#[doc = "        400"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Stock\": {"]
#[doc = "      \"title\": \"Stock\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        20,"]
#[doc = "        18"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"id\","]
#[doc = "      \"examples\": ["]
#[doc = "        128961556,"]
#[doc = "        128972334,"]
#[doc = "        128961527"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ItemsItem {
    #[serde(rename = "Demand")]
    pub demand: i64,
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Price")]
    pub price: i64,
    #[serde(rename = "Stock")]
    pub stock: i64,
}
impl ::std::convert::From<&ItemsItem> for ItemsItem {
    fn from(value: &ItemsItem) -> Self {
        value.clone()
    }
}

impl FCMaterials {
    pub fn event_value() -> ::std::string::String {
        "FCMaterials".to_string()
    }
}
