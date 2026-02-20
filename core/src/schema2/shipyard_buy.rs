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
#[doc = "When Written: when buying a new ship in the shipyard"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when buying a new ship in the shipyard\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MarketID\","]
#[doc = "    \"ShipPrice\","]
#[doc = "    \"ShipType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128666762,"]
#[doc = "        128803246,"]
#[doc = "        3226858240"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SellOldShip\": {"]
#[doc = "      \"title\": \"SellOldShip\","]
#[doc = "      \"description\": \"If selling current ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Type9\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SellPrice\": {"]
#[doc = "      \"title\": \"SellPrice\","]
#[doc = "      \"description\": \"If selling current ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        56166444"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SellShipID\": {"]
#[doc = "      \"title\": \"SellShipID\","]
#[doc = "      \"description\": \"If selling current ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        24"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipPrice\": {"]
#[doc = "      \"title\": \"ShipPrice\","]
#[doc = "      \"examples\": ["]
#[doc = "        5995039,"]
#[doc = "        132272506,"]
#[doc = "        6494626"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipType\": {"]
#[doc = "      \"title\": \"ShipType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"asp\","]
#[doc = "        \"anaconda\","]
#[doc = "        \"sidewinder\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipType_Localised\": {"]
#[doc = "      \"title\": \"ShipType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asp Explorer\","]
#[doc = "        \"Krait Phantom\","]
#[doc = "        \"Type-7 Transporter\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoreOldShip\": {"]
#[doc = "      \"title\": \"StoreOldShip\","]
#[doc = "      \"description\": \"If storing old ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asp\","]
#[doc = "        \"Type9\","]
#[doc = "        \"Anaconda\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StoreShipID\": {"]
#[doc = "      \"title\": \"StoreShipID\","]
#[doc = "      \"description\": \"If storing old ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        34,"]
#[doc = "        1"]
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
pub struct ShipyardBuy {
    #[serde(skip_deserializing, default = "ShipyardBuy::event_value")]
    pub event: String,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "If selling current ship"]
    #[serde(
        rename = "SellOldShip",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_old_ship: ::std::option::Option<::std::string::String>,
    #[doc = "If selling current ship"]
    #[serde(
        rename = "SellPrice",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_price: ::std::option::Option<i64>,
    #[doc = "If selling current ship"]
    #[serde(
        rename = "SellShipID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_ship_id: ::std::option::Option<i64>,
    #[serde(rename = "ShipPrice")]
    pub ship_price: i64,
    #[serde(rename = "ShipType")]
    pub ship_type: ::std::string::String,
    #[serde(
        rename = "ShipType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "If storing old ship"]
    #[serde(
        rename = "StoreOldShip",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_old_ship: ::std::option::Option<::std::string::String>,
    #[doc = "If storing old ship"]
    #[serde(
        rename = "StoreShipID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_ship_id: ::std::option::Option<i64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ShipyardBuy> for ShipyardBuy {
    fn from(value: &ShipyardBuy) -> Self {
        value.clone()
    }
}

impl ShipyardBuy {
    pub fn event_value() -> ::std::string::String {
        "ShipyardBuy".to_string()
    }
}
