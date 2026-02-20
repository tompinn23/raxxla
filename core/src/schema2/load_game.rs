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
#[doc = "`LoadGame`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Commander\","]
#[doc = "    \"Credits\","]
#[doc = "    \"FID\","]
#[doc = "    \"Horizons\","]
#[doc = "    \"Loan\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Commander\": {"]
#[doc = "      \"title\": \"Commander\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Alice\","]
#[doc = "        \"Bob\","]
#[doc = "        \"Mallory\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Credits\": {"]
#[doc = "      \"title\": \"Credits\","]
#[doc = "      \"description\": \"Current credit balance\","]
#[doc = "      \"examples\": ["]
#[doc = "        1774886079,"]
#[doc = "        1775449365,"]
#[doc = "        1775448690"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"FID\": {"]
#[doc = "      \"title\": \"FID\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"F123456\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"FuelCapacity\": {"]
#[doc = "      \"title\": \"FuelCapacity\","]
#[doc = "      \"description\": \"Size of main tank\","]
#[doc = "      \"examples\": ["]
#[doc = "        32.0,"]
#[doc = "        1.0,"]
#[doc = "        16.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"FuelLevel\": {"]
#[doc = "      \"title\": \"FuelLevel\","]
#[doc = "      \"description\": \"Current fuel level\","]
#[doc = "      \"examples\": ["]
#[doc = "        31.873062,"]
#[doc = "        31.243063,"]
#[doc = "        1.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"GameMode\": {"]
#[doc = "      \"title\": \"GameMode\","]
#[doc = "      \"description\": \"Open, Solo or Group\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Solo\","]
#[doc = "        \"Group\","]
#[doc = "        \"Open\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Group\": {"]
#[doc = "      \"title\": \"Group\","]
#[doc = "      \"description\": \"name of group (if in a group session)\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Group A\","]
#[doc = "        \"Group B\","]
#[doc = "        \"Group C\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Horizons\": {"]
#[doc = "      \"title\": \"Horizons\","]
#[doc = "      \"description\": \"Whether Game has Horizons enabled.\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Loan\": {"]
#[doc = "      \"title\": \"Loan\","]
#[doc = "      \"description\": \"Current loan\","]
#[doc = "      \"examples\": ["]
#[doc = "        0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Odyssey\": {"]
#[doc = "      \"title\": \"Odyssey\","]
#[doc = "      \"description\": \"Whether Game has Odyssey enabled. Only present if in Odyssey game mode\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Ship\": {"]
#[doc = "      \"title\": \"Ship\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Krait_MkII\","]
#[doc = "        \"UtilitySuit_Class5\","]
#[doc = "        \"Asp\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipID\": {"]
#[doc = "      \"title\": \"ShipID\","]
#[doc = "      \"examples\": ["]
#[doc = "        18,"]
#[doc = "        4293000004,"]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ShipIdent\": {"]
#[doc = "      \"title\": \"ShipIdent\","]
#[doc = "      \"description\": \"User-defined ship ID string\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"ASP-01\","]
#[doc = "        \"\","]
#[doc = "        \"KM2-02\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ShipName\": {"]
#[doc = "      \"title\": \"ShipName\","]
#[doc = "      \"description\": \"User-defined ship name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Goidgobbler\","]
#[doc = "        \"\","]
#[doc = "        \"Starhopper\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Ship_Localised\": {"]
#[doc = "      \"title\": \"Ship_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Krait Mk II\","]
#[doc = "        \"$UtilitySuit_Class1_Name;\","]
#[doc = "        \"Asp Explorer\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StartDead\": {"]
#[doc = "      \"title\": \"StartDead\","]
#[doc = "      \"description\": \"only present if starting dead: see Resurrect\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"StartLanded\": {"]
#[doc = "      \"title\": \"StartLanded\","]
#[doc = "      \"description\": \"Only present if landed\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"build\": {"]
#[doc = "      \"title\": \"build\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"r282108/r0 \","]
#[doc = "        \"r282352/r0 \","]
#[doc = "        \"r284072/r0 \""]
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
#[doc = "    \"gameversion\": {"]
#[doc = "      \"title\": \"gameversion\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"4.0.0.1200\","]
#[doc = "        \"4.0.0.1201\","]
#[doc = "        \"4.0.0.1300\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"language\": {"]
#[doc = "      \"title\": \"language\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"English/UK\","]
#[doc = "        \"Russian/RU\","]
#[doc = "        \"French/FR\""]
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
pub struct LoadGame {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub build: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Commander")]
    pub commander: ::std::string::String,
    #[doc = "Current credit balance"]
    #[serde(rename = "Credits")]
    pub credits: i64,
    #[serde(skip_deserializing, default = "LoadGame::event_value")]
    pub event: String,
    #[serde(rename = "FID")]
    pub fid: ::std::string::String,
    #[serde(
        rename = "FuelCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fuel_capacity: ::std::option::Option<f64>,
    #[serde(
        rename = "FuelLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fuel_level: ::std::option::Option<f64>,
    #[doc = "Open, Solo or Group"]
    #[serde(
        rename = "GameMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub game_mode: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gameversion: ::std::option::Option<::std::string::String>,
    #[doc = "name of group (if in a group session)"]
    #[serde(
        rename = "Group",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group: ::std::option::Option<::std::string::String>,
    #[doc = "Whether Game has Horizons enabled."]
    #[serde(rename = "Horizons")]
    pub horizons: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "Current loan"]
    #[serde(rename = "Loan")]
    pub loan: i64,
    #[doc = "Whether Game has Odyssey enabled. Only present if in Odyssey game mode"]
    #[serde(
        rename = "Odyssey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub odyssey: ::std::option::Option<bool>,
    #[serde(
        rename = "Ship",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ShipID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_id: ::std::option::Option<i64>,
    #[doc = "User-defined ship ID string"]
    #[serde(
        rename = "ShipIdent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_ident: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Ship_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_localised: ::std::option::Option<::std::string::String>,
    #[doc = "User-defined ship name"]
    #[serde(
        rename = "ShipName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ship_name: ::std::option::Option<::std::string::String>,
    #[doc = "only present if starting dead: see Resurrect"]
    #[serde(
        rename = "StartDead",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_dead: ::std::option::Option<bool>,
    #[doc = "Only present if landed"]
    #[serde(
        rename = "StartLanded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_landed: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&LoadGame> for LoadGame {
    fn from(value: &LoadGame) -> Self {
        value.clone()
    }
}

impl LoadGame {
    pub fn event_value() -> ::std::string::String {
        "LoadGame".to_string()
    }
}
