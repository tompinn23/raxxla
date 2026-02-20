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
#[doc = "`ResourcesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Count\","]
#[doc = "    \"Name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"weaponschematic\","]
#[doc = "        \"ionisedgas\","]
#[doc = "        \"manufacturinginstructions\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Weapon Schematic\","]
#[doc = "        \"Ionised Gas\","]
#[doc = "        \"Manufacturing Instructions\""]
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
pub struct ResourcesItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ResourcesItem> for ResourcesItem {
    fn from(value: &ResourcesItem) -> Self {
        value.clone()
    }
}
#[doc = "`UpgradeWeapon`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Class\","]
#[doc = "    \"Cost\","]
#[doc = "    \"Name\","]
#[doc = "    \"SuitModuleID\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Class\": {"]
#[doc = "      \"title\": \"Class\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        4,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Cost\": {"]
#[doc = "      \"title\": \"Cost\","]
#[doc = "      \"examples\": ["]
#[doc = "        6250000,"]
#[doc = "        1500000,"]
#[doc = "        2500000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"wpn_m_assaultrifle_laser_fauto\","]
#[doc = "        \"wpn_s_pistol_kinetic_sauto\","]
#[doc = "        \"wpn_m_sniper_plasma_charged\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"TK Aphelion\","]
#[doc = "        \"Karma P-15\","]
#[doc = "        \"Manticore Executioner\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Resources\": {"]
#[doc = "      \"title\": \"Resources\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Count\","]
#[doc = "          \"Name\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              1,"]
#[doc = "              5"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"weaponschematic\","]
#[doc = "              \"ionisedgas\","]
#[doc = "              \"manufacturinginstructions\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Weapon Schematic\","]
#[doc = "              \"Ionised Gas\","]
#[doc = "              \"Manufacturing Instructions\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"SuitModuleID\": {"]
#[doc = "      \"title\": \"SuitModuleID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1700585855356669,"]
#[doc = "        1700575777091218,"]
#[doc = "        1700314469329676"]
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
pub struct UpgradeWeapon {
    #[serde(rename = "Class")]
    pub class: i64,
    #[serde(rename = "Cost")]
    pub cost: i64,
    #[serde(skip_deserializing, default = "UpgradeWeapon::event_value")]
    pub event: String,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Resources",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub resources: ::std::vec::Vec<ResourcesItem>,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&UpgradeWeapon> for UpgradeWeapon {
    fn from(value: &UpgradeWeapon) -> Self {
        value.clone()
    }
}

impl UpgradeWeapon {
    pub fn event_value() -> ::std::string::String {
        "UpgradeWeapon".to_string()
    }
}
