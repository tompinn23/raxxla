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
#[doc = "When written: when requesting a module is transferred from storage at another station"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when requesting a module is transferred from storage at another station\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ServerId\","]
#[doc = "    \"Ship\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"StorageSlot\","]
#[doc = "    \"StoredItem\","]
#[doc = "    \"TransferCost\","]
#[doc = "    \"TransferTime\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ServerId\": {"]
#[doc = "      \"title\": \"ServerId\","]
#[doc = "      \"examples\": ["]
#[doc = "        129001926,"]
#[doc = "        129001922,"]
#[doc = "        129001924"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"asp\","]
#[doc = "        \"anaconda\","]
#[doc = "        \"federation_corvette\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        7,"]
#[doc = "        29"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StorageSlot\": {"]
#[doc = "      \"title\": \"StorageSlot\","]
#[doc = "      \"examples\": ["]
#[doc = "        100,"]
#[doc = "        101,"]
#[doc = "        87"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StoredItem\": {"]
#[doc = "      \"title\": \"StoredItem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$int_multidronecontrol_rescue_size3_class3_name;\","]
#[doc = "        \"$int_multidronecontrol_mining_size3_class3_name;\","]
#[doc = "        \"$int_multidronecontrol_operations_size3_class4_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoredItem_Localised\": {"]
#[doc = "      \"title\": \"StoredItem_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Rescue Multi-Limpet Controller\","]
#[doc = "        \"Mining Multi-Limpet Controller\","]
#[doc = "        \"Operations Multi-Limpet Controller\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TransferCost\": {"]
#[doc = "      \"title\": \"TransferCost\","]
#[doc = "      \"examples\": ["]
#[doc = "        1166,"]
#[doc = "        1805,"]
#[doc = "        176"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TransferTime\": {"]
#[doc = "      \"title\": \"TransferTime\","]
#[doc = "      \"description\": \"In seconds\","]
#[doc = "      \"examples\": ["]
#[doc = "        4316,"]
#[doc = "        356,"]
#[doc = "        2564"]
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
pub struct FetchRemoteModule {
    #[serde(skip_deserializing, default = "FetchRemoteModule::event_value")]
    pub event: String,
    #[serde(rename = "ServerId")]
    pub server_id: i64,
    #[serde(rename = "Ship")]
    pub ship: ::std::string::String,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[serde(rename = "StorageSlot")]
    pub storage_slot: i64,
    #[serde(rename = "StoredItem")]
    pub stored_item: ::std::string::String,
    #[serde(
        rename = "StoredItem_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stored_item_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "TransferCost")]
    pub transfer_cost: i64,
    #[doc = "In seconds"]
    #[serde(rename = "TransferTime")]
    pub transfer_time: i64,
}
impl ::std::convert::From<&FetchRemoteModule> for FetchRemoteModule {
    fn from(value: &FetchRemoteModule) -> Self {
        value.clone()
    }
}

impl FetchRemoteModule {
    pub fn event_value() -> ::std::string::String {
        "FetchRemoteModule".to_string()
    }
}
