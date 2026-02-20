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
#[doc = "    \"BuyPrice\","]
#[doc = "    \"Hot\","]
#[doc = "    \"Name\","]
#[doc = "    \"StorageSlot\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BuyPrice\": {"]
#[doc = "      \"title\": \"BuyPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        504,"]
#[doc = "        1411"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"EngineerModifications\": {"]
#[doc = "      \"title\": \"EngineerModifications\","]
#[doc = "      \"description\": \"If engineered module\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Weapon_HighCapacity\","]
#[doc = "        \"FSD_LongRange\","]
#[doc = "        \"Sensor_Expanded\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Hot\": {"]
#[doc = "      \"title\": \"Hot\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"InTransit\": {"]
#[doc = "      \"title\": \"InTransit\","]
#[doc = "      \"description\": \"If the module is being transferred\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Level\": {"]
#[doc = "      \"title\": \"Level\","]
#[doc = "      \"description\": \"If engineered module\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        1,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        3225728000,"]
#[doc = "        128666762"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$hpt_basicmissilerack_fixed_medium_name;\","]
#[doc = "        \"$int_hyperdrive_size4_class5_name;\","]
#[doc = "        \"$int_detailedsurfacescanner_tiny_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Seeker Missile Rack\","]
#[doc = "        \"FSD\","]
#[doc = "        \"Surface Scanner\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"title\": \"Quality\","]
#[doc = "      \"description\": \"If engineered module\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0,"]
#[doc = "        0.0,"]
#[doc = "        0.732"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Bodhinga\","]
#[doc = "        \"Shinrarta Dezhra\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StorageSlot\": {"]
#[doc = "      \"title\": \"StorageSlot\","]
#[doc = "      \"examples\": ["]
#[doc = "        10,"]
#[doc = "        17,"]
#[doc = "        73"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TransferCost\": {"]
#[doc = "      \"title\": \"TransferCost\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        110,"]
#[doc = "        130"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TransferTime\": {"]
#[doc = "      \"title\": \"TransferTime\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        3856,"]
#[doc = "        2235"]
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
    #[serde(rename = "BuyPrice")]
    pub buy_price: i64,
    #[doc = "If engineered module"]
    #[serde(
        rename = "EngineerModifications",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer_modifications: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Hot")]
    pub hot: bool,
    #[doc = "If the module is being transferred"]
    #[serde(
        rename = "InTransit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub in_transit: ::std::option::Option<bool>,
    #[doc = "If engineered module"]
    #[serde(
        rename = "Level",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub level: ::std::option::Option<i64>,
    #[serde(
        rename = "MarketID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub market_id: ::std::option::Option<i64>,
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
    #[serde(
        rename = "StarSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub star_system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "StorageSlot")]
    pub storage_slot: i64,
    #[serde(
        rename = "TransferCost",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transfer_cost: ::std::option::Option<i64>,
    #[serde(
        rename = "TransferTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transfer_time: ::std::option::Option<i64>,
}
impl ::std::convert::From<&ItemsItem> for ItemsItem {
    fn from(value: &ItemsItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: when first visiting Outfitting, and when the set of stored modules has changed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when first visiting Outfitting, and when the set of stored modules has changed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Items\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"StationName\","]
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
#[doc = "          \"BuyPrice\","]
#[doc = "          \"Hot\","]
#[doc = "          \"Name\","]
#[doc = "          \"StorageSlot\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"BuyPrice\": {"]
#[doc = "            \"title\": \"BuyPrice\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              504,"]
#[doc = "              1411"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"EngineerModifications\": {"]
#[doc = "            \"title\": \"EngineerModifications\","]
#[doc = "            \"description\": \"If engineered module\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Weapon_HighCapacity\","]
#[doc = "              \"FSD_LongRange\","]
#[doc = "              \"Sensor_Expanded\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Hot\": {"]
#[doc = "            \"title\": \"Hot\","]
#[doc = "            \"examples\": ["]
#[doc = "              false,"]
#[doc = "              true"]
#[doc = "            ],"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"InTransit\": {"]
#[doc = "            \"title\": \"InTransit\","]
#[doc = "            \"description\": \"If the module is being transferred\","]
#[doc = "            \"examples\": ["]
#[doc = "              true"]
#[doc = "            ],"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"Level\": {"]
#[doc = "            \"title\": \"Level\","]
#[doc = "            \"description\": \"If engineered module\","]
#[doc = "            \"examples\": ["]
#[doc = "              5,"]
#[doc = "              1,"]
#[doc = "              4"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"MarketID\": {"]
#[doc = "            \"title\": \"MarketID\","]
#[doc = "            \"examples\": ["]
#[doc = "              3705689344,"]
#[doc = "              3225728000,"]
#[doc = "              128666762"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$hpt_basicmissilerack_fixed_medium_name;\","]
#[doc = "              \"$int_hyperdrive_size4_class5_name;\","]
#[doc = "              \"$int_detailedsurfacescanner_tiny_name;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Seeker Missile Rack\","]
#[doc = "              \"FSD\","]
#[doc = "              \"Surface Scanner\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Quality\": {"]
#[doc = "            \"title\": \"Quality\","]
#[doc = "            \"description\": \"If engineered module\","]
#[doc = "            \"examples\": ["]
#[doc = "              1.0,"]
#[doc = "              0.0,"]
#[doc = "              0.732"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"StarSystem\": {"]
#[doc = "            \"title\": \"StarSystem\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"HIP 20485\","]
#[doc = "              \"Bodhinga\","]
#[doc = "              \"Shinrarta Dezhra\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"StorageSlot\": {"]
#[doc = "            \"title\": \"StorageSlot\","]
#[doc = "            \"examples\": ["]
#[doc = "              10,"]
#[doc = "              17,"]
#[doc = "              73"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"TransferCost\": {"]
#[doc = "            \"title\": \"TransferCost\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              110,"]
#[doc = "              130"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"TransferTime\": {"]
#[doc = "            \"title\": \"TransferTime\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              3856,"]
#[doc = "              2235"]
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
#[doc = "        3705689344,"]
#[doc = "        3225943808,"]
#[doc = "        3700367104"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Swoiwns ZD-B d1\","]
#[doc = "        \"Synuefai LW-N b52-1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StationName\": {"]
#[doc = "      \"title\": \"StationName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"A1A-B2B\","]
#[doc = "        \"Smith Enterprise\""]
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
pub struct StoredModules {
    #[serde(skip_deserializing, default = "StoredModules::event_value")]
    pub event: String,
    #[serde(rename = "Items")]
    pub items: ::std::vec::Vec<ItemsItem>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "StationName")]
    pub station_name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&StoredModules> for StoredModules {
    fn from(value: &StoredModules) -> Self {
        value.clone()
    }
}

impl StoredModules {
    pub fn event_value() -> ::std::string::String {
        "StoredModules".to_string()
    }
}
