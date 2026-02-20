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
#[doc = "When written: when looking at the cockpit Right Hand Side modules info panel, if data has changed. This also writes a ModulesInfo.json file alongside the journal, listing the modules in the same order as displayed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when looking at the cockpit Right Hand Side modules info panel, if data has changed. This also writes a ModulesInfo.json file alongside the journal, listing the modules in the same order as displayed.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Modules\": {"]
#[doc = "      \"title\": \"Modules\","]
#[doc = "      \"description\": \"When written to ModulesInfo.json\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Item\","]
#[doc = "          \"Power\","]
#[doc = "          \"Slot\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Item\": {"]
#[doc = "            \"title\": \"Item\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"int_engine_size3_class5\","]
#[doc = "              \"hpt_beamlaser_turret_medium\","]
#[doc = "              \"int_shieldgenerator_size2_class1\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Power\": {"]
#[doc = "            \"title\": \"Power\","]
#[doc = "            \"examples\": ["]
#[doc = "              3.72,"]
#[doc = "              0.93,"]
#[doc = "              0.9"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"Priority\": {"]
#[doc = "            \"title\": \"Priority\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              2"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Slot\": {"]
#[doc = "            \"title\": \"Slot\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"MainEngines\","]
#[doc = "              \"MediumHardpoint1\","]
#[doc = "              \"Slot03_Size2\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
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
pub struct ModuleInfo {
    #[serde(skip_deserializing, default = "ModuleInfo::event_value")]
    pub event: String,
    #[doc = "When written to ModulesInfo.json"]
    #[serde(
        rename = "Modules",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub modules: ::std::vec::Vec<ModulesItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ModuleInfo> for ModuleInfo {
    fn from(value: &ModuleInfo) -> Self {
        value.clone()
    }
}
#[doc = "`ModulesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Item\","]
#[doc = "    \"Power\","]
#[doc = "    \"Slot\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Item\": {"]
#[doc = "      \"title\": \"Item\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"int_engine_size3_class5\","]
#[doc = "        \"hpt_beamlaser_turret_medium\","]
#[doc = "        \"int_shieldgenerator_size2_class1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Power\": {"]
#[doc = "      \"title\": \"Power\","]
#[doc = "      \"examples\": ["]
#[doc = "        3.72,"]
#[doc = "        0.93,"]
#[doc = "        0.9"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Priority\": {"]
#[doc = "      \"title\": \"Priority\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Slot\": {"]
#[doc = "      \"title\": \"Slot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"MainEngines\","]
#[doc = "        \"MediumHardpoint1\","]
#[doc = "        \"Slot03_Size2\""]
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
pub struct ModulesItem {
    #[serde(rename = "Item")]
    pub item: ::std::string::String,
    #[serde(rename = "Power")]
    pub power: f64,
    #[serde(
        rename = "Priority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub priority: ::std::option::Option<i64>,
    #[serde(rename = "Slot")]
    pub slot: ::std::string::String,
}
impl ::std::convert::From<&ModulesItem> for ModulesItem {
    fn from(value: &ModulesItem) -> Self {
        value.clone()
    }
}

impl ModuleInfo {
    pub fn event_value() -> ::std::string::String {
        "ModuleInfo".to_string()
    }
}
