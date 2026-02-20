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
#[doc = "When written: when a mission is updated with a new destination"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When written: when a mission is updated with a new destination\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"MissionID\","]
#[doc = "    \"Name\","]
#[doc = "    \"NewDestinationStation\","]
#[doc = "    \"NewDestinationSystem\","]
#[doc = "    \"OldDestinationStation\","]
#[doc = "    \"OldDestinationSystem\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"LocalisedName\": {"]
#[doc = "      \"title\": \"LocalisedName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Turn on power at Walter Drilling Platform\","]
#[doc = "        \"$Mission_OnFoot_SalvageIllegal_MB_name:#startStationName=Vespucci Stop:#startSystemName=Apuris:#startStationSystemName=Apuris:#DestinationSystemName=Apuris:#CommodityName=$MemoryChip_Name;:#CommodityQuantity=1:#missionGiverFactionContact=Owen Bates:#missionGiverFactionContactTitle=$GovernmentRankContactConfederacy_3_Male;:#commanderName=Banana;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"LocalisedName_Localised\": {"]
#[doc = "      \"title\": \"LocalisedName_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Turn on power at Walter Drilling Platform\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        868354915,"]
#[doc = "        868354970,"]
#[doc = "        869372871"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Mission_OnFoot_Hack_Download_Covert_MB\","]
#[doc = "        \"Mission_OnFoot_Hack_Download_MB\","]
#[doc = "        \"Mission_OnFoot_Hack_Download_Offline_MB\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewDestinationStation\": {"]
#[doc = "      \"title\": \"NewDestinationStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Bowersox Terminal\","]
#[doc = "        \"Hardy Orbital\","]
#[doc = "        \"Jameson Memorial\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewDestinationSystem\": {"]
#[doc = "      \"title\": \"NewDestinationSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Latorioson\","]
#[doc = "        \"Asooraja\","]
#[doc = "        \"Shinrarta Dezhra\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OldDestinationStation\": {"]
#[doc = "      \"title\": \"OldDestinationStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Watanabe Botanical Nursery\","]
#[doc = "        \"Pomeroy Analytics Installation\","]
#[doc = "        \"Okusanya Manufacturing Hub\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OldDestinationSystem\": {"]
#[doc = "      \"title\": \"OldDestinationSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Latorioson\","]
#[doc = "        \"Asooraja\","]
#[doc = "        \"Fenrichua\""]
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
pub struct MissionRedirected {
    #[serde(skip_deserializing, default = "MissionRedirected::event_value")]
    pub event: String,
    #[serde(
        rename = "LocalisedName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub localised_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "LocalisedName_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub localised_name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "MissionID")]
    pub mission_id: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(rename = "NewDestinationStation")]
    pub new_destination_station: ::std::string::String,
    #[serde(rename = "NewDestinationSystem")]
    pub new_destination_system: ::std::string::String,
    #[serde(rename = "OldDestinationStation")]
    pub old_destination_station: ::std::string::String,
    #[serde(rename = "OldDestinationSystem")]
    pub old_destination_system: ::std::string::String,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&MissionRedirected> for MissionRedirected {
    fn from(value: &MissionRedirected) -> Self {
        value.clone()
    }
}

impl MissionRedirected {
    pub fn event_value() -> ::std::string::String {
        "MissionRedirected".to_string()
    }
}
