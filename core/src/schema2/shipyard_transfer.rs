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
#[doc = "When Written: when requesting a ship at another station be transported to this station"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when requesting a ship at another station be transported to this station\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Distance\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"ShipID\","]
#[doc = "    \"ShipMarketID\","]
#[doc = "    \"ShipType\","]
#[doc = "    \"System\","]
#[doc = "    \"TransferPrice\","]
#[doc = "    \"TransferTime\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Distance\": {"]
#[doc = "      \"title\": \"Distance\","]
#[doc = "      \"examples\": ["]
#[doc = "        5.600031,"]
#[doc = "        27306.621094,"]
#[doc = "        12865.205078"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128666762,"]
#[doc = "        3705689344,"]
#[doc = "        3229301760"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        25,"]
#[doc = "        34"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipMarketID\": {"]
#[doc = "      \"title\": \"ShipMarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        3705689344,"]
#[doc = "        128666762,"]
#[doc = "        3703568640"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipType\": {"]
#[doc = "      \"title\": \"ShipType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asp\","]
#[doc = "        \"Cutter\","]
#[doc = "        \"Python\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipType_Localised\": {"]
#[doc = "      \"title\": \"ShipType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asp Explorer\","]
#[doc = "        \"Imperial Cutter\","]
#[doc = "        \"Imperial Courier\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"System\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"description\": \"Location of the ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"LFT 926\","]
#[doc = "        \"Shinrarta Dezhra\","]
#[doc = "        \"Cloomeia FG-Y e95\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TransferPrice\": {"]
#[doc = "      \"title\": \"TransferPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        23053,"]
#[doc = "        300353,"]
#[doc = "        70642812"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TransferTime\": {"]
#[doc = "      \"title\": \"TransferTime\","]
#[doc = "      \"description\": \"Time taken in seconds\","]
#[doc = "      \"examples\": ["]
#[doc = "        356,"]
#[doc = "        273366,"]
#[doc = "        128952"]
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
pub struct ShipyardTransfer {
    #[serde(rename = "Distance")]
    pub distance: f64,
    #[serde(skip_deserializing, default = "ShipyardTransfer::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: i64,
    #[serde(rename = "ShipType")]
    pub ship_type: ::std::string::String,
    #[serde(
        rename = "ShipType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Location of the ship"]
    #[serde(rename = "System")]
    pub system: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "TransferPrice")]
    pub transfer_price: i64,
    #[doc = "Time taken in seconds"]
    #[serde(rename = "TransferTime")]
    pub transfer_time: i64,
}
impl ::std::convert::From<&ShipyardTransfer> for ShipyardTransfer {
    fn from(value: &ShipyardTransfer) -> Self {
        value.clone()
    }
}

impl ShipyardTransfer {
    pub fn event_value() -> ::std::string::String {
        "ShipyardTransfer".to_string()
    }
}
