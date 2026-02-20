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
#[doc = "When Written: when a screen snapshot is saved. The latitude, longitude, altitude and heading will be included if on a planet or in low-altitude flight."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when a screen snapshot is saved. The latitude, longitude, altitude and heading will be included if on a planet or in low-altitude flight.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Filename\","]
#[doc = "    \"Height\","]
#[doc = "    \"Width\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Altitude\": {"]
#[doc = "      \"title\": \"Altitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.034059,"]
#[doc = "        1911.993042,"]
#[doc = "        68.702759"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Eocs Phyloea LD-K d8-0 ABC\","]
#[doc = "        \"Dryao Ain QD-K c8-0\","]
#[doc = "        \"Shrogaae KK-A d983 A 1 A Ring\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Filename\": {"]
#[doc = "      \"title\": \"Filename\","]
#[doc = "      \"description\": \"Filename of screenshot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Screenshot_0000.bmp\","]
#[doc = "        \"Screenshot_0001.bmp\","]
#[doc = "        \"Screenshot_0002.bmp\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Heading\": {"]
#[doc = "      \"title\": \"Heading\","]
#[doc = "      \"examples\": ["]
#[doc = "        247,"]
#[doc = "        0,"]
#[doc = "        287"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Height\": {"]
#[doc = "      \"title\": \"Height\","]
#[doc = "      \"examples\": ["]
#[doc = "        1440,"]
#[doc = "        6400,"]
#[doc = "        2160"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Latitude\": {"]
#[doc = "      \"title\": \"Latitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        -1.477483,"]
#[doc = "        -40.646881,"]
#[doc = "        22.613714"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 90.0,"]
#[doc = "      \"minimum\": -90.0"]
#[doc = "    },"]
#[doc = "    \"Longitude\": {"]
#[doc = "      \"title\": \"Longitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        -17.250286,"]
#[doc = "        -17.250288,"]
#[doc = "        -44.13446"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 180.0,"]
#[doc = "      \"minimum\": -180.0"]
#[doc = "    },"]
#[doc = "    \"System\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Eocs Phyloea LD-K d8-0\","]
#[doc = "        \"Dryao Ain QD-K c8-0\","]
#[doc = "        \"Shrogaae KK-A d983\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Width\": {"]
#[doc = "      \"title\": \"Width\","]
#[doc = "      \"examples\": ["]
#[doc = "        2560,"]
#[doc = "        15360,"]
#[doc = "        3840"]
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
pub struct Screenshot {
    #[serde(
        rename = "Altitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub altitude: ::std::option::Option<f64>,
    #[serde(
        rename = "Body",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "Screenshot::event_value")]
    pub event: String,
    #[doc = "Filename of screenshot"]
    #[serde(rename = "Filename")]
    pub filename: ::std::string::String,
    #[serde(
        rename = "Heading",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub heading: ::std::option::Option<i64>,
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(
        rename = "Latitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latitude: ::std::option::Option<f64>,
    #[serde(
        rename = "Longitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub longitude: ::std::option::Option<f64>,
    #[serde(
        rename = "System",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "Width")]
    pub width: i64,
}
impl ::std::convert::From<&Screenshot> for Screenshot {
    fn from(value: &Screenshot) -> Self {
        value.clone()
    }
}

impl Screenshot {
    pub fn event_value() -> ::std::string::String {
        "Screenshot".to_string()
    }
}
