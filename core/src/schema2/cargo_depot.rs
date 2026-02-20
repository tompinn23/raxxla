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
#[doc = "When written: when collecting or delivering cargo for a wing mission, or if a wing member updates progress. The CargoType and Count are included when you collect or deliver goods, they are not included for a wing update. The Progress value actually represents pending progress for goods in transit: (ItemsCollected-ItemsDelivered)/TotalItemsToDeliver"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when collecting or delivering cargo for a wing mission, or if a wing member updates progress. The CargoType and Count are included when you collect or deliver goods, they are not included for a wing update. The Progress value actually represents pending progress for goods in transit: (ItemsCollected-ItemsDelivered)/TotalItemsToDeliver\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"EndMarketID\","]
#[doc = "    \"ItemsCollected\","]
#[doc = "    \"ItemsDelivered\","]
#[doc = "    \"MissionID\","]
#[doc = "    \"Progress\","]
#[doc = "    \"StartMarketID\","]
#[doc = "    \"TotalItemsToDeliver\","]
#[doc = "    \"UpdateType\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CargoType\": {"]
#[doc = "      \"title\": \"CargoType\","]
#[doc = "      \"description\": \"Not included for UpdateType of WingUpdate\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Water\","]
#[doc = "        \"FoodCartridges\","]
#[doc = "        \"Fish\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"CargoType_Localised\": {"]
#[doc = "      \"title\": \"CargoType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Food Cartridges\","]
#[doc = "        \"Advanced Medicines\","]
#[doc = "        \"Reactive Armour\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"description\": \"Not included for UpdateType of WingUpdate\","]
#[doc = "      \"examples\": ["]
#[doc = "        195,"]
#[doc = "        220,"]
#[doc = "        305"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"EndMarketID\": {"]
#[doc = "      \"title\": \"EndMarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128666762,"]
#[doc = "        3537185280,"]
#[doc = "        3228917248"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ItemsCollected\": {"]
#[doc = "      \"title\": \"ItemsCollected\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        66,"]
#[doc = "        196"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ItemsDelivered\": {"]
#[doc = "      \"title\": \"ItemsDelivered\","]
#[doc = "      \"examples\": ["]
#[doc = "        195,"]
#[doc = "        220,"]
#[doc = "        305"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        890359772,"]
#[doc = "        890359794,"]
#[doc = "        890367421"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Progress\": {"]
#[doc = "      \"title\": \"Progress\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.0,"]
#[doc = "        1.0,"]
#[doc = "        0.004545"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"StartMarketID\": {"]
#[doc = "      \"title\": \"StartMarketID\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        3228917248,"]
#[doc = "        128858186"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TotalItemsToDeliver\": {"]
#[doc = "      \"title\": \"TotalItemsToDeliver\","]
#[doc = "      \"examples\": ["]
#[doc = "        195,"]
#[doc = "        220,"]
#[doc = "        310"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"UpdateType\": {"]
#[doc = "      \"title\": \"UpdateType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Deliver\","]
#[doc = "        \"Collect\","]
#[doc = "        \"WingUpdate\""]
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
pub struct CargoDepot {
    #[doc = "Not included for UpdateType of WingUpdate"]
    #[serde(
        rename = "CargoType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cargo_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "CargoType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cargo_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Not included for UpdateType of WingUpdate"]
    #[serde(
        rename = "Count",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub count: ::std::option::Option<i64>,
    #[serde(rename = "EndMarketID")]
    pub end_market_id: i64,
    #[serde(skip_deserializing, default = "CargoDepot::event_value")]
    pub event: String,
    #[serde(rename = "ItemsCollected")]
    pub items_collected: i64,
    #[serde(rename = "ItemsDelivered")]
    pub items_delivered: i64,
    #[serde(rename = "MissionID")]
    pub mission_id: i64,
    #[serde(rename = "Progress")]
    pub progress: f64,
    #[serde(rename = "StartMarketID")]
    pub start_market_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "TotalItemsToDeliver")]
    pub total_items_to_deliver: i64,
    #[serde(rename = "UpdateType")]
    pub update_type: ::std::string::String,
}
impl ::std::convert::From<&CargoDepot> for CargoDepot {
    fn from(value: &CargoDepot) -> Self {
        value.clone()
    }
}

impl CargoDepot {
    pub fn event_value() -> ::std::string::String {
        "CargoDepot".to_string()
    }
}
