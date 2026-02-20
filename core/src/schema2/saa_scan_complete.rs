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
#[doc = "When written: after using the Surface Area Analysis Scanner"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: after using the Surface Area Analysis Scanner\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyID\","]
#[doc = "    \"BodyName\","]
#[doc = "    \"EfficiencyTarget\","]
#[doc = "    \"ProbesUsed\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        23,"]
#[doc = "        22,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyName\": {"]
#[doc = "      \"title\": \"BodyName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Swoiwns ZD-B d1 1 B Ring\","]
#[doc = "        \"Swoiwns ZD-B d1 1 A Ring\","]
#[doc = "        \"HIP 62918 1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"EfficiencyTarget\": {"]
#[doc = "      \"title\": \"EfficiencyTarget\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        8,"]
#[doc = "        6"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ProbesUsed\": {"]
#[doc = "      \"title\": \"ProbesUsed\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        5,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        44753062275,"]
#[doc = "        743701424945,"]
#[doc = "        2831053823913"]
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
pub struct SAAScanComplete {
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(rename = "BodyName")]
    pub body_name: ::std::string::String,
    #[serde(rename = "EfficiencyTarget")]
    pub efficiency_target: i64,
    #[serde(skip_deserializing, default = "SAAScanComplete::event_value")]
    pub event: String,
    #[serde(rename = "ProbesUsed")]
    pub probes_used: i64,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&SAAScanComplete> for SAAScanComplete {
    fn from(value: &SAAScanComplete) -> Self {
        value.clone()
    }
}

impl SAAScanComplete {
    pub fn event_value() -> ::std::string::String {
        "SAAScanComplete".to_string()
    }
}
