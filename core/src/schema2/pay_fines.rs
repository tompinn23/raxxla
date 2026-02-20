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
#[doc = "When written: when paying fines"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when paying fines\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Amount\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AllFines\": {"]
#[doc = "      \"title\": \"AllFines\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Amount\": {"]
#[doc = "      \"title\": \"Amount\","]
#[doc = "      \"description\": \"Total amount paid, including any broker fee\","]
#[doc = "      \"examples\": ["]
#[doc = "        550217,"]
#[doc = "        4875,"]
#[doc = "        23875"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BrokerPercentage\": {"]
#[doc = "      \"title\": \"BrokerPercentage\","]
#[doc = "      \"description\": \"Present if paid via a broker\","]
#[doc = "      \"examples\": ["]
#[doc = "        25.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"description\": \"If paying off an individual faction's fines\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Harow Commodities\","]
#[doc = "        \"Cara Partnership\","]
#[doc = "        \"HIP 44997 Legal Partners\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Faction_Localised\": {"]
#[doc = "      \"title\": \"Faction_Localised\","]
#[doc = "      \"description\": \"If paying off an individual faction's fines\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Independent\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        12,"]
#[doc = "        34"]
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
pub struct PayFines {
    #[serde(
        rename = "AllFines",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub all_fines: ::std::option::Option<bool>,
    #[doc = "Total amount paid, including any broker fee"]
    #[serde(rename = "Amount")]
    pub amount: i64,
    #[serde(
        rename = "BrokerPercentage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub broker_percentage: ::std::option::Option<f64>,
    #[serde(skip_deserializing, default = "PayFines::event_value")]
    pub event: String,
    #[doc = "If paying off an individual faction's fines"]
    #[serde(
        rename = "Faction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction: ::std::option::Option<::std::string::String>,
    #[doc = "If paying off an individual faction's fines"]
    #[serde(
        rename = "Faction_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub faction_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&PayFines> for PayFines {
    fn from(value: &PayFines) -> Self {
        value.clone()
    }
}

impl PayFines {
    pub fn event_value() -> ::std::string::String {
        "PayFines".to_string()
    }
}
