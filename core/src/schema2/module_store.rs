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
#[doc = "When written: when storing a module in Outfitting"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when storing a module in Outfitting\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"Slot\","]
#[doc = "    \"StoredItem\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Cost\": {"]
#[doc = "      \"title\": \"Cost\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        29,"]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"EngineerModifications\": {"]
#[doc = "      \"title\": \"EngineerModifications\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FSD_LongRange\","]
#[doc = "        \"ShieldGenerator_Thermic\","]
#[doc = "        \"Misc_HeatSinkCapacity\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Hot\": {"]
#[doc = "      \"title\": \"Hot\","]
#[doc = "      \"examples\": ["]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Level\": {"]
#[doc = "      \"title\": \"Level\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        1,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        128666762,"]
#[doc = "        128679559"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"title\": \"Quality\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0,"]
#[doc = "        0.978,"]
#[doc = "        0.9569"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ReplacementItem\": {"]
#[doc = "      \"title\": \"ReplacementItem\","]
#[doc = "      \"description\": \"If a core module is stored. Documented, but appears to be unused.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_engine_size4_class5_name;\","]
#[doc = "        \"$int_powerplant_size3_class5_name;\","]
#[doc = "        \"$int_hyperdrive_size5_class5_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"asp\","]
#[doc = "        \"federation_corvette\","]
#[doc = "        \"cutter\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        29,"]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Slot\": {"]
#[doc = "      \"title\": \"Slot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Slot03_Size3\","]
#[doc = "        \"Slot06_Size2\","]
#[doc = "        \"Slot07_Size2\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoredItem\": {"]
#[doc = "      \"title\": \"StoredItem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_cargorack_size3_class1_name;\","]
#[doc = "        \"$int_cargorack_size2_class1_name;\","]
#[doc = "        \"$int_cargorack_size5_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoredItem_Localised\": {"]
#[doc = "      \"title\": \"StoredItem_Localised\","]
#[doc = "      \"description\": \"If a core module is stored. Documented, but appears to be unused.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Engine\","]
#[doc = "        \"Power Plant\","]
#[doc = "        \"FSD Drive\""]
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
pub struct ModuleStore {
    #[serde(
        rename = "Cost",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cost: ::std::option::Option<i64>,
    #[serde(
        rename = "EngineerModifications",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer_modifications: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "ModuleStore::event_value")]
    pub event: String,
    #[serde(
        rename = "Hot",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hot: ::std::option::Option<bool>,
    #[serde(
        rename = "Level",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub level: ::std::option::Option<i64>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(
        rename = "Quality",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quality: ::std::option::Option<f64>,
    #[doc = "If a core module is stored. Documented, but appears to be unused."]
    #[serde(
        rename = "ReplacementItem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub replacement_item: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[serde(rename = "Slot")]
    pub slot: ::std::string::String,
    #[serde(rename = "StoredItem")]
    pub stored_item: ::std::string::String,
    #[doc = "If a core module is stored. Documented, but appears to be unused."]
    #[serde(
        rename = "StoredItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stored_item_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ModuleStore> for ModuleStore {
    fn from(value: &ModuleStore) -> Self {
        value.clone()
    }
}

impl ModuleStore {
    pub fn event_value() -> ::std::string::String {
        "ModuleStore".to_string()
    }
}
