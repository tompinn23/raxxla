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
#[doc = "`ItemsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Hot\","]
#[doc = "    \"Name\","]
#[doc = "    \"Slot\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"EngineerModifications\": {"]
#[doc = "      \"title\": \"EngineerModifications\","]
#[doc = "      \"description\": \"Only if modifications are present\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Weapon_Efficient\","]
#[doc = "        \"Weapon_HighCapacity\","]
#[doc = "        \"ShieldBooster_Thermic\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Hot\": {"]
#[doc = "      \"title\": \"Hot\","]
#[doc = "      \"description\": \"If there is a fine/bounty associated with the module\","]
#[doc = "      \"examples\": ["]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Level\": {"]
#[doc = "      \"title\": \"Level\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        3,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_repairer_size3_class5_name;\","]
#[doc = "        \"$int_dronecontrol_repair_size3_class3_name;\","]
#[doc = "        \"$int_cargorack_size2_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"AFM Unit\","]
#[doc = "        \"Repair\","]
#[doc = "        \"Cargo Rack\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"title\": \"Quality\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.921,"]
#[doc = "        0.9058,"]
#[doc = "        1.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Slot\": {"]
#[doc = "      \"title\": \"Slot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Slot03_Size3\","]
#[doc = "        \"Slot04_Size3\","]
#[doc = "        \"Slot06_Size2\""]
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
pub struct ItemsItem {
    #[doc = "Only if modifications are present"]
    #[serde(
        rename = "EngineerModifications",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer_modifications: ::std::option::Option<::std::string::String>,
    #[doc = "If there is a fine/bounty associated with the module"]
    #[serde(rename = "Hot")]
    pub hot: bool,
    #[serde(
        rename = "Level",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub level: ::std::option::Option<i64>,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Quality",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quality: ::std::option::Option<f64>,
    #[serde(rename = "Slot")]
    pub slot: ::std::string::String,
}
impl ::std::convert::From<&ItemsItem> for ItemsItem {
    fn from(value: &ItemsItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: when putting multiple modules into storage"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when putting multiple modules into storage\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Items\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Items\": {"]
#[doc = "      \"title\": \"Items\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Hot\","]
#[doc = "          \"Name\","]
#[doc = "          \"Slot\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"EngineerModifications\": {"]
#[doc = "            \"title\": \"EngineerModifications\","]
#[doc = "            \"description\": \"Only if modifications are present\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Weapon_Efficient\","]
#[doc = "              \"Weapon_HighCapacity\","]
#[doc = "              \"ShieldBooster_Thermic\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Hot\": {"]
#[doc = "            \"title\": \"Hot\","]
#[doc = "            \"description\": \"If there is a fine/bounty associated with the module\","]
#[doc = "            \"examples\": ["]
#[doc = "              false"]
#[doc = "            ],"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"Level\": {"]
#[doc = "            \"title\": \"Level\","]
#[doc = "            \"examples\": ["]
#[doc = "              5,"]
#[doc = "              3,"]
#[doc = "              4"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$int_repairer_size3_class5_name;\","]
#[doc = "              \"$int_dronecontrol_repair_size3_class3_name;\","]
#[doc = "              \"$int_cargorack_size2_class1_name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"AFM Unit\","]
#[doc = "              \"Repair\","]
#[doc = "              \"Cargo Rack\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Quality\": {"]
#[doc = "            \"title\": \"Quality\","]
#[doc = "            \"examples\": ["]
#[doc = "              0.921,"]
#[doc = "              0.9058,"]
#[doc = "              1.0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minimum\": 0.0"]
#[doc = "          },"]
#[doc = "          \"Slot\": {"]
#[doc = "            \"title\": \"Slot\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Slot03_Size3\","]
#[doc = "              \"Slot04_Size3\","]
#[doc = "              \"Slot06_Size2\""]
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
#[doc = "        128666762,"]
#[doc = "        3705689344,"]
#[doc = "        128988470"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"asp\","]
#[doc = "        \"cutter\","]
#[doc = "        \"anaconda\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        25,"]
#[doc = "        7"]
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
pub struct MassModuleStore {
    #[serde(skip_deserializing, default = "MassModuleStore::event_value")]
    pub event: String,
    #[serde(rename = "Items")]
    pub items: ::std::vec::Vec<ItemsItem>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&MassModuleStore> for MassModuleStore {
    fn from(value: &MassModuleStore) -> Self {
        value.clone()
    }
}

impl MassModuleStore {
    pub fn event_value() -> ::std::string::String {
        "MassModuleStore".to_string()
    }
}
