#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use crate::schema2::ApproachSettlement;

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
#[doc = "When written: when in Supercruise, and distance from planet drops to within the 'Orbital Cruise' zone"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when in Supercruise, and distance from planet drops to within the 'Orbital Cruise' zone\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Body\","]
#[doc = "    \"BodyID\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485 C 5\","]
#[doc = "        \"Siris 5 c\","]
#[doc = "        \"Vanth\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        35,"]
#[doc = "        40,"]
#[doc = "        38"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"HIP 20485\","]
#[doc = "        \"Siris\","]
#[doc = "        \"Sol\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        3657265287866,"]
#[doc = "        7269634614689,"]
#[doc = "        10477373803"]
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
pub struct ApproachBody {
    #[serde(rename = "Body")]
    pub body: ::std::string::String,
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[serde(skip_deserializing, default = "ApproachBody::event_value")]
    pub event: ::std::string::String,
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ApproachBody> for ApproachBody {
    fn from(value: &ApproachBody) -> Self {
        value.clone()
    }
}
impl ApproachBody {
    pub fn builder() -> builder::ApproachBody {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ApproachBody {
        body: ::std::result::Result<::std::string::String, ::std::string::String>,
        body_id: ::std::result::Result<i64, ::std::string::String>,
        event: ::std::result::Result<::std::string::String, ::std::string::String>,
        star_system: ::std::result::Result<::std::string::String, ::std::string::String>,
        system_address: ::std::result::Result<i64, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for ApproachBody {
        fn default() -> Self {
            Self {
                body: Err("no value supplied for body".to_string()),
                body_id: Err("no value supplied for body_id".to_string()),
                event: Err("no value supplied for event".to_string()),
                star_system: Err("no value supplied for star_system".to_string()),
                system_address: Err("no value supplied for system_address".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl ApproachBody {
        pub fn body<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for body: {}", e));
            self
        }
        pub fn body_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.body_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for body_id: {}", e));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {}", e));
            self
        }
        pub fn star_system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.star_system = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for star_system: {}", e));
            self
        }
        pub fn system_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.system_address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_address: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ApproachBody> for super::ApproachBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ApproachBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                body: value.body?,
                body_id: value.body_id?,
                event: value.event?,
                star_system: value.star_system?,
                system_address: value.system_address?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::ApproachBody> for ApproachBody {
        fn from(value: super::ApproachBody) -> Self {
            Self {
                body: Ok(value.body),
                body_id: Ok(value.body_id),
                event: Ok(value.event),
                star_system: Ok(value.star_system),
                system_address: Ok(value.system_address),
                timestamp: Ok(value.timestamp),
            }
        }
    }
}

impl ApproachBody {
    pub fn event_value() -> ::std::string::String {
        "ApproachBody".to_string()
    }
}
