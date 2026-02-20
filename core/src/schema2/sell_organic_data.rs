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
#[doc = "`BioDataItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Bonus\","]
#[doc = "    \"Genus\","]
#[doc = "    \"Species\","]
#[doc = "    \"Value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Bonus\": {"]
#[doc = "      \"title\": \"Bonus\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        121300,"]
#[doc = "        102500"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Genus\": {"]
#[doc = "      \"title\": \"Genus\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Fonticulus_Genus_Name;\","]
#[doc = "        \"$Codex_Ent_Bacterial_Genus_Name;\","]
#[doc = "        \"$Codex_Ent_Stratum_Genus_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Genus_Localised\": {"]
#[doc = "      \"title\": \"Genus_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Fonticulua\","]
#[doc = "        \"Bacterium\","]
#[doc = "        \"Stratum\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Species\": {"]
#[doc = "      \"title\": \"Species\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Fonticulus_02_Name;\","]
#[doc = "        \"$Codex_Ent_Bacterial_12_Name;\","]
#[doc = "        \"$Codex_Ent_Stratum_02_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Species_Localised\": {"]
#[doc = "      \"title\": \"Species_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Fonticulua Campestris\","]
#[doc = "        \"Bacterium Cerbrus\","]
#[doc = "        \"Stratum Paleas\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"examples\": ["]
#[doc = "        63600,"]
#[doc = "        121300,"]
#[doc = "        102500"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Variant\": {"]
#[doc = "      \"title\": \"Variant\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Tubus_01_A_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Variant_Localised\": {"]
#[doc = "      \"title\": \"Variant_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Tubus Conifer - Indigo\""]
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
pub struct BioDataItem {
    #[serde(rename = "Bonus")]
    pub bonus: i64,
    #[serde(rename = "Genus")]
    pub genus: ::std::string::String,
    #[serde(
        rename = "Genus_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub genus_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Species")]
    pub species: ::std::string::String,
    #[serde(
        rename = "Species_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub species_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Value")]
    pub value: i64,
    #[serde(
        rename = "Variant",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub variant: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Variant_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub variant_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&BioDataItem> for BioDataItem {
    fn from(value: &BioDataItem) -> Self {
        value.clone()
    }
}
#[doc = "This event records that a player has sold organic data (see ScanOrganic)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event records that a player has sold organic data (see ScanOrganic)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BioData\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BioData\": {"]
#[doc = "      \"title\": \"BioData\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Bonus\","]
#[doc = "          \"Genus\","]
#[doc = "          \"Species\","]
#[doc = "          \"Value\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Bonus\": {"]
#[doc = "            \"title\": \"Bonus\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              121300,"]
#[doc = "              102500"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Genus\": {"]
#[doc = "            \"title\": \"Genus\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$Codex_Ent_Fonticulus_Genus_Name;\","]
#[doc = "              \"$Codex_Ent_Bacterial_Genus_Name;\","]
#[doc = "              \"$Codex_Ent_Stratum_Genus_Name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Genus_Localised\": {"]
#[doc = "            \"title\": \"Genus_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Fonticulua\","]
#[doc = "              \"Bacterium\","]
#[doc = "              \"Stratum\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Species\": {"]
#[doc = "            \"title\": \"Species\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$Codex_Ent_Fonticulus_02_Name;\","]
#[doc = "              \"$Codex_Ent_Bacterial_12_Name;\","]
#[doc = "              \"$Codex_Ent_Stratum_02_Name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Species_Localised\": {"]
#[doc = "            \"title\": \"Species_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Fonticulua Campestris\","]
#[doc = "              \"Bacterium Cerbrus\","]
#[doc = "              \"Stratum Paleas\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Value\": {"]
#[doc = "            \"title\": \"Value\","]
#[doc = "            \"examples\": ["]
#[doc = "              63600,"]
#[doc = "              121300,"]
#[doc = "              102500"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Variant\": {"]
#[doc = "            \"title\": \"Variant\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$Codex_Ent_Tubus_01_A_Name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Variant_Localised\": {"]
#[doc = "            \"title\": \"Variant_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Tubus Conifer - Indigo\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        128927917,"]
#[doc = "        3226858240"]
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
pub struct SellOrganicData {
    #[serde(rename = "BioData")]
    pub bio_data: ::std::vec::Vec<BioDataItem>,
    #[serde(skip_deserializing, default = "SellOrganicData::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&SellOrganicData> for SellOrganicData {
    fn from(value: &SellOrganicData) -> Self {
        value.clone()
    }
}

impl SellOrganicData {
    pub fn event_value() -> ::std::string::String {
        "SellOrganicData".to_string()
    }
}
