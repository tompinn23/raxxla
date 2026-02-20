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
#[doc = "This event is logged when a player sells a flight suit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when a player sells a flight suit\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"Price\","]
#[doc = "    \"SuitID\","]
#[doc = "    \"SuitMods\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"explorationsuit_class1\","]
#[doc = "        \"utilitysuit_class1\","]
#[doc = "        \"tacticalsuit_class1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Artemis Suit\","]
#[doc = "        \"Maverick Suit\","]
#[doc = "        \"Dominator Suit\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Price\": {"]
#[doc = "      \"title\": \"Price\","]
#[doc = "      \"examples\": ["]
#[doc = "        90000,"]
#[doc = "        270000,"]
#[doc = "        450000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitID\": {"]
#[doc = "      \"title\": \"SuitID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1750365504380639,"]
#[doc = "        1700368583180959,"]
#[doc = "        1700368586180690"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitMods\": {"]
#[doc = "      \"title\": \"SuitMods\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"suit_increasedmeleedamage\","]
#[doc = "          \"suit_increasedsprintduration\","]
#[doc = "          \"suit_reducedtoolbatteryconsumption\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
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
pub struct SellSuit {
    #[serde(skip_deserializing, default = "SellSuit::event_value")]
    pub event: String,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Price")]
    pub price: i64,
    #[serde(rename = "SuitID")]
    pub suit_id: i64,
    #[serde(rename = "SuitMods")]
    pub suit_mods: ::std::vec::Vec<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&SellSuit> for SellSuit {
    fn from(value: &SellSuit) -> Self {
        value.clone()
    }
}

impl SellSuit {
    pub fn event_value() -> ::std::string::String {
        "SellSuit".to_string()
    }
}
