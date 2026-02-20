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
#[doc = "When Written: when moving a module to a different slot on the ship"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when moving a module to a different slot on the ship\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"FromItem\","]
#[doc = "    \"FromSlot\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"ToItem\","]
#[doc = "    \"ToSlot\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"FromItem\": {"]
#[doc = "      \"title\": \"FromItem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_buggybay_size2_class2_name;\","]
#[doc = "        \"$int_detailedsurfacescanner_tiny_name;\","]
#[doc = "        \"$int_shieldcellbank_size6_class5_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"FromItem_Localised\": {"]
#[doc = "      \"title\": \"FromItem_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Planetary Vehicle Hangar\","]
#[doc = "        \"Surface Scanner\","]
#[doc = "        \"Shield Cell Bank\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"FromSlot\": {"]
#[doc = "      \"title\": \"FromSlot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Slot03_Size3\","]
#[doc = "        \"Slot04_Size3\","]
#[doc = "        \"Slot03_Size7\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
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
#[doc = "    \"ToItem\": {"]
#[doc = "      \"title\": \"ToItem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Null\","]
#[doc = "        \"$int_hullreinforcement_size3_class2_name;\","]
#[doc = "        \"$int_corrosionproofcargorack_size4_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ToItem_Localised\": {"]
#[doc = "      \"title\": \"ToItem_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Hull Reinforcement\","]
#[doc = "        \"Anti-Corrosion Cargo Rack\","]
#[doc = "        \"Cargo Rack\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ToSlot\": {"]
#[doc = "      \"title\": \"ToSlot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Slot06_Size2\","]
#[doc = "        \"Slot08_Size1\","]
#[doc = "        \"Slot04_Size6\""]
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
pub struct ModuleSwap {
    #[serde(skip_deserializing, default = "ModuleSwap::event_value")]
    pub event: String,
    #[serde(rename = "FromItem")]
    pub from_item: ::std::string::String,
    #[serde(
        rename = "FromItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub from_item_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "FromSlot")]
    pub from_slot: ::std::string::String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "ToItem")]
    pub to_item: ::std::string::String,
    #[serde(
        rename = "ToItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub to_item_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ToSlot")]
    pub to_slot: ::std::string::String,
}
impl ::std::convert::From<&ModuleSwap> for ModuleSwap {
    fn from(value: &ModuleSwap) -> Self {
        value.clone()
    }
}

impl ModuleSwap {
    pub fn event_value() -> ::std::string::String {
        "ModuleSwap".to_string()
    }
}
