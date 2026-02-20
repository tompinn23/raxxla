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
#[doc = "`Destination`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Destination\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Body\","]
#[doc = "    \"Name\","]
#[doc = "    \"System\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Body\": {"]
#[doc = "      \"title\": \"Body\","]
#[doc = "      \"examples\": ["]
#[doc = "        5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Harow\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Harow\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"System\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"examples\": ["]
#[doc = "        7269098333561"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Destination {
    #[serde(rename = "Body")]
    pub body: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "Name_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "System")]
    pub system: i64,
}
impl ::std::convert::From<&Destination> for Destination {
    fn from(value: &Destination) -> Self {
        value.clone()
    }
}
#[doc = "`Fuel`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Fuel\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"FuelMain\","]
#[doc = "    \"FuelReservoir\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"FuelMain\": {"]
#[doc = "      \"title\": \"FuelMain\","]
#[doc = "      \"examples\": ["]
#[doc = "        15.146626,"]
#[doc = "        8.0,"]
#[doc = "        0.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"FuelReservoir\": {"]
#[doc = "      \"title\": \"FuelReservoir\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.382796,"]
#[doc = "        0.41,"]
#[doc = "        0.404189"]
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
pub struct Fuel {
    #[serde(rename = "FuelMain")]
    pub fuel_main: f64,
    #[serde(rename = "FuelReservoir")]
    pub fuel_reservoir: f64,
}
impl ::std::convert::From<&Fuel> for Fuel {
    fn from(value: &Fuel) -> Self {
        value.clone()
    }
}
#[doc = "This event is written to Status.json which is updated every few seconds"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This event is written to Status.json which is updated every few seconds\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Flags\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Altitude\": {"]
#[doc = "      \"title\": \"Altitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        404,"]
#[doc = "        0,"]
#[doc = "        88"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Balance\": {"]
#[doc = "      \"title\": \"Balance\","]
#[doc = "      \"examples\": ["]
#[doc = "        1593446845,"]
#[doc = "        1593431575"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyName\": {"]
#[doc = "      \"title\": \"BodyName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Crown Depot\","]
#[doc = "        \"Harow A 1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Cargo\": {"]
#[doc = "      \"title\": \"Cargo\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Destination\": {"]
#[doc = "      \"title\": \"Destination\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Body\","]
#[doc = "        \"Name\","]
#[doc = "        \"System\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Body\": {"]
#[doc = "          \"title\": \"Body\","]
#[doc = "          \"examples\": ["]
#[doc = "            5"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"Name\": {"]
#[doc = "          \"title\": \"Name\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Harow\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"Name_Localised\": {"]
#[doc = "          \"title\": \"Name_Localised\","]
#[doc = "          \"examples\": ["]
#[doc = "            \"Harow\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"System\": {"]
#[doc = "          \"title\": \"System\","]
#[doc = "          \"examples\": ["]
#[doc = "            7269098333561"]
#[doc = "          ],"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"FireGroup\": {"]
#[doc = "      \"title\": \"FireGroup\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"examples\": ["]
#[doc = "        16842765,"]
#[doc = "        18874376,"]
#[doc = "        0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Flags2\": {"]
#[doc = "      \"title\": \"Flags2\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        90121,"]
#[doc = "        35345"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Fuel\": {"]
#[doc = "      \"title\": \"Fuel\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"FuelMain\","]
#[doc = "        \"FuelReservoir\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"FuelMain\": {"]
#[doc = "          \"title\": \"FuelMain\","]
#[doc = "          \"examples\": ["]
#[doc = "            15.146626,"]
#[doc = "            8.0,"]
#[doc = "            0.0"]
#[doc = "          ],"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"FuelReservoir\": {"]
#[doc = "          \"title\": \"FuelReservoir\","]
#[doc = "          \"examples\": ["]
#[doc = "            0.382796,"]
#[doc = "            0.41,"]
#[doc = "            0.404189"]
#[doc = "          ],"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"Gravity\": {"]
#[doc = "      \"title\": \"Gravity\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.347632"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"GuiFocus\": {"]
#[doc = "      \"title\": \"GuiFocus\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        0,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Heading\": {"]
#[doc = "      \"title\": \"Heading\","]
#[doc = "      \"examples\": ["]
#[doc = "        109,"]
#[doc = "        34,"]
#[doc = "        37"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Health\": {"]
#[doc = "      \"title\": \"Health\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Latitude\": {"]
#[doc = "      \"title\": \"Latitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        -28.584963,"]
#[doc = "        -66.410538,"]
#[doc = "        -66.413887"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"LegalState\": {"]
#[doc = "      \"title\": \"LegalState\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Clean\","]
#[doc = "        \"Speeding\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Longitude\": {"]
#[doc = "      \"title\": \"Longitude\","]
#[doc = "      \"examples\": ["]
#[doc = "        6.826313,"]
#[doc = "        19.822344,"]
#[doc = "        19.815832"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Oxygen\": {"]
#[doc = "      \"title\": \"Oxygen\","]
#[doc = "      \"examples\": ["]
#[doc = "        1.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Pips\": {"]
#[doc = "      \"title\": \"Pips\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          2,"]
#[doc = "          8,"]
#[doc = "          4"]
#[doc = "        ],"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"PlanetRadius\": {"]
#[doc = "      \"title\": \"PlanetRadius\","]
#[doc = "      \"examples\": ["]
#[doc = "        2321861.5"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"SelectedWeapon\": {"]
#[doc = "      \"title\": \"SelectedWeapon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"\","]
#[doc = "        \"$humanoid_fists_name;\","]
#[doc = "        \"$wpn_m_sniper_plasma_charged_name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SelectedWeapon_Localised\": {"]
#[doc = "      \"title\": \"SelectedWeapon_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Unarmed\","]
#[doc = "        \"Manticore Executioner\","]
#[doc = "        \"Manticore Tormentor\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Temperature\": {"]
#[doc = "      \"title\": \"Temperature\","]
#[doc = "      \"examples\": ["]
#[doc = "        293.0,"]
#[doc = "        713.976318,"]
#[doc = "        713.944336"]
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
pub struct Status {
    #[serde(
        rename = "Altitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub altitude: ::std::option::Option<i64>,
    #[serde(
        rename = "Balance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub balance: ::std::option::Option<i64>,
    #[serde(
        rename = "BodyName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Cargo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cargo: ::std::option::Option<f64>,
    #[serde(
        rename = "Destination",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination: ::std::option::Option<Destination>,
    #[serde(skip_deserializing, default = "Status::event_value")]
    pub event: String,
    #[serde(
        rename = "FireGroup",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fire_group: ::std::option::Option<i64>,
    #[serde(rename = "Flags")]
    pub flags: i64,
    #[serde(
        rename = "Flags2",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub flags2: ::std::option::Option<i64>,
    #[serde(
        rename = "Fuel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fuel: ::std::option::Option<Fuel>,
    #[serde(
        rename = "Gravity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gravity: ::std::option::Option<f64>,
    #[serde(
        rename = "GuiFocus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gui_focus: ::std::option::Option<i64>,
    #[serde(
        rename = "Heading",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub heading: ::std::option::Option<i64>,
    #[serde(
        rename = "Health",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub health: ::std::option::Option<f64>,
    #[serde(
        rename = "Latitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latitude: ::std::option::Option<f64>,
    #[serde(
        rename = "LegalState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_state: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Longitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub longitude: ::std::option::Option<f64>,
    #[serde(
        rename = "Oxygen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub oxygen: ::std::option::Option<f64>,
    #[serde(
        rename = "Pips",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub pips: ::std::vec::Vec<i64>,
    #[serde(
        rename = "PlanetRadius",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planet_radius: ::std::option::Option<f64>,
    #[serde(
        rename = "SelectedWeapon",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub selected_weapon: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "SelectedWeapon_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub selected_weapon_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Temperature",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temperature: ::std::option::Option<f64>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}

impl Status {
    pub fn event_value() -> ::std::string::String {
        "Status".to_string()
    }
}
