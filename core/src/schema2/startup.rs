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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StartUp {
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_deserializing, default = "StartUp::event_value")]
    pub event: String,
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "StarPos")]
    pub star_pos: Vec<f64>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[serde(rename = "SystemPopulation")]
    pub system_population: i64,
    #[serde(rename = "Body")]
    pub body: Option<String>,
    #[serde(rename = "BodyID")]
    pub body_id: Option<i64>,
    #[serde(rename = "BodyType")]
    pub body_type: Option<String>,
    pub docked: bool,
    #[serde(rename = "MarketId")]
    pub market_id: Option<i64>,
    #[serde(rename = "StationName")]
    pub station_name: Option<String>,
    #[serde(rename = "StationType")]
    pub station_type: Option<String>,
}
impl ::std::convert::From<&StartUp> for StartUp {
    fn from(value: &StartUp) -> Self {
        value.clone()
    }
}

impl StartUp {
    pub fn event_value() -> ::std::string::String {
        "StartUp".to_string()
    }
}
