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
#[doc = "When written: when offering items cash or bounties to an Engineer to gain access"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when offering items cash or bounties to an Engineer to gain access\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Engineer\","]
#[doc = "    \"EngineerID\","]
#[doc = "    \"Quantity\","]
#[doc = "    \"TotalQuantity\","]
#[doc = "    \"Type\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Commodity\": {"]
#[doc = "      \"title\": \"Commodity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"xihecompanions\","]
#[doc = "        \"konggaale\","]
#[doc = "        \"bromellite\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Commodity_Localised\": {"]
#[doc = "      \"title\": \"Commodity_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Xihe Biomorphic Companions\","]
#[doc = "        \"Kongga Ale\","]
#[doc = "        \"Occupied Escape Pod\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Engineer\": {"]
#[doc = "      \"title\": \"Engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Zacariah Nemo\","]
#[doc = "        \"Lori Jameson\","]
#[doc = "        \"The Sarge\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"EngineerID\": {"]
#[doc = "      \"title\": \"EngineerID\","]
#[doc = "      \"examples\": ["]
#[doc = "        300050,"]
#[doc = "        300230,"]
#[doc = "        300040"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Material\": {"]
#[doc = "      \"title\": \"Material\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"shieldpatternanalysis\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Material_Localised\": {"]
#[doc = "      \"title\": \"Material_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Aberrant Shield Pattern Analysis\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Quantity\": {"]
#[doc = "      \"title\": \"Quantity\","]
#[doc = "      \"description\": \"Quantity offered this time\","]
#[doc = "      \"examples\": ["]
#[doc = "        25,"]
#[doc = "        16,"]
#[doc = "        9"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TotalQuantity\": {"]
#[doc = "      \"title\": \"TotalQuantity\","]
#[doc = "      \"description\": \"Total amount donated\","]
#[doc = "      \"examples\": ["]
#[doc = "        25,"]
#[doc = "        16,"]
#[doc = "        50"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Commodity\","]
#[doc = "        \"Materials\","]
#[doc = "        \"Bounty\""]
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
pub struct EngineerContribution {
    #[serde(
        rename = "Commodity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Commodity_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Engineer")]
    pub engineer: ::std::string::String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,
    #[serde(skip_deserializing, default = "EngineerContribution::event_value")]
    pub event: String,
    #[serde(
        rename = "Material",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub material: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Material_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub material_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Quantity offered this time"]
    #[serde(rename = "Quantity")]
    pub quantity: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Total amount donated"]
    #[serde(rename = "TotalQuantity")]
    pub total_quantity: i64,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&EngineerContribution> for EngineerContribution {
    fn from(value: &EngineerContribution) -> Self {
        value.clone()
    }
}

impl EngineerContribution {
    pub fn event_value() -> ::std::string::String {
        "EngineerContribution".to_string()
    }
}
