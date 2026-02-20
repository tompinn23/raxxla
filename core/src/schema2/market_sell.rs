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
#[doc = "When Written: when selling goods in the market"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when selling goods in the market\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"AvgPricePaid\","]
#[doc = "    \"Count\","]
#[doc = "    \"MarketID\","]
#[doc = "    \"SellPrice\","]
#[doc = "    \"TotalSale\","]
#[doc = "    \"Type\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AvgPricePaid\": {"]
#[doc = "      \"title\": \"AvgPricePaid\","]
#[doc = "      \"description\": \"Average price paid\","]
#[doc = "      \"examples\": ["]
#[doc = "        42,"]
#[doc = "        4666,"]
#[doc = "        585"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BlackMarket\": {"]
#[doc = "      \"title\": \"BlackMarket\","]
#[doc = "      \"description\": \"Whether selling in a black market\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        593,"]
#[doc = "        15,"]
#[doc = "        7"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"IllegalGoods\": {"]
#[doc = "      \"title\": \"IllegalGoods\","]
#[doc = "      \"description\": \"Whether goods are illegal here\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"MarketID\": {"]
#[doc = "      \"title\": \"MarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128666762,"]
#[doc = "        128675207,"]
#[doc = "        128679815"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SellPrice\": {"]
#[doc = "      \"title\": \"SellPrice\","]
#[doc = "      \"description\": \"Price per unit\","]
#[doc = "      \"examples\": ["]
#[doc = "        567,"]
#[doc = "        19180,"]
#[doc = "        16560"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StolenGoods\": {"]
#[doc = "      \"title\": \"StolenGoods\","]
#[doc = "      \"description\": \"Whether goods were stolen\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"TotalSale\": {"]
#[doc = "      \"title\": \"TotalSale\","]
#[doc = "      \"description\": \"Total sale value\","]
#[doc = "      \"examples\": ["]
#[doc = "        336231,"]
#[doc = "        287700,"]
#[doc = "        115920"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"water\","]
#[doc = "        \"xihecompanions\","]
#[doc = "        \"konggaale\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type_Localised\": {"]
#[doc = "      \"title\": \"Type_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Xihe Biomorphic Companions\","]
#[doc = "        \"Kongga Ale\","]
#[doc = "        \"Personal Weapons\""]
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
pub struct MarketSell {
    #[doc = "Average price paid"]
    #[serde(rename = "AvgPricePaid")]
    pub avg_price_paid: i64,
    #[doc = "Whether selling in a black market"]
    #[serde(
        rename = "BlackMarket",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub black_market: ::std::option::Option<bool>,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(skip_deserializing, default = "MarketSell::event_value")]
    pub event: String,
    #[doc = "Whether goods are illegal here"]
    #[serde(
        rename = "IllegalGoods",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub illegal_goods: ::std::option::Option<bool>,
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    #[doc = "Price per unit"]
    #[serde(rename = "SellPrice")]
    pub sell_price: i64,
    #[doc = "Whether goods were stolen"]
    #[serde(
        rename = "StolenGoods",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stolen_goods: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Total sale value"]
    #[serde(rename = "TotalSale")]
    pub total_sale: i64,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[doc = "The localised value will be omitted if it is exactly the same as Type"]
    #[serde(
        rename = "Type_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&MarketSell> for MarketSell {
    fn from(value: &MarketSell) -> Self {
        value.clone()
    }
}

impl MarketSell {
    pub fn event_value() -> ::std::string::String {
        "MarketSell".to_string()
    }
}
