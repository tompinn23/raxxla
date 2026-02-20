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
#[doc = "When Written: when purchasing an SRV or Fighter"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when purchasing an SRV or Fighter\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Cost\","]
#[doc = "    \"Count\","]
#[doc = "    \"Loadout\","]
#[doc = "    \"Type\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Cost\": {"]
#[doc = "      \"title\": \"Cost\","]
#[doc = "      \"examples\": ["]
#[doc = "        5270,"]
#[doc = "        5139,"]
#[doc = "        1030"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"description\": \"Number of vehicles purchased\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ID\": {"]
#[doc = "      \"title\": \"ID\","]
#[doc = "      \"examples\": ["]
#[doc = "        33,"]
#[doc = "        37,"]
#[doc = "        34"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Loadout\": {"]
#[doc = "      \"title\": \"Loadout\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"starter\","]
#[doc = "        \"default\","]
#[doc = "        \"one\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"testbuggy\","]
#[doc = "        \"combat_multicrew_srv_01\","]
#[doc = "        \"independent_fighter\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Type_Localised\": {"]
#[doc = "      \"title\": \"Type_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"SRV Scarab\","]
#[doc = "        \"SRV Scorpion\","]
#[doc = "        \"Gu-97\""]
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
pub struct RestockVehicle {
    #[serde(rename = "Cost")]
    pub cost: i64,
    #[doc = "Number of vehicles purchased"]
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(skip_deserializing, default = "RestockVehicle::event_value")]
    pub event: String,
    #[serde(
        rename = "ID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<i64>,
    #[serde(rename = "Loadout")]
    pub loadout: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
    #[serde(
        rename = "Type_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&RestockVehicle> for RestockVehicle {
    fn from(value: &RestockVehicle) -> Self {
        value.clone()
    }
}

impl RestockVehicle {
    pub fn event_value() -> ::std::string::String {
        "RestockVehicle".to_string()
    }
}
