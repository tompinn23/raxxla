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
#[doc = "When plotting a multi-star route, the file NavRoute.json is written in the same directory as the journal, with a list of stars along that route"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When plotting a multi-star route, the file NavRoute.json is written in the same directory as the journal, with a list of stars along that route\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Route\": {"]
#[doc = "      \"title\": \"Route\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"StarClass\","]
#[doc = "          \"StarPos\","]
#[doc = "          \"StarSystem\","]
#[doc = "          \"SystemAddress\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"StarClass\": {"]
#[doc = "            \"title\": \"StarClass\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"G\","]
#[doc = "              \"M\","]
#[doc = "              \"DQ\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"StarPos\": {"]
#[doc = "            \"title\": \"StarPos\","]
#[doc = "            \"description\": \"star position, as a Json array [x, y, z], relative to Sol in Ly\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"examples\": ["]
#[doc = "                -81.09375,"]
#[doc = "                -148.3125,"]
#[doc = "                -337.09375"]
#[doc = "              ],"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"maxItems\": 3,"]
#[doc = "            \"minItems\": 3"]
#[doc = "          },"]
#[doc = "          \"StarSystem\": {"]
#[doc = "            \"title\": \"StarSystem\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"i Bootis\","]
#[doc = "              \"Acihaut\","]
#[doc = "              \"LHS 455\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"SystemAddress\": {"]
#[doc = "            \"title\": \"SystemAddress\","]
#[doc = "            \"examples\": ["]
#[doc = "              1281787693419,"]
#[doc = "              11665802405289,"]
#[doc = "              3686969379179"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
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
pub struct NavRoute {
    #[serde(skip_deserializing, default = "NavRoute::event_value")]
    pub event: String,
    #[serde(
        rename = "Route",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub route: ::std::vec::Vec<RouteItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&NavRoute> for NavRoute {
    fn from(value: &NavRoute) -> Self {
        value.clone()
    }
}
#[doc = "`RouteItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"StarClass\","]
#[doc = "    \"StarPos\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"SystemAddress\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"StarClass\": {"]
#[doc = "      \"title\": \"StarClass\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"G\","]
#[doc = "        \"M\","]
#[doc = "        \"DQ\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StarPos\": {"]
#[doc = "      \"title\": \"StarPos\","]
#[doc = "      \"description\": \"star position, as a Json array [x, y, z], relative to Sol in Ly\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          -81.09375,"]
#[doc = "          -148.3125,"]
#[doc = "          -337.09375"]
#[doc = "        ],"]
#[doc = "        \"type\": \"number\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 3,"]
#[doc = "      \"minItems\": 3"]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"i Bootis\","]
#[doc = "        \"Acihaut\","]
#[doc = "        \"LHS 455\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        1281787693419,"]
#[doc = "        11665802405289,"]
#[doc = "        3686969379179"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RouteItem {
    #[serde(rename = "StarClass")]
    pub star_class: ::std::string::String,
    #[doc = "star position, as a Json array [x, y, z], relative to Sol in Ly"]
    #[serde(rename = "StarPos")]
    pub star_pos: [f64; 3usize],
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
}
impl ::std::convert::From<&RouteItem> for RouteItem {
    fn from(value: &RouteItem) -> Self {
        value.clone()
    }
}

impl NavRoute {
    pub fn event_value() -> ::std::string::String {
        "NavRoute".to_string()
    }
}
