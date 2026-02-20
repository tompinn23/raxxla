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
#[doc = "This event is logged when buying a new suit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when buying a new suit\","]
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
#[doc = "        \"ExplorationSuit_Class2\","]
#[doc = "        \"UtilitySuit_Class3\","]
#[doc = "        \"UtilitySuit_Class2\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$ExplorationSuit_Class1_Name;\","]
#[doc = "        \"$UtilitySuit_Class1_Name;\","]
#[doc = "        \"$TacticalSuit_Class1_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Price\": {"]
#[doc = "      \"title\": \"Price\","]
#[doc = "      \"examples\": ["]
#[doc = "        2250000,"]
#[doc = "        750000,"]
#[doc = "        3750000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitID\": {"]
#[doc = "      \"title\": \"SuitID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1734434022684162,"]
#[doc = "        1734434028284341,"]
#[doc = "        1701563085654285"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitMods\": {"]
#[doc = "      \"title\": \"SuitMods\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"suit_backpackcapacity\","]
#[doc = "          \"suit_increasedammoreserves\","]
#[doc = "          \"suit_adsmovementspeed\""]
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
pub struct BuySuit {
    #[serde(skip_deserializing, default = "BuySuit::event_value")]
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
impl ::std::convert::From<&BuySuit> for BuySuit {
    fn from(value: &BuySuit) -> Self {
        value.clone()
    }
}

impl BuySuit {
    pub fn event_value() -> ::std::string::String {
        "BuySuit".to_string()
    }
}
