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
#[doc = "When written: when performing a full system scan (Honk)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when performing a full system scan (Honk)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyCount\","]
#[doc = "    \"NonBodyCount\","]
#[doc = "    \"Progress\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"SystemName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BodyCount\": {"]
#[doc = "      \"title\": \"BodyCount\","]
#[doc = "      \"examples\": ["]
#[doc = "        14,"]
#[doc = "        21,"]
#[doc = "        40"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"NonBodyCount\": {"]
#[doc = "      \"title\": \"NonBodyCount\","]
#[doc = "      \"examples\": ["]
#[doc = "        15,"]
#[doc = "        7,"]
#[doc = "        24"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Progress\": {"]
#[doc = "      \"title\": \"Progress\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0,"]
#[doc = "        0.391172,"]
#[doc = "        0.263902"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        732048640355,"]
#[doc = "        7269097743753,"]
#[doc = "        4030600300907"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SystemName\": {"]
#[doc = "      \"title\": \"SystemName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 21383\","]
#[doc = "        \"Turdet\","]
#[doc = "        \"Koleti\""]
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
pub struct FSSDiscoveryScan {
    #[serde(rename = "BodyCount")]
    pub body_count: i64,
    #[serde(skip_deserializing, default = "FSSDiscoveryScan::event_value")]
    pub event: String,
    #[serde(rename = "NonBodyCount")]
    pub non_body_count: i64,
    #[serde(rename = "Progress")]
    pub progress: f64,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[serde(rename = "SystemName")]
    pub system_name: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&FSSDiscoveryScan> for FSSDiscoveryScan {
    fn from(value: &FSSDiscoveryScan) -> Self {
        value.clone()
    }
}

impl FSSDiscoveryScan {
    pub fn event_value() -> ::std::string::String {
        "FSSDiscoveryScan".to_string()
    }
}
