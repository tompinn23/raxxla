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
#[doc = "When transferring cargo between ship and fleet carrier, or between ship and SRV"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When transferring cargo between ship and fleet carrier, or between ship and SRV\","]
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
#[doc = "          \"Count\","]
#[doc = "          \"Direction\","]
#[doc = "          \"Type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              1,"]
#[doc = "              3,"]
#[doc = "              52"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Direction\": {"]
#[doc = "            \"title\": \"Direction\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"toship\","]
#[doc = "              \"tocarrier\","]
#[doc = "              \"tosrv\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"MissionID\": {"]
#[doc = "            \"title\": \"MissionID\","]
#[doc = "            \"examples\": ["]
#[doc = "              895910576,"]
#[doc = "              895910565,"]
#[doc = "              897152407"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Type\": {"]
#[doc = "            \"title\": \"Type\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"drones\","]
#[doc = "              \"metaalloys\","]
#[doc = "              \"platinum\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Type_Localised\": {"]
#[doc = "            \"title\": \"Type_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Limpet\","]
#[doc = "              \"Meta-Alloys\","]
#[doc = "              \"Thargoid Biological Matter\""]
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
pub struct CargoTransfer {
    #[serde(skip_deserializing, default = "CargoTransfer::event_value")]
    pub event: String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Transfers")]
    pub transfers: ::std::vec::Vec<TransfersItem>,
}
impl ::std::convert::From<&CargoTransfer> for CargoTransfer {
    fn from(value: &CargoTransfer) -> Self {
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
#[doc = "    \"Count\","]
#[doc = "    \"Direction\","]
#[doc = "    \"Type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        3,"]
#[doc = "        52"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Direction\": {"]
#[doc = "      \"title\": \"Direction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"toship\","]
#[doc = "        \"tocarrier\","]
#[doc = "        \"tosrv\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        895910576,"]
#[doc = "        895910565,"]
#[doc = "        897152407"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"drones\","]
#[doc = "        \"metaalloys\","]
#[doc = "        \"platinum\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type_Localised\": {"]
#[doc = "      \"title\": \"Type_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Limpet\","]
#[doc = "        \"Meta-Alloys\","]
#[doc = "        \"Thargoid Biological Matter\""]
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
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Direction")]
    pub direction: ::std::string::String,
    #[serde(
        rename = "MissionID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mission_id: ::std::option::Option<i64>,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[serde(
        rename = "Type_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TransfersItem> for TransfersItem {
    fn from(value: &TransfersItem) -> Self {
        value.clone()
    }
}

impl CargoTransfer {
    pub fn event_value() -> ::std::string::String {
        "CargoTransfer".to_string()
    }
}
