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
#[doc = "This event is logged when a player adds a weapon to a suit loadout"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when a player adds a weapon to a suit loadout\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Class\","]
#[doc = "    \"LoadoutID\","]
#[doc = "    \"LoadoutName\","]
#[doc = "    \"ModuleName\","]
#[doc = "    \"SlotName\","]
#[doc = "    \"SuitID\","]
#[doc = "    \"SuitModuleID\","]
#[doc = "    \"SuitName\","]
#[doc = "    \"WeaponMods\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Class\": {"]
#[doc = "      \"title\": \"Class\","]
#[doc = "      \"examples\": ["]
#[doc = "        4,"]
#[doc = "        5,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LoadoutID\": {"]
#[doc = "      \"title\": \"LoadoutID\","]
#[doc = "      \"examples\": ["]
#[doc = "        4293000004,"]
#[doc = "        4293000005,"]
#[doc = "        4293000006"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LoadoutName\": {"]
#[doc = "      \"title\": \"LoadoutName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"maverick\","]
#[doc = "        \"dominator\","]
#[doc = "        \"artemis\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ModuleName\": {"]
#[doc = "      \"title\": \"ModuleName\","]
#[doc = "      \"description\": \"New weapon or other item added to loadout\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"wpn_m_sniper_plasma_charged\","]
#[doc = "        \"wpn_s_pistol_plasma_charged\","]
#[doc = "        \"wpn_m_shotgun_plasma_doublebarrel\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ModuleName_Localised\": {"]
#[doc = "      \"title\": \"ModuleName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Manticore Executioner\","]
#[doc = "        \"Manticore Tormentor\","]
#[doc = "        \"Manticore Intimidator\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SlotName\": {"]
#[doc = "      \"title\": \"SlotName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"PrimaryWeapon1\","]
#[doc = "        \"SecondaryWeapon\","]
#[doc = "        \"PrimaryWeapon2\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SuitID\": {"]
#[doc = "      \"title\": \"SuitID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1700585833788820,"]
#[doc = "        1700575810179595,"]
#[doc = "        1700567461457237"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitModuleID\": {"]
#[doc = "      \"title\": \"SuitModuleID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1700314469329676,"]
#[doc = "        1700314451340369,"]
#[doc = "        1700314462068060"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SuitName\": {"]
#[doc = "      \"title\": \"SuitName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"utilitysuit_class5\","]
#[doc = "        \"tacticalsuit_class5\","]
#[doc = "        \"explorationsuit_class2\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SuitName_Localised\": {"]
#[doc = "      \"title\": \"SuitName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$UtilitySuit_Class1_Name;\","]
#[doc = "        \"$TacticalSuit_Class1_Name;\","]
#[doc = "        \"$ExplorationSuit_Class1_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"WeaponMods\": {"]
#[doc = "      \"title\": \"WeaponMods\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"weapon_stability\","]
#[doc = "          \"weapon_handling\","]
#[doc = "          \"weapon_suppression_unpressurised\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
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
pub struct LoadoutEquipModule {
    #[serde(rename = "Class")]
    pub class: i64,
    #[serde(skip_deserializing, default = "LoadoutEquipModule::event_value")]
    pub event: String,
    #[serde(rename = "LoadoutID")]
    pub loadout_id: i64,
    #[serde(rename = "LoadoutName")]
    pub loadout_name: ::std::string::String,
    #[doc = "New weapon or other item added to loadout"]
    #[serde(rename = "ModuleName")]
    pub module_name: ::std::string::String,
    #[serde(
        rename = "ModuleName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub module_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SlotName")]
    pub slot_name: ::std::string::String,
    #[serde(rename = "SuitID")]
    pub suit_id: i64,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: i64,
    #[serde(rename = "SuitName")]
    pub suit_name: ::std::string::String,
    #[serde(
        rename = "SuitName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub suit_name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "WeaponMods")]
    pub weapon_mods: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&LoadoutEquipModule> for LoadoutEquipModule {
    fn from(value: &LoadoutEquipModule) -> Self {
        value.clone()
    }
}

impl LoadoutEquipModule {
    pub fn event_value() -> ::std::string::String {
        "LoadoutEquipModule".to_string()
    }
}
