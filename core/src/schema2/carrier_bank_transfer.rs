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
#[doc = "Player transfers credits to/from carrier"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Player transfers credits to/from carrier\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CarrierBalance\","]
#[doc = "    \"CarrierID\","]
#[doc = "    \"PlayerBalance\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CarrierBalance\": {"]
#[doc = "      \"title\": \"CarrierBalance\","]
#[doc = "      \"description\": \"Carrier balance after transfer\","]
#[doc = "      \"examples\": ["]
#[doc = "        1568400000,"]
#[doc = "        3058818193,"]
#[doc = "        1490320265"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierID\": {"]
#[doc = "      \"title\": \"CarrierID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3707430144,"]
#[doc = "        3705689344,"]
#[doc = "        3705124096"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierType\": {"]
#[doc = "      \"title\": \"CarrierType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"FleetCarrier\","]
#[doc = "        \"SquadronCarrier\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Deposit\": {"]
#[doc = "      \"title\": \"Deposit\","]
#[doc = "      \"examples\": ["]
#[doc = "        1568400000,"]
#[doc = "        2122500000,"]
#[doc = "        872810000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"PlayerBalance\": {"]
#[doc = "      \"title\": \"PlayerBalance\","]
#[doc = "      \"description\": \"Player balance after transfer\","]
#[doc = "      \"examples\": ["]
#[doc = "        500008232,"]
#[doc = "        2435383946,"]
#[doc = "        173416954"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Withdraw\": {"]
#[doc = "      \"title\": \"Withdraw\","]
#[doc = "      \"examples\": ["]
#[doc = "        4060,"]
#[doc = "        869994,"]
#[doc = "        2370000"]
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
pub struct CarrierBankTransfer {
    #[doc = "Carrier balance after transfer"]
    #[serde(rename = "CarrierBalance")]
    pub carrier_balance: i64,
    #[serde(rename = "CarrierID")]
    pub carrier_id: i64,
    #[serde(
        rename = "CarrierType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub carrier_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Deposit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deposit: ::std::option::Option<i64>,
    #[serde(skip_deserializing, default = "CarrierBankTransfer::event_value")]
    pub event: String,
    #[doc = "Player balance after transfer"]
    #[serde(rename = "PlayerBalance")]
    pub player_balance: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "Withdraw",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub withdraw: ::std::option::Option<i64>,
}
impl ::std::convert::From<&CarrierBankTransfer> for CarrierBankTransfer {
    fn from(value: &CarrierBankTransfer) -> Self {
        value.clone()
    }
}

impl CarrierBankTransfer {
    pub fn event_value() -> ::std::string::String {
        "CarrierBankTransfer".to_string()
    }
}
