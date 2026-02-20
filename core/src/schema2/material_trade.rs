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
#[doc = "When written: when exchanging materials at the Material trader contact"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when exchanging materials at the Material trader contact\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"Paid\","]
#[doc = "    \"Received\","]
#[doc = "    \"TraderType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3230671360,"]
#[doc = "        3224503040,"]
#[doc = "        3230033664"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Paid\": {"]
#[doc = "      \"title\": \"Paid\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Category\","]
#[doc = "        \"Material\","]
#[doc = "        \"Quantity\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Category\": {"]
#[doc = "          \"title\": \"Category\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Manufactured\","]
#[doc = "            \"Encoded\","]
#[doc = "            \"Raw\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Material\": {"]
#[doc = "          \"title\": \"Material\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"protolightalloys\","]
#[doc = "            \"pharmaceuticalisolators\","]
#[doc = "            \"militarygradealloys\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Material_Localised\": {"]
#[doc = "          \"title\": \"Material_Localised\","]
#[doc = "          \"description\": \"The localised value will be omitted if it is exactly the same as Material\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Proto Light Alloys\","]
#[doc = "            \"Pharmaceutical Isolators\","]
#[doc = "            \"Military Grade Alloys\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Quantity\": {"]
#[doc = "          \"title\": \"Quantity\","]
#[doc = "          \"examples\": ["]
#[doc = "            60,"]
#[doc = "            14,"]
#[doc = "            40"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"Received\": {"]
#[doc = "      \"title\": \"Received\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Category\","]
#[doc = "        \"Material\","]
#[doc = "        \"Quantity\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Category\": {"]
#[doc = "          \"title\": \"Category\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Manufactured\","]
#[doc = "            \"Encoded\","]
#[doc = "            \"Raw\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Material\": {"]
#[doc = "          \"title\": \"Material\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"protoradiolicalloys\","]
#[doc = "            \"phasealloys\","]
#[doc = "            \"chemicalmanipulators\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Material_Localised\": {"]
#[doc = "          \"title\": \"Material_Localised\","]
#[doc = "          \"description\": \"The localised value will be omitted if it is exactly the same as Material\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Proto Radiolic Alloys\","]
#[doc = "            \"Phase Alloys\","]
#[doc = "            \"Chemical Manipulators\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Quantity\": {"]
#[doc = "          \"title\": \"Quantity\","]
#[doc = "          \"examples\": ["]
#[doc = "            10,"]
#[doc = "            42,"]
#[doc = "            120"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"TraderType\": {"]
#[doc = "      \"title\": \"TraderType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"manufactured\","]
#[doc = "        \"encoded\","]
#[doc = "        \"raw\""]
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
pub struct MaterialTrade {
    #[serde(skip_deserializing, default = "MaterialTrade::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "Paid")]
    pub paid: Paid,
    #[serde(rename = "Received")]
    pub received: Received,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "TraderType")]
    pub trader_type: ::std::string::String,
}
impl ::std::convert::From<&MaterialTrade> for MaterialTrade {
    fn from(value: &MaterialTrade) -> Self {
        value.clone()
    }
}
#[doc = "`Paid`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Paid\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Category\","]
#[doc = "    \"Material\","]
#[doc = "    \"Quantity\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Manufactured\","]
#[doc = "        \"Encoded\","]
#[doc = "        \"Raw\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Material\": {"]
#[doc = "      \"title\": \"Material\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"protolightalloys\","]
#[doc = "        \"pharmaceuticalisolators\","]
#[doc = "        \"militarygradealloys\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Material_Localised\": {"]
#[doc = "      \"title\": \"Material_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Material\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Proto Light Alloys\","]
#[doc = "        \"Pharmaceutical Isolators\","]
#[doc = "        \"Military Grade Alloys\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Quantity\": {"]
#[doc = "      \"title\": \"Quantity\","]
#[doc = "      \"examples\": ["]
#[doc = "        60,"]
#[doc = "        14,"]
#[doc = "        40"]
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
pub struct Paid {
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[serde(rename = "Material")]
    pub material: ::std::string::String,
    #[doc = "The localised value will be omitted if it is exactly the same as Material"]
    #[serde(
        rename = "Material_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub material_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}
impl ::std::convert::From<&Paid> for Paid {
    fn from(value: &Paid) -> Self {
        value.clone()
    }
}
#[doc = "`Received`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Received\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Category\","]
#[doc = "    \"Material\","]
#[doc = "    \"Quantity\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Manufactured\","]
#[doc = "        \"Encoded\","]
#[doc = "        \"Raw\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Material\": {"]
#[doc = "      \"title\": \"Material\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"protoradiolicalloys\","]
#[doc = "        \"phasealloys\","]
#[doc = "        \"chemicalmanipulators\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Material_Localised\": {"]
#[doc = "      \"title\": \"Material_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Material\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Proto Radiolic Alloys\","]
#[doc = "        \"Phase Alloys\","]
#[doc = "        \"Chemical Manipulators\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Quantity\": {"]
#[doc = "      \"title\": \"Quantity\","]
#[doc = "      \"examples\": ["]
#[doc = "        10,"]
#[doc = "        42,"]
#[doc = "        120"]
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
pub struct Received {
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[serde(rename = "Material")]
    pub material: ::std::string::String,
    #[doc = "The localised value will be omitted if it is exactly the same as Material"]
    #[serde(
        rename = "Material_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub material_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Quantity")]
    pub quantity: i64,
}
impl ::std::convert::From<&Received> for Received {
    fn from(value: &Received) -> Self {
        value.clone()
    }
}

impl MaterialTrade {
    pub fn event_value() -> ::std::string::String {
        "MaterialTrade".to_string()
    }
}
