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
#[doc = "Written when transferring items between backpack and ship locker"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Written when transferring items between backpack and ship locker\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Transfers\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Transfers\": {"]
#[doc = "      \"title\": \"Transfers\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Category\","]
#[doc = "          \"Direction\","]
#[doc = "          \"LockerNewCount\","]
#[doc = "          \"LockerOldCount\","]
#[doc = "          \"Name\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Category\": {"]
#[doc = "            \"title\": \"Category\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Consumable\","]
#[doc = "              \"Component\","]
#[doc = "              \"Item\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Direction\": {"]
#[doc = "            \"title\": \"Direction\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"ToBackpack\","]
#[doc = "              \"ToShipLocker\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"LockerNewCount\": {"]
#[doc = "            \"title\": \"LockerNewCount\","]
#[doc = "            \"examples\": ["]
#[doc = "              95,"]
#[doc = "              92,"]
#[doc = "              98"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"LockerOldCount\": {"]
#[doc = "            \"title\": \"LockerOldCount\","]
#[doc = "            \"examples\": ["]
#[doc = "              96,"]
#[doc = "              94,"]
#[doc = "              99"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"healthpack\","]
#[doc = "              \"energycell\","]
#[doc = "              \"amm_grenade_emp\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Medkit\","]
#[doc = "              \"Energy Cell\","]
#[doc = "              \"Shield Disruptor\""]
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
pub struct TransferMicroResources {
    #[serde(skip_deserializing, default = "TransferMicroResources::event_value")]
    pub event: String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Transfers")]
    pub transfers: ::std::vec::Vec<TransfersItem>,
}
impl ::std::convert::From<&TransferMicroResources> for TransferMicroResources {
    fn from(value: &TransferMicroResources) -> Self {
        value.clone()
    }
}
#[doc = "`TransfersItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Category\","]
#[doc = "    \"Direction\","]
#[doc = "    \"LockerNewCount\","]
#[doc = "    \"LockerOldCount\","]
#[doc = "    \"Name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Consumable\","]
#[doc = "        \"Component\","]
#[doc = "        \"Item\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Direction\": {"]
#[doc = "      \"title\": \"Direction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"ToBackpack\","]
#[doc = "        \"ToShipLocker\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"LockerNewCount\": {"]
#[doc = "      \"title\": \"LockerNewCount\","]
#[doc = "      \"examples\": ["]
#[doc = "        95,"]
#[doc = "        92,"]
#[doc = "        98"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LockerOldCount\": {"]
#[doc = "      \"title\": \"LockerOldCount\","]
#[doc = "      \"examples\": ["]
#[doc = "        96,"]
#[doc = "        94,"]
#[doc = "        99"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"healthpack\","]
#[doc = "        \"energycell\","]
#[doc = "        \"amm_grenade_emp\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Medkit\","]
#[doc = "        \"Energy Cell\","]
#[doc = "        \"Shield Disruptor\""]
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
pub struct TransfersItem {
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[serde(rename = "Direction")]
    pub direction: ::std::string::String,
    #[serde(rename = "LockerNewCount")]
    pub locker_new_count: i64,
    #[serde(rename = "LockerOldCount")]
    pub locker_old_count: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TransfersItem> for TransfersItem {
    fn from(value: &TransfersItem) -> Self {
        value.clone()
    }
}

impl TransferMicroResources {
    pub fn event_value() -> ::std::string::String {
        "TransferMicroResources".to_string()
    }
}
