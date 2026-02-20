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
#[doc = "`MaterialsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"Proportion\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Gallite\","]
#[doc = "        \"Praseodymium\","]
#[doc = "        \"Samarium\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Methane Clathrate\","]
#[doc = "        \"Hydrogen Peroxide\","]
#[doc = "        \"Liquid oxygen\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Proportion\": {"]
#[doc = "      \"title\": \"Proportion\","]
#[doc = "      \"examples\": ["]
#[doc = "        21.276043,"]
#[doc = "        15.267719,"]
#[doc = "        9.73971"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MaterialsItem {
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Proportion")]
    pub proportion: f64,
}
impl ::std::convert::From<&MaterialsItem> for MaterialsItem {
    fn from(value: &MaterialsItem) -> Self {
        value.clone()
    }
}
#[doc = "When using a prospecting drone"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When using a prospecting drone\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Content\","]
#[doc = "    \"Materials\","]
#[doc = "    \"Remaining\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Content\": {"]
#[doc = "      \"title\": \"Content\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$AsteroidMaterialContent_Low;\","]
#[doc = "        \"$AsteroidMaterialContent_Medium;\","]
#[doc = "        \"$AsteroidMaterialContent_High;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Content_Localised\": {"]
#[doc = "      \"title\": \"Content_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Material Content: Low\","]
#[doc = "        \"Material Content: Medium\","]
#[doc = "        \"Material Content: High\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Materials\": {"]
#[doc = "      \"title\": \"Materials\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Name\","]
#[doc = "          \"Proportion\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Gallite\","]
#[doc = "              \"Praseodymium\","]
#[doc = "              \"Samarium\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Methane Clathrate\","]
#[doc = "              \"Hydrogen Peroxide\","]
#[doc = "              \"Liquid oxygen\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Proportion\": {"]
#[doc = "            \"title\": \"Proportion\","]
#[doc = "            \"examples\": ["]
#[doc = "              21.276043,"]
#[doc = "              15.267719,"]
#[doc = "              9.73971"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"MotherlodeMaterial\": {"]
#[doc = "      \"title\": \"MotherlodeMaterial\","]
#[doc = "      \"description\": \"If it’s a motherlode\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Bromellite\","]
#[doc = "        \"LowTemperatureDiamond\","]
#[doc = "        \"Alexandrite\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"MotherlodeMaterial_Localised\": {"]
#[doc = "      \"title\": \"MotherlodeMaterial_Localised\","]
#[doc = "      \"description\": \"If it’s a motherlode. The localised value will be omitted if it is exactly the same as MotherlodeMaterial\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Low Temperature Diamonds\","]
#[doc = "        \"Void Opal\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Remaining\": {"]
#[doc = "      \"title\": \"Remaining\","]
#[doc = "      \"description\": \"Percentage of materials remaining\","]
#[doc = "      \"examples\": ["]
#[doc = "        100.0,"]
#[doc = "        0.0,"]
#[doc = "        41.935486"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
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
pub struct ProspectedAsteroid {
    #[serde(rename = "Content")]
    pub content: ::std::string::String,
    #[serde(
        rename = "Content_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub content_localised: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "ProspectedAsteroid::event_value")]
    pub event: String,
    #[serde(rename = "Materials")]
    pub materials: ::std::vec::Vec<MaterialsItem>,
    #[doc = "If it’s a motherlode"]
    #[serde(
        rename = "MotherlodeMaterial",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub motherlode_material: ::std::option::Option<::std::string::String>,
    #[doc = "If it’s a motherlode. The localised value will be omitted if it is exactly the same as MotherlodeMaterial"]
    #[serde(
        rename = "MotherlodeMaterial_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub motherlode_material_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Remaining")]
    pub remaining: f64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&ProspectedAsteroid> for ProspectedAsteroid {
    fn from(value: &ProspectedAsteroid) -> Self {
        value.clone()
    }
}

impl ProspectedAsteroid {
    pub fn event_value() -> ::std::string::String {
        "ProspectedAsteroid".to_string()
    }
}
