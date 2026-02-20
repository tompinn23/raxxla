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
#[doc = "Change to tax rate or reserve"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Change to tax rate or reserve\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"AvailableBalance\","]
#[doc = "    \"CarrierBalance\","]
#[doc = "    \"CarrierID\","]
#[doc = "    \"ReserveBalance\","]
#[doc = "    \"ReservePercent\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AvailableBalance\": {"]
#[doc = "      \"title\": \"AvailableBalance\","]
#[doc = "      \"examples\": ["]
#[doc = "        1483671510,"]
#[doc = "        437798365,"]
#[doc = "        955487317"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierBalance\": {"]
#[doc = "      \"title\": \"CarrierBalance\","]
#[doc = "      \"examples\": ["]
#[doc = "        1660931542,"]
#[doc = "        565786612,"]
#[doc = "        1083237005"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"CarrierID\": {"]
#[doc = "      \"title\": \"CarrierID\","]
#[doc = "      \"examples\": ["]
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
#[doc = "    \"ReserveBalance\": {"]
#[doc = "      \"title\": \"ReserveBalance\","]
#[doc = "      \"examples\": ["]
#[doc = "        127749688,"]
#[doc = "        0,"]
#[doc = "        60000000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ReservePercent\": {"]
#[doc = "      \"title\": \"ReservePercent\","]
#[doc = "      \"examples\": ["]
#[doc = "        8,"]
#[doc = "        23,"]
#[doc = "        12"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate\": {"]
#[doc = "      \"title\": \"TaxRate\","]
#[doc = "      \"examples\": ["]
#[doc = "        40,"]
#[doc = "        25,"]
#[doc = "        7"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate_outfitting\": {"]
#[doc = "      \"title\": \"TaxRate_outfitting\","]
#[doc = "      \"examples\": ["]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate_pioneersupplies\": {"]
#[doc = "      \"title\": \"TaxRate_pioneersupplies\","]
#[doc = "      \"examples\": ["]
#[doc = "        25,"]
#[doc = "        100"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate_rearm\": {"]
#[doc = "      \"title\": \"TaxRate_rearm\","]
#[doc = "      \"examples\": ["]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate_refuel\": {"]
#[doc = "      \"title\": \"TaxRate_refuel\","]
#[doc = "      \"examples\": ["]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate_repair\": {"]
#[doc = "      \"title\": \"TaxRate_repair\","]
#[doc = "      \"examples\": ["]
#[doc = "        25"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TaxRate_shipyard\": {"]
#[doc = "      \"title\": \"TaxRate_shipyard\","]
#[doc = "      \"examples\": ["]
#[doc = "        25"]
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
pub struct CarrierFinance {
    #[serde(rename = "AvailableBalance")]
    pub available_balance: i64,
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
    #[serde(skip_deserializing, default = "CarrierFinance::event_value")]
    pub event: String,
    #[serde(rename = "ReserveBalance")]
    pub reserve_balance: i64,
    #[serde(rename = "ReservePercent")]
    pub reserve_percent: i64,
    #[serde(
        rename = "TaxRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate: ::std::option::Option<i64>,
    #[serde(
        rename = "TaxRate_outfitting",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate_outfitting: ::std::option::Option<i64>,
    #[serde(
        rename = "TaxRate_pioneersupplies",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate_pioneersupplies: ::std::option::Option<i64>,
    #[serde(
        rename = "TaxRate_rearm",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate_rearm: ::std::option::Option<i64>,
    #[serde(
        rename = "TaxRate_refuel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate_refuel: ::std::option::Option<i64>,
    #[serde(
        rename = "TaxRate_repair",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate_repair: ::std::option::Option<i64>,
    #[serde(
        rename = "TaxRate_shipyard",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tax_rate_shipyard: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CarrierFinance> for CarrierFinance {
    fn from(value: &CarrierFinance) -> Self {
        value.clone()
    }
}

impl CarrierFinance {
    pub fn event_value() -> ::std::string::String {
        "CarrierFinance".to_string()
    }
}
