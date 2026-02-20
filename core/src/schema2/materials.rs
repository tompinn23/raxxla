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
#[doc = "`EncodedItem`"]
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
#[doc = "        86,"]
#[doc = "        83,"]
#[doc = "        125"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"shieldsoakanalysis\","]
#[doc = "        \"shieldpatternanalysis\","]
#[doc = "        \"shielddensityreports\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Inconsistent Shield Soak Analysis\","]
#[doc = "        \"Aberrant Shield Pattern Analysis\","]
#[doc = "        \"Untypical Shield Scans\""]
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
pub struct EncodedItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[doc = "The localised value will be omitted if it is exactly the same as Name"]
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&EncodedItem> for EncodedItem {
    fn from(value: &EncodedItem) -> Self {
        value.clone()
    }
}
#[doc = "`ManufacturedItem`"]
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
#[doc = "        60,"]
#[doc = "        18,"]
#[doc = "        107"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"hybridcapacitors\","]
#[doc = "        \"protoradiolicalloys\","]
#[doc = "        \"protolightalloys\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Hybrid Capacitors\","]
#[doc = "        \"Proto Radiolic Alloys\","]
#[doc = "        \"Proto Light Alloys\""]
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
pub struct ManufacturedItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[doc = "The localised value will be omitted if it is exactly the same as Name"]
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ManufacturedItem> for ManufacturedItem {
    fn from(value: &ManufacturedItem) -> Self {
        value.clone()
    }
}
#[doc = "When written: at startup, when loading from main menu into game"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: at startup, when loading from main menu into game\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Encoded\","]
#[doc = "    \"Manufactured\","]
#[doc = "    \"Raw\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Encoded\": {"]
#[doc = "      \"title\": \"Encoded\","]
#[doc = "      \"default\": null,"]
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
#[doc = "              86,"]
#[doc = "              83,"]
#[doc = "              125"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"shieldsoakanalysis\","]
#[doc = "              \"shieldpatternanalysis\","]
#[doc = "              \"shielddensityreports\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Inconsistent Shield Soak Analysis\","]
#[doc = "              \"Aberrant Shield Pattern Analysis\","]
#[doc = "              \"Untypical Shield Scans\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Manufactured\": {"]
#[doc = "      \"title\": \"Manufactured\","]
#[doc = "      \"default\": null,"]
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
#[doc = "              60,"]
#[doc = "              18,"]
#[doc = "              107"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"hybridcapacitors\","]
#[doc = "              \"protoradiolicalloys\","]
#[doc = "              \"protolightalloys\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Hybrid Capacitors\","]
#[doc = "              \"Proto Radiolic Alloys\","]
#[doc = "              \"Proto Light Alloys\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Raw\": {"]
#[doc = "      \"title\": \"Raw\","]
#[doc = "      \"default\": null,"]
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
#[doc = "              248,"]
#[doc = "              250,"]
#[doc = "              266"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"iron\","]
#[doc = "              \"nickel\","]
#[doc = "              \"carbon\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Хром\","]
#[doc = "              \"Углерод\","]
#[doc = "              \"Железо\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
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
pub struct Materials {
    #[serde(rename = "Encoded")]
    pub encoded: ::std::vec::Vec<EncodedItem>,
    #[serde(skip_deserializing, default = "Materials::event_value")]
    pub event: String,
    #[serde(rename = "Manufactured")]
    pub manufactured: ::std::vec::Vec<ManufacturedItem>,
    #[serde(rename = "Raw")]
    pub raw: ::std::vec::Vec<RawItem>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Materials> for Materials {
    fn from(value: &Materials) -> Self {
        value.clone()
    }
}
#[doc = "`RawItem`"]
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
#[doc = "        248,"]
#[doc = "        250,"]
#[doc = "        266"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"iron\","]
#[doc = "        \"nickel\","]
#[doc = "        \"carbon\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Хром\","]
#[doc = "        \"Углерод\","]
#[doc = "        \"Железо\""]
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
pub struct RawItem {
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[doc = "The localised value will be omitted if it is exactly the same as Name"]
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&RawItem> for RawItem {
    fn from(value: &RawItem) -> Self {
        value.clone()
    }
}

impl Materials {
    pub fn event_value() -> ::std::string::String {
        "Materials".to_string()
    }
}
