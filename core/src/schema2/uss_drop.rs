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
#[doc = "When written: when dropping from Supercruise at a Unidentified Signal Source"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when dropping from Supercruise at a Unidentified Signal Source\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"USSThreat\","]
#[doc = "    \"USSType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"USSThreat\": {"]
#[doc = "      \"title\": \"USSThreat\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        6,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"USSType\": {"]
#[doc = "      \"title\": \"USSType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$USS_Type_Salvage;\","]
#[doc = "        \"$USS_Type_MissionTarget;\","]
#[doc = "        \"$USS_Type_ValuableSalvage;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"USSType_Localised\": {"]
#[doc = "      \"title\": \"USSType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Degraded emissions\","]
#[doc = "        \"Target\","]
#[doc = "        \"Encoded emissions\""]
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
pub struct USSDrop {
    #[serde(skip_deserializing, default = "USSDrop::event_value")]
    pub event: String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "USSThreat")]
    pub uss_threat: i64,
    #[serde(rename = "USSType")]
    pub uss_type: ::std::string::String,
    #[serde(
        rename = "USSType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub uss_type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&USSDrop> for USSDrop {
    fn from(value: &USSDrop) -> Self {
        value.clone()
    }
}

impl USSDrop {
    pub fn event_value() -> ::std::string::String {
        "USSDrop".to_string()
    }
}
