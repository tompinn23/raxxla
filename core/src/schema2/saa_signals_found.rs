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
#[doc = "`GenusesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Genus\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Genus\": {"]
#[doc = "      \"title\": \"Genus\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Aleoids_Genus_Name;\","]
#[doc = "        \"$Codex_Ent_Bacterial_Genus_Name;\","]
#[doc = "        \"$Codex_Ent_Fungoids_Genus_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Genus_Localised\": {"]
#[doc = "      \"title\": \"Genus_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Aleoida\","]
#[doc = "        \"Bacterium\","]
#[doc = "        \"Fungoida\""]
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
pub struct GenusesItem {
    #[serde(rename = "Genus")]
    pub genus: ::std::string::String,
    #[serde(
        rename = "Genus_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub genus_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&GenusesItem> for GenusesItem {
    fn from(value: &GenusesItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: when using Surface Area Analysis Scanner on a planet or rings"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when using Surface Area Analysis Scanner on a planet or rings\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyID\","]
#[doc = "    \"BodyName\","]
#[doc = "    \"Signals\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        34,"]
#[doc = "        35,"]
#[doc = "        37"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyName\": {"]
#[doc = "      \"title\": \"BodyName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"90482 Orcus\","]
#[doc = "        \"Vanth\","]
#[doc = "        \"Salacia\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Genuses\": {"]
#[doc = "      \"title\": \"Genuses\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Genus\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Genus\": {"]
#[doc = "            \"title\": \"Genus\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$Codex_Ent_Aleoids_Genus_Name;\","]
#[doc = "              \"$Codex_Ent_Bacterial_Genus_Name;\","]
#[doc = "              \"$Codex_Ent_Fungoids_Genus_Name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Genus_Localised\": {"]
#[doc = "            \"title\": \"Genus_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Aleoida\","]
#[doc = "              \"Bacterium\","]
#[doc = "              \"Fungoida\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Signals\": {"]
#[doc = "      \"title\": \"Signals\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Count\","]
#[doc = "          \"Type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              2,"]
#[doc = "              3,"]
#[doc = "              6"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Type\": {"]
#[doc = "            \"title\": \"Type\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$SAA_SignalType_Human;\","]
#[doc = "              \"$SAA_SignalType_Geological;\","]
#[doc = "              \"Serendibite\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Type_Localised\": {"]
#[doc = "            \"title\": \"Type_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Human\","]
#[doc = "              \"Geological\","]
#[doc = "              \"Biological\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        10477373803,"]
#[doc = "        44753062275,"]
#[doc = "        593108606041"]
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
pub struct SAASignalsFound {
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "BodyName")]
    pub body_name: ::std::string::String,
    #[serde(skip_deserializing, default = "SAASignalsFound::event_value")]
    pub event: String,
    #[serde(
        rename = "Genuses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub genuses: ::std::vec::Vec<GenusesItem>,
    #[serde(rename = "Signals")]
    pub signals: ::std::vec::Vec<SignalsItem>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&SAASignalsFound> for SAASignalsFound {
    fn from(value: &SAASignalsFound) -> Self {
        value.clone()
    }
}
#[doc = "`SignalsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"Type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        2,"]
#[doc = "        3,"]
#[doc = "        6"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$SAA_SignalType_Human;\","]
#[doc = "        \"$SAA_SignalType_Geological;\","]
#[doc = "        \"Serendibite\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type_Localised\": {"]
#[doc = "      \"title\": \"Type_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Human\","]
#[doc = "        \"Geological\","]
#[doc = "        \"Biological\""]
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
pub struct SignalsItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[serde(
        rename = "Type_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SignalsItem> for SignalsItem {
    fn from(value: &SignalsItem) -> Self {
        value.clone()
    }
}

impl SAASignalsFound {
    pub fn event_value() -> ::std::string::String {
        "SAASignalsFound".to_string()
    }
}
