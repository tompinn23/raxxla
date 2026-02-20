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
#[doc = "This event is logged when the player deletes a suit loadout"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when the player deletes a suit loadout\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"LoadoutID\","]
#[doc = "    \"LoadoutName\","]
#[doc = "    \"SuitID\","]
#[doc = "    \"SuitName\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"LoadoutID\": {"]
#[doc = "      \"title\": \"LoadoutID\","]
#[doc = "      \"examples\": ["]
#[doc = "        4293000023,"]
#[doc = "        4293000000,"]
#[doc = "        4293000006"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LoadoutName\": {"]
#[doc = "      \"title\": \"LoadoutName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Scavenger\","]
#[doc = "        \"Tanky\","]
#[doc = "        \"Explorer\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SuitID\": {"]
#[doc = "      \"title\": \"SuitID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1722478814612428,"]
#[doc = "        1703397409838859,"]
#[doc = "        1700215335054978"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitName\": {"]
#[doc = "      \"title\": \"SuitName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"utilitysuit_class2\","]
#[doc = "        \"tacticalsuit_class3\","]
#[doc = "        \"flightsuit\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SuitName_Localised\": {"]
#[doc = "      \"title\": \"SuitName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$UtilitySuit_Class1_Name;\","]
#[doc = "        \"$TacticalSuit_Class1_Name;\","]
#[doc = "        \"Flight Suit\""]
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
pub struct DeleteSuitLoadout {
    #[serde(skip_deserializing, default = "DeleteSuitLoadout::event_value")]
    pub event: String,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: i64,
    #[serde(rename = "LoadoutName")]
    pub loadout_name: ::std::string::String,
    #[serde(rename = "SuitID")]
    pub suit_id: i64,
    #[serde(rename = "SuitName")]
    pub suit_name: ::std::string::String,
    #[serde(
        rename = "SuitName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub suit_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&DeleteSuitLoadout> for DeleteSuitLoadout {
    fn from(value: &DeleteSuitLoadout) -> Self {
        value.clone()
    }
}

impl DeleteSuitLoadout {
    pub fn event_value() -> ::std::string::String {
        "DeleteSuitLoadout".to_string()
    }
}
