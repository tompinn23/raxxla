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
#[doc = "This event is written when completing a Full Spectrum Scan of a starsystem, to list the number of SAA signals found in the system (as shown in-game in the top-right panel)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is written when completing a Full Spectrum Scan of a starsystem, to list the number of SAA signals found in the system (as shown in-game in the top-right panel)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyID\","]
#[doc = "    \"BodyName\","]
#[doc = "    \"Signals\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        33,"]
#[doc = "        2,"]
#[doc = "        47"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyName\": {"]
#[doc = "      \"title\": \"BodyName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"BS Crucis 5 a\","]
#[doc = "        \"BS Crucis 1\","]
#[doc = "        \"BS Crucis 7 c a\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Signals\": {"]
#[doc = "      \"title\": \"Signals\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Count\","]
#[doc = "          \"Type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              3,"]
#[doc = "              2,"]
#[doc = "              1"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Type\": {"]
#[doc = "            \"title\": \"Type\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$SAA_SignalType_Geological;\","]
#[doc = "              \"$SAA_SignalType_Biological;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Type_Localised\": {"]
#[doc = "            \"title\": \"Type_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Geological\","]
#[doc = "              \"Biological\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        10043270500,"]
#[doc = "        2943530117929,"]
#[doc = "        2944872557345"]
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
pub struct FSSBodySignals {
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "BodyName")]
    pub body_name: ::std::string::String,
    #[serde(skip_deserializing, default = "FSSBodySignals::event_value")]
    pub event: String,
    #[serde(rename = "Signals")]
    pub signals: ::std::vec::Vec<SignalsItem>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&FSSBodySignals> for FSSBodySignals {
    fn from(value: &FSSBodySignals) -> Self {
        value.clone()
    }
}
#[doc = "`SignalsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"Type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        3,"]
#[doc = "        2,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$SAA_SignalType_Geological;\","]
#[doc = "        \"$SAA_SignalType_Biological;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type_Localised\": {"]
#[doc = "      \"title\": \"Type_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Geological\","]
#[doc = "        \"Biological\""]
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
pub struct SignalsItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[serde(
        rename = "Type_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SignalsItem> for SignalsItem {
    fn from(value: &SignalsItem) -> Self {
        value.clone()
    }
}

impl FSSBodySignals {
    pub fn event_value() -> ::std::string::String {
        "FSSBodySignals".to_string()
    }
}
