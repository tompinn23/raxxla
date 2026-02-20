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
#[doc = "When written: when selling a module in storage at another station"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when selling a module in storage at another station\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"SellItem\","]
#[doc = "    \"SellPrice\","]
#[doc = "    \"ServerId\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"StorageSlot\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"SellItem\": {"]
#[doc = "      \"title\": \"SellItem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_hyperdrive_size3_class1_name;\","]
#[doc = "        \"$int_hyperdrive_size5_class1_name;\","]
#[doc = "        \"$int_lifesupport_size4_class1_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SellItem_Localised\": {"]
#[doc = "      \"title\": \"SellItem_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FSD\","]
#[doc = "        \"Life Support\","]
#[doc = "        \"Power Distributor\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SellPrice\": {"]
#[doc = "      \"title\": \"SellPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        6114,"]
#[doc = "        63012,"]
#[doc = "        11065"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ServerId\": {"]
#[doc = "      \"title\": \"ServerId\","]
#[doc = "      \"examples\": ["]
#[doc = "        128064108,"]
#[doc = "        128064118,"]
#[doc = "        128064153"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"empire_trader\","]
#[doc = "        \"raxxla\","]
#[doc = "        \"federation_corvette\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        11,"]
#[doc = "        27,"]
#[doc = "        29"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StorageSlot\": {"]
#[doc = "      \"title\": \"StorageSlot\","]
#[doc = "      \"examples\": ["]
#[doc = "        6,"]
#[doc = "        54,"]
#[doc = "        9"]
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
pub struct ModuleSellRemote {
    #[serde(skip_deserializing, default = "ModuleSellRemote::event_value")]
    pub event: String,
    #[serde(rename = "SellItem")]
    pub sell_item: ::std::string::String,
    #[serde(
        rename = "SellItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_item_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SellPrice")]
    pub sell_price: i64,
    #[serde(rename = "ServerId")]
    pub server_id: i64,
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[serde(rename = "StorageSlot")]
    pub storage_slot: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ModuleSellRemote> for ModuleSellRemote {
    fn from(value: &ModuleSellRemote) -> Self {
        value.clone()
    }
}

impl ModuleSellRemote {
    pub fn event_value() -> ::std::string::String {
        "ModuleSellRemote".to_string()
    }
}
