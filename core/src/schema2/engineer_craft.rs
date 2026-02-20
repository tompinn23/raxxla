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
#[doc = "When Written: when requesting an engineer upgrade"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when requesting an engineer upgrade\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BlueprintID\","]
#[doc = "    \"BlueprintName\","]
#[doc = "    \"EngineerID\","]
#[doc = "    \"Ingredients\","]
#[doc = "    \"Level\","]
#[doc = "    \"Modifiers\","]
#[doc = "    \"Module\","]
#[doc = "    \"Quality\","]
#[doc = "    \"Slot\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ApplyExperimentalEffect\": {"]
#[doc = "      \"title\": \"ApplyExperimentalEffect\","]
#[doc = "      \"description\": \"when applying a new effect\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"special_engine_overloaded\","]
#[doc = "        \"special_fsd_heavy\","]
#[doc = "        \"special_powerdistributor_fast\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"BlueprintID\": {"]
#[doc = "      \"title\": \"BlueprintID\","]
#[doc = "      \"examples\": ["]
#[doc = "        128731491,"]
#[doc = "        128731492,"]
#[doc = "        128731493"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BlueprintName\": {"]
#[doc = "      \"title\": \"BlueprintName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Misc_LightWeight\","]
#[doc = "        \"Sensor_LightWeight\","]
#[doc = "        \"Engine_Dirty\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Engineer\": {"]
#[doc = "      \"title\": \"Engineer\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Etienne Dorn\","]
#[doc = "        \"Juri Ishmaak\","]
#[doc = "        \"Chloe Sedesi\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"EngineerID\": {"]
#[doc = "      \"title\": \"EngineerID\","]
#[doc = "      \"examples\": ["]
#[doc = "        300290,"]
#[doc = "        300250,"]
#[doc = "        300300"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ExperimentalEffect\": {"]
#[doc = "      \"title\": \"ExperimentalEffect\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"special_engine_overloaded\","]
#[doc = "        \"special_fsd_heavy\","]
#[doc = "        \"special_powerdistributor_fast\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ExperimentalEffect_Localised\": {"]
#[doc = "      \"title\": \"ExperimentalEffect_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Drag Drives\","]
#[doc = "        \"Mass Manager\","]
#[doc = "        \"Super Conduits\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Ingredients\": {"]
#[doc = "      \"title\": \"Ingredients\","]
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
#[doc = "              1,"]
#[doc = "              5,"]
#[doc = "              3"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"phosphorus\","]
#[doc = "              \"salvagedalloys\","]
#[doc = "              \"manganese\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Salvaged Alloys\","]
#[doc = "              \"Conductive Ceramics\","]
#[doc = "              \"Conductive Components\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Level\": {"]
#[doc = "      \"title\": \"Level\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        2,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Modifiers\": {"]
#[doc = "      \"title\": \"Modifiers\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Label\","]
#[doc = "          \"LessIsGood\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Label\": {"]
#[doc = "            \"title\": \"Label\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Mass\","]
#[doc = "              \"Integrity\","]
#[doc = "              \"SensorTargetScanAngle\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"LessIsGood\": {"]
#[doc = "            \"title\": \"LessIsGood\","]
#[doc = "            \"description\": \"Either 0 or 1\","]
#[doc = "            \"examples\": ["]
#[doc = "              1,"]
#[doc = "              0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"OriginalValue\": {"]
#[doc = "            \"title\": \"OriginalValue\","]
#[doc = "            \"examples\": ["]
#[doc = "              4.0,"]
#[doc = "              72.0,"]
#[doc = "              8.0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"Value\": {"]
#[doc = "            \"title\": \"Value\","]
#[doc = "            \"description\": \"Either Value or ValueStr is used. These modification types have string values: WeaponMode, DamageType, CabinClass.\","]
#[doc = "            \"examples\": ["]
#[doc = "              2.2132,"]
#[doc = "              64.799995,"]
#[doc = "              1.896"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"ValueStr\": {"]
#[doc = "            \"title\": \"ValueStr\","]
#[doc = "            \"description\": \"Either Value or ValueStr is used. These modification types have string values: WeaponMode, DamageType, CabinClass.\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$Thermic;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"ValueStr_Localised\": {"]
#[doc = "            \"title\": \"ValueStr_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Thermal\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Module\": {"]
#[doc = "      \"title\": \"Module\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"int_lifesupport_size4_class2\","]
#[doc = "        \"int_sensors_size5_class2\","]
#[doc = "        \"int_engine_size6_class5\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"title\": \"Quality\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.967,"]
#[doc = "        0.76,"]
#[doc = "        1.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"Slot\": {"]
#[doc = "      \"title\": \"Slot\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"LifeSupport\","]
#[doc = "        \"Radar\","]
#[doc = "        \"MainEngines\""]
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
pub struct EngineerCraft {
    #[doc = "when applying a new effect"]
    #[serde(
        rename = "ApplyExperimentalEffect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub apply_experimental_effect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: i64,
    #[serde(rename = "BlueprintName")]
    pub blueprint_name: ::std::string::String,
    #[serde(
        rename = "Engineer",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub engineer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,
    #[serde(skip_deserializing, default = "EngineerCraft::event_value")]
    pub event: String,
    #[serde(
        rename = "ExperimentalEffect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub experimental_effect: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ExperimentalEffect_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub experimental_effect_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Ingredients")]
    pub ingredients: ::std::vec::Vec<IngredientsItem>,
    #[serde(rename = "Level")]
    pub level: i64,
    #[serde(rename = "Modifiers")]
    pub modifiers: ::std::vec::Vec<ModifiersItem>,
    #[serde(rename = "Module")]
    pub module: ::std::string::String,
    #[serde(rename = "Quality")]
    pub quality: f64,
    #[serde(rename = "Slot")]
    pub slot: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&EngineerCraft> for EngineerCraft {
    fn from(value: &EngineerCraft) -> Self {
        value.clone()
    }
}
#[doc = "`IngredientsItem`"]
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
#[doc = "        5,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"phosphorus\","]
#[doc = "        \"salvagedalloys\","]
#[doc = "        \"manganese\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Salvaged Alloys\","]
#[doc = "        \"Conductive Ceramics\","]
#[doc = "        \"Conductive Components\""]
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
pub struct IngredientsItem {
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
impl ::std::convert::From<&IngredientsItem> for IngredientsItem {
    fn from(value: &IngredientsItem) -> Self {
        value.clone()
    }
}
#[doc = "`ModifiersItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Label\","]
#[doc = "    \"LessIsGood\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Label\": {"]
#[doc = "      \"title\": \"Label\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Mass\","]
#[doc = "        \"Integrity\","]
#[doc = "        \"SensorTargetScanAngle\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"LessIsGood\": {"]
#[doc = "      \"title\": \"LessIsGood\","]
#[doc = "      \"description\": \"Either 0 or 1\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"OriginalValue\": {"]
#[doc = "      \"title\": \"OriginalValue\","]
#[doc = "      \"examples\": ["]
#[doc = "        4.0,"]
#[doc = "        72.0,"]
#[doc = "        8.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"description\": \"Either Value or ValueStr is used. These modification types have string values: WeaponMode, DamageType, CabinClass.\","]
#[doc = "      \"examples\": ["]
#[doc = "        2.2132,"]
#[doc = "        64.799995,"]
#[doc = "        1.896"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ValueStr\": {"]
#[doc = "      \"title\": \"ValueStr\","]
#[doc = "      \"description\": \"Either Value or ValueStr is used. These modification types have string values: WeaponMode, DamageType, CabinClass.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Thermic;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ValueStr_Localised\": {"]
#[doc = "      \"title\": \"ValueStr_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Thermal\""]
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
pub struct ModifiersItem {
    #[serde(rename = "Label")]
    pub label: ::std::string::String,
    #[doc = "Either 0 or 1"]
    #[serde(rename = "LessIsGood")]
    pub less_is_good: i64,
    #[serde(
        rename = "OriginalValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_value: ::std::option::Option<f64>,
    #[serde(
        rename = "Value",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value: ::std::option::Option<f64>,
    #[doc = "Either Value or ValueStr is used. These modification types have string values: WeaponMode, DamageType, CabinClass."]
    #[serde(
        rename = "ValueStr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_str: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ValueStr_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_str_localised: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ModifiersItem> for ModifiersItem {
    fn from(value: &ModifiersItem) -> Self {
        value.clone()
    }
}

impl EngineerCraft {
    pub fn event_value() -> ::std::string::String {
        "EngineerCraft".to_string()
    }
}
