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
#[doc = "This is written when there is any change to the contents of the suit backpack – note this can be written at the same time as other events like UseConsumable"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This is written when there is any change to the contents of the suit backpack – note this can be written at the same time as other events like UseConsumable\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Added\": {"]
#[doc = "      \"title\": \"Added\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/definitions/changeEntry\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Removed\": {"]
#[doc = "      \"title\": \"Removed\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/definitions/changeEntry\""]
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
pub struct BackpackChange {
    #[serde(
        rename = "Added",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub added: ::std::vec::Vec<ChangeEntry>,
    #[serde(skip_deserializing, default = "BackpackChange::event_value")]
    pub event: String,
    #[serde(
        rename = "Removed",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub removed: ::std::vec::Vec<ChangeEntry>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&BackpackChange> for BackpackChange {
    fn from(value: &BackpackChange) -> Self {
        value.clone()
    }
}
#[doc = "`ChangeEntry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"Name\","]
#[doc = "    \"OwnerID\","]
#[doc = "    \"Type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        8,"]
#[doc = "        4"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        839014036,"]
#[doc = "        895446120,"]
#[doc = "        895536604"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"energycell\","]
#[doc = "        \"amm_grenade_emp\","]
#[doc = "        \"amm_grenade_frag\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Energy Cell\","]
#[doc = "        \"Shield Disruptor\","]
#[doc = "        \"Frag Grenade\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OwnerID\": {"]
#[doc = "      \"title\": \"OwnerID\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        2954264951,"]
#[doc = "        2952913247"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Consumable\","]
#[doc = "        \"Component\","]
#[doc = "        \"Item\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChangeEntry {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(
        rename = "MissionID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mission_id: ::std::option::Option<i64>,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "OwnerID")]
    pub owner_id: i64,
    #[serde(rename = "Type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ChangeEntry> for ChangeEntry {
    fn from(value: &ChangeEntry) -> Self {
        value.clone()
    }
}

impl BackpackChange {
    pub fn event_value() -> ::std::string::String {
        "BackpackChange".to_string()
    }
}
