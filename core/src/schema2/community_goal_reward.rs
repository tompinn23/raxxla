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
#[doc = "When Written: when receiving a reward for a community goal"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when receiving a reward for a community goal\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CGID\","]
#[doc = "    \"Name\","]
#[doc = "    \"Reward\","]
#[doc = "    \"System\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CGID\": {"]
#[doc = "      \"title\": \"CGID\","]
#[doc = "      \"examples\": ["]
#[doc = "        754,"]
#[doc = "        758,"]
#[doc = "        690"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Operation Tyndareus Reacts to Thargoid Fleet\","]
#[doc = "        \"Disrupt Azimuth's Resource Campaign\","]
#[doc = "        \"The Colonia Bridge Project 2nd Phase - Alcor\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Reward\": {"]
#[doc = "      \"title\": \"Reward\","]
#[doc = "      \"examples\": ["]
#[doc = "        8000000,"]
#[doc = "        2400000,"]
#[doc = "        10000000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"System\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Andecavi\","]
#[doc = "        \"Wandrama\","]
#[doc = "        \"Alcor\""]
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
pub struct CommunityGoalReward {
    #[serde(rename = "CGID")]
    pub cgid: i64,
    #[serde(skip_deserializing, default = "CommunityGoalReward::event_value")]
    pub event: String,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(rename = "Reward")]
    pub reward: i64,
    #[serde(rename = "System")]
    pub system: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&CommunityGoalReward> for CommunityGoalReward {
    fn from(value: &CommunityGoalReward) -> Self {
        value.clone()
    }
}

impl CommunityGoalReward {
    pub fn event_value() -> ::std::string::String {
        "CommunityGoalReward".to_string()
    }
}
