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
#[doc = "When written: when a new discovery is added to the Codex"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when a new discovery is added to the Codex\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Category\","]
#[doc = "    \"EntryID\","]
#[doc = "    \"Name\","]
#[doc = "    \"Region\","]
#[doc = "    \"SubCategory\","]
#[doc = "    \"System\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Category_StellarBodies;\","]
#[doc = "        \"$Codex_Category_Civilisations;\","]
#[doc = "        \"$Codex_Category_Biology;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Category_Localised\": {"]
#[doc = "      \"title\": \"Category_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Astronomical Bodies\","]
#[doc = "        \"Xenological\","]
#[doc = "        \"Biological and Geological\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"EntryID\": {"]
#[doc = "      \"title\": \"EntryID\","]
#[doc = "      \"examples\": ["]
#[doc = "        1100801,"]
#[doc = "        1102300,"]
#[doc = "        1100701"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"IsNewEntry\": {"]
#[doc = "      \"title\": \"IsNewEntry\","]
#[doc = "      \"description\": \"The IsNewEntry field is optional depending on the results of the scan\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Latitude\": {"]
#[doc = "      \"title\": \"Latitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        -20.319307,"]
#[doc = "        20.352921,"]
#[doc = "        -2.925347"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 90.0,"]
#[doc = "      \"minimum\": -90.0"]
#[doc = "    },"]
#[doc = "    \"Longitude\": {"]
#[doc = "      \"title\": \"Longitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        135.494843,"]
#[doc = "        -135.307236,"]
#[doc = "        60.841995"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 180.0,"]
#[doc = "      \"minimum\": -180.0"]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_L_Type_Name;\","]
#[doc = "        \"$Codex_Ent_Neutron_Stars_Name;\","]
#[doc = "        \"$Codex_Ent_M_Type_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"L Type Star\","]
#[doc = "        \"Neutron Star\","]
#[doc = "        \"M Type Star\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NearestDestination\": {"]
#[doc = "      \"title\": \"NearestDestination\","]
#[doc = "      \"description\": \"The NearestDestination is added if within 50km of a location listed in the navigation panel\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Settlement_Unflattened_WreckedUnknown:#index=5;\","]
#[doc = "        \"\","]
#[doc = "        \"$Fixed_Event_Life_Ring;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NearestDestination_Localised\": {"]
#[doc = "      \"title\": \"NearestDestination_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Crashed Thargoid Ship\","]
#[doc = "        \"Notable stellar phenomena\","]
#[doc = "        \"$POIScene_Trap_Cargo_02; \""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewTraitsDiscovered\": {"]
#[doc = "      \"title\": \"NewTraitsDiscovered\","]
#[doc = "      \"description\": \"The NewTraitsDiscovered field is optional depending on the results of the scan\","]
#[doc = "      \"examples\": ["]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Region\": {"]
#[doc = "      \"title\": \"Region\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_RegionName_35;\","]
#[doc = "        \"$Codex_RegionName_18;\","]
#[doc = "        \"$Codex_RegionName_34;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Region_Localised\": {"]
#[doc = "      \"title\": \"Region_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Outer Orion Spur\","]
#[doc = "        \"Inner Orion Spur\","]
#[doc = "        \"Sanguineous Rim\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SubCategory\": {"]
#[doc = "      \"title\": \"SubCategory\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_SubCategory_Stars;\","]
#[doc = "        \"$Codex_SubCategory_Terrestrials;\","]
#[doc = "        \"$Codex_SubCategory_Gas_Giants;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SubCategory_Localised\": {"]
#[doc = "      \"title\": \"SubCategory_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Stars\","]
#[doc = "        \"Terrestrial planets\","]
#[doc = "        \"Gas giant planets\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"System\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Blaa Eoq WN-Q a73-0\","]
#[doc = "        \"Blaa Eoq IZ-M d8-14\","]
#[doc = "        \"Blaa Eoq ET-Q b38-0\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        5790152675952,"]
#[doc = "        492352555595,"]
#[doc = "        730815146313"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Traits\": {"]
#[doc = "      \"title\": \"Traits\","]
#[doc = "      \"description\": \"The Traits field is only available for entries that have unlocked traits\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"VoucherAmount\": {"]
#[doc = "      \"title\": \"VoucherAmount\","]
#[doc = "      \"examples\": ["]
#[doc = "        2500,"]
#[doc = "        50000"]
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
pub struct CodexEntry {
    #[serde(
        rename = "BodyID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body_id: ::std::option::Option<i64>,
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[serde(
        rename = "Category_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub category_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "EntryID")]
    pub entry_id: i64,
    #[serde(skip_deserializing, default = "CodexEntry::event_value")]
    pub event: String,
    #[doc = "The IsNewEntry field is optional depending on the results of the scan"]
    #[serde(
        rename = "IsNewEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_new_entry: ::std::option::Option<bool>,
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
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[doc = "The NearestDestination is added if within 50km of a location listed in the navigation panel"]
    #[serde(
        rename = "NearestDestination",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nearest_destination: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "NearestDestination_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nearest_destination_localised: ::std::option::Option<::std::string::String>,
    #[doc = "The NewTraitsDiscovered field is optional depending on the results of the scan"]
    #[serde(
        rename = "NewTraitsDiscovered",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_traits_discovered: ::std::option::Option<bool>,
    #[serde(rename = "Region")]
    pub region: ::std::string::String,
    #[serde(
        rename = "Region_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub region_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SubCategory")]
    pub sub_category: ::std::string::String,
    #[serde(
        rename = "SubCategory_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_category_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "System")]
    pub system: ::std::string::String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The Traits field is only available for entries that have unlocked traits"]
    #[serde(
        rename = "Traits",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub traits: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "VoucherAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub voucher_amount: ::std::option::Option<i64>,
}
impl ::std::convert::From<&CodexEntry> for CodexEntry {
    fn from(value: &CodexEntry) -> Self {
        value.clone()
    }
}

impl CodexEntry {
    pub fn event_value() -> ::std::string::String {
        "CodexEntry".to_string()
    }
}
