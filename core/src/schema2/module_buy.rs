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
#[doc = "When Written: when buying a module in outfitting"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when buying a module in outfitting\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BuyItem\","]
#[doc = "    \"BuyPrice\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"Slot\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BuyItem\": {"]
#[doc = "      \"title\": \"BuyItem\","]
#[doc = "      \"description\": \"The module being purchased\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_dronecontrol_unkvesselresearch_name;\","]
#[doc = "        \"$int_multidronecontrol_xeno_size3_class3_name;\","]
#[doc = "        \"$int_multidronecontrol_mining_size3_class3_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BuyItem_Localised\": {"]
#[doc = "      \"title\": \"BuyItem_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Research\","]
#[doc = "        \"Xeno Multi-Limpet Controller\","]
#[doc = "        \"Mining Multi-Limpet Controller\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BuyPrice\": {"]
#[doc = "      \"title\": \"BuyPrice\","]
#[doc = "      \"description\": \"Price paid\","]
#[doc = "      \"examples\": ["]
#[doc = "        1535274,"]
#[doc = "        43875,"]
#[doc = "        70200"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128666762,"]
#[doc = "        128679559,"]
#[doc = "        128927917"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SellItem\": {"]
#[doc = "      \"title\": \"SellItem\","]
#[doc = "      \"description\": \"If existing module in slot is sold\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_hyperdrive_size5_class1_name;\","]
#[doc = "        \"$int_powerplant_size5_class1_name;\","]
#[doc = "        \"$int_powerdistributor_size4_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SellItem_Localised\": {"]
#[doc = "      \"title\": \"SellItem_Localised\","]
#[doc = "      \"description\": \"If existing module in slot is sold\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FSD\","]
#[doc = "        \"Power Plant\","]
#[doc = "        \"Power Distributor\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SellPrice\": {"]
#[doc = "      \"title\": \"SellPrice\","]
#[doc = "      \"description\": \"If existing module in slot is sold\","]
#[doc = "      \"examples\": ["]
#[doc = "        61436,"]
#[doc = "        52044,"]
#[doc = "        11065"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"description\": \"The player's ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"asp\","]
#[doc = "        \"cutter\","]
#[doc = "        \"krait_mkii\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"description\": \"The player's ship ID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        25,"]
#[doc = "        34"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Slot\": {"]
#[doc = "      \"title\": \"Slot\","]
#[doc = "      \"description\": \"Slot that is outfitted\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Slot03_Size3\","]
#[doc = "        \"Slot04_Size3\","]
#[doc = "        \"TinyHardpoint1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoredItem\": {"]
#[doc = "      \"title\": \"StoredItem\","]
#[doc = "      \"description\": \"If existing module in slot is stored\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_lifesupport_size4_class1_name;\","]
#[doc = "        \"$int_sensors_size5_class1_name;\","]
#[doc = "        \"$int_powerdistributor_size4_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoredItem_Localised\": {"]
#[doc = "      \"title\": \"StoredItem_Localised\","]
#[doc = "      \"description\": \"If existing module in slot is stored\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Life Support\","]
#[doc = "        \"Sensors\","]
#[doc = "        \"Power Distributor\""]
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
pub struct ModuleBuy {
    #[doc = "The module being purchased"]
    #[serde(rename = "BuyItem")]
    pub buy_item: ::std::string::String,
    #[serde(
        rename = "BuyItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub buy_item_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Price paid"]
    #[serde(rename = "BuyPrice")]
    pub buy_price: i64,
    #[serde(skip_deserializing, default = "ModuleBuy::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "If existing module in slot is sold"]
    #[serde(
        rename = "SellItem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_item: ::std::option::Option<::std::string::String>,
    #[doc = "If existing module in slot is sold"]
    #[serde(
        rename = "SellItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_item_localised: ::std::option::Option<::std::string::String>,
    #[doc = "If existing module in slot is sold"]
    #[serde(
        rename = "SellPrice",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_price: ::std::option::Option<i64>,
    #[doc = "The player's ship"]
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[doc = "The player's ship ID"]
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[doc = "Slot that is outfitted"]
    #[serde(rename = "Slot")]
    pub slot: ::std::string::String,
    #[doc = "If existing module in slot is stored"]
    #[serde(
        rename = "StoredItem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stored_item: ::std::option::Option<::std::string::String>,
    #[doc = "If existing module in slot is stored"]
    #[serde(
        rename = "StoredItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stored_item_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ModuleBuy> for ModuleBuy {
    fn from(value: &ModuleBuy) -> Self {
        value.clone()
    }
}

impl ModuleBuy {
    pub fn event_value() -> ::std::string::String {
        "ModuleBuy".to_string()
    }
}
