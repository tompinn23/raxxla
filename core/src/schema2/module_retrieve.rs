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
#[doc = "When written: when fetching a previously stored module"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when fetching a previously stored module\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Hot\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"RetrievedItem\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"Slot\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"EngineerModifications\": {"]
#[doc = "      \"title\": \"EngineerModifications\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Sensor_Expanded\","]
#[doc = "        \"ShieldGenerator_Reinforced\","]
#[doc = "        \"ShieldCellBank_Specialised\""]
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
#[doc = "    \"Level\": {"]
#[doc = "      \"title\": \"Level\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        3,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        129009496,"]
#[doc = "        3228761344"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"title\": \"Quality\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0,"]
#[doc = "        0.8467,"]
#[doc = "        0.977"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"RetrievedItem\": {"]
#[doc = "      \"title\": \"RetrievedItem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_buggybay_size2_class2_name;\","]
#[doc = "        \"$int_dockingcomputer_advanced_name;\","]
#[doc = "        \"$int_detailedsurfacescanner_tiny_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"RetrievedItem_Localised\": {"]
#[doc = "      \"title\": \"RetrievedItem_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Planetary Vehicle Hangar\","]
#[doc = "        \"Docking Computer\","]
#[doc = "        \"Surface Scanner\""]
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
#[doc = "        \"Slot07_Size2\","]
#[doc = "        \"Slot04_Size3\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SwapOutItem\": {"]
#[doc = "      \"title\": \"SwapOutItem\","]
#[doc = "      \"description\": \"If slot was not empty\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_cargorack_size4_class1_name;\","]
#[doc = "        \"$int_guardianhullreinforcement_size5_class2_name;\","]
#[doc = "        \"$int_powerplant_size8_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SwapOutItem_Localised\": {"]
#[doc = "      \"title\": \"SwapOutItem_Localised\","]
#[doc = "      \"description\": \"If slot was not empty\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Cargo Rack\","]
#[doc = "        \"Guardian Hull Reinforcement\","]
#[doc = "        \"Power Plant\""]
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
pub struct ModuleRetrieve {
    #[serde(
        rename = "EngineerModifications",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer_modifications: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "ModuleRetrieve::event_value")]
    pub event: String,
    #[serde(rename = "Hot")]
    pub hot: bool,
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
    #[serde(rename = "RetrievedItem")]
    pub retrieved_item: ::std::string::String,
    #[serde(
        rename = "RetrievedItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub retrieved_item_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[serde(rename = "Slot")]
    pub slot: ::std::string::String,
    #[doc = "If slot was not empty"]
    #[serde(
        rename = "SwapOutItem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub swap_out_item: ::std::option::Option<::std::string::String>,
    #[doc = "If slot was not empty"]
    #[serde(
        rename = "SwapOutItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub swap_out_item_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ModuleRetrieve> for ModuleRetrieve {
    fn from(value: &ModuleRetrieve) -> Self {
        value.clone()
    }
}

impl ModuleRetrieve {
    pub fn event_value() -> ::std::string::String {
        "ModuleRetrieve".to_string()
    }
}
