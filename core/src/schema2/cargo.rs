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
#[doc = "When written: at startup, note this is now written slightly later in startup, after we have initialised missions, so we can detect if any cargo came from an abandoned delivery mission. The first Cargo event in the file will contain the full inventory, others just indicate a separate file has been written. The full data is now written to a separate file Cargo.json. A simple event (with no parameters) is written to the main journal file when the cargo file is updated."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: at startup, note this is now written slightly later in startup, after we have initialised missions, so we can detect if any cargo came from an abandoned delivery mission. The first Cargo event in the file will contain the full inventory, others just indicate a separate file has been written. The full data is now written to a separate file Cargo.json. A simple event (with no parameters) is written to the main journal file when the cargo file is updated.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"Vessel\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        16,"]
#[doc = "        0,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Inventory\": {"]
#[doc = "      \"title\": \"Inventory\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Count\","]
#[doc = "          \"Name\","]
#[doc = "          \"Stolen\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              16,"]
#[doc = "              1,"]
#[doc = "              5"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"MissionID\": {"]
#[doc = "            \"title\": \"MissionID\","]
#[doc = "            \"examples\": ["]
#[doc = "              811118045,"]
#[doc = "              811114594,"]
#[doc = "              893617620"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"drones\","]
#[doc = "              \"metaalloys\","]
#[doc = "              \"bertrandite\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Limpet\","]
#[doc = "              \"Meta-Alloys\","]
#[doc = "              \"Guardian Relic\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Stolen\": {"]
#[doc = "            \"title\": \"Stolen\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              1,"]
#[doc = "              7"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Vessel\": {"]
#[doc = "      \"title\": \"Vessel\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Ship\","]
#[doc = "        \"SRV\""]
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
pub struct Cargo {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(skip_deserializing, default = "Cargo::event_value")]
    pub event: String,
    #[serde(
        rename = "Inventory",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub inventory: ::std::vec::Vec<InventoryItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Vessel")]
    pub vessel: ::std::string::String,
}
impl ::std::convert::From<&Cargo> for Cargo {
    fn from(value: &Cargo) -> Self {
        value.clone()
    }
}
#[doc = "`InventoryItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"Name\","]
#[doc = "    \"Stolen\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        16,"]
#[doc = "        1,"]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        811118045,"]
#[doc = "        811114594,"]
#[doc = "        893617620"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"drones\","]
#[doc = "        \"metaalloys\","]
#[doc = "        \"bertrandite\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Limpet\","]
#[doc = "        \"Meta-Alloys\","]
#[doc = "        \"Guardian Relic\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Stolen\": {"]
#[doc = "      \"title\": \"Stolen\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1,"]
#[doc = "        7"]
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
pub struct InventoryItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(
        rename = "MissionID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mission_id: ::std::option::Option<i64>,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Stolen")]
    pub stolen: i64,
}
impl ::std::convert::From<&InventoryItem> for InventoryItem {
    fn from(value: &InventoryItem) -> Self {
        value.clone()
    }
}

impl Cargo {
    pub fn event_value() -> ::std::string::String {
        "Cargo".to_string()
    }
}
