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
#[doc = "This event is logged when the player uses the Organic Sampling Tool to scan, log or analyse organic discoveries. The first scan is Log, subsequent scans are Sample until fully scanned, final scan is Analyse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is logged when the player uses the Organic Sampling Tool to scan, log or analyse organic discoveries. The first scan is Log, subsequent scans are Sample until fully scanned, final scan is Analyse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Body\","]
#[doc = "    \"Genus\","]
#[doc = "    \"ScanType\","]
#[doc = "    \"Species\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        8,"]
#[doc = "        49"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Genus\": {"]
#[doc = "      \"title\": \"Genus\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Stratum_Genus_Name;\","]
#[doc = "        \"$Codex_Ent_Bacterial_Genus_Name;\","]
#[doc = "        \"$Codex_Ent_Fonticulus_Genus_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Genus_Localised\": {"]
#[doc = "      \"title\": \"Genus_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Stratum\","]
#[doc = "        \"Bacterium\","]
#[doc = "        \"Fonticulua\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ScanType\": {"]
#[doc = "      \"title\": \"ScanType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Log\","]
#[doc = "        \"Sample\","]
#[doc = "        \"Analyse\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Species\": {"]
#[doc = "      \"title\": \"Species\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Stratum_07_Name;\","]
#[doc = "        \"$Codex_Ent_Bacterial_12_Name;\","]
#[doc = "        \"$Codex_Ent_Fonticulus_02_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Species_Localised\": {"]
#[doc = "      \"title\": \"Species_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Stratum Tectonicas\","]
#[doc = "        \"Bacterium Cerbrus\","]
#[doc = "        \"Fonticulua Campestris\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        647903226970,"]
#[doc = "        40406380861921,"]
#[doc = "        4226214177978"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Variant\": {"]
#[doc = "      \"title\": \"Variant\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$Codex_Ent_Tubus_01_A_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Variant_Localised\": {"]
#[doc = "      \"title\": \"Variant_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Tubus Conifer - Indigo\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"WasLogged\": {"]
#[doc = "      \"title\": \"WasLogged\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
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
pub struct ScanOrganic {
    #[serde(rename = "Body")]
    pub body: i64,
    #[serde(skip_deserializing, default = "ScanOrganic::event_value")]
    pub event: String,
    #[serde(rename = "Genus")]
    pub genus: ::std::string::String,
    #[serde(
        rename = "Genus_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub genus_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ScanType")]
    pub scan_type: ::std::string::String,
    #[serde(rename = "Species")]
    pub species: ::std::string::String,
    #[serde(
        rename = "Species_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub species_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "Variant",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub variant: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Variant_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub variant_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "WasLogged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub was_logged: ::std::option::Option<bool>,
}
impl ::std::convert::From<&ScanOrganic> for ScanOrganic {
    fn from(value: &ScanOrganic) -> Self {
        value.clone()
    }
}

impl ScanOrganic {
    pub fn event_value() -> ::std::string::String {
        "ScanOrganic".to_string()
    }
}
