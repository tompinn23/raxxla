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
#[doc = "`AtmosphereCompositionItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Name\","]
#[doc = "    \"Percent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Water\","]
#[doc = "        \"Nitrogen\","]
#[doc = "        \"Ammonia\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Percent\": {"]
#[doc = "      \"title\": \"Percent\","]
#[doc = "      \"examples\": ["]
#[doc = "        99.124542,"]
#[doc = "        0.492685,"]
#[doc = "        0.302259"]
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
pub struct AtmosphereCompositionItem {
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(rename = "Percent")]
    pub percent: f64,
}
impl ::std::convert::From<&AtmosphereCompositionItem> for AtmosphereCompositionItem {
    fn from(value: &AtmosphereCompositionItem) -> Self {
        value.clone()
    }
}
#[doc = "Written for Planet/Moon"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Composition\","]
#[doc = "  \"description\": \"Written for Planet/Moon\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Ice\","]
#[doc = "    \"Metal\","]
#[doc = "    \"Rock\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Ice\": {"]
#[doc = "      \"title\": \"Ice\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.008881,"]
#[doc = "        0.0,"]
#[doc = "        0.825387"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Metal\": {"]
#[doc = "      \"title\": \"Metal\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.329764,"]
#[doc = "        0.327762,"]
#[doc = "        0.015514"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Rock\": {"]
#[doc = "      \"title\": \"Rock\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.661355,"]
#[doc = "        0.672238,"]
#[doc = "        0.159098"]
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
pub struct Composition {
    #[serde(rename = "Ice")]
    pub ice: f64,
    #[serde(rename = "Metal")]
    pub metal: f64,
    #[serde(rename = "Rock")]
    pub rock: f64,
}
impl ::std::convert::From<&Composition> for Composition {
    fn from(value: &Composition) -> Self {
        value.clone()
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
#[doc = "    \"Percent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"iron\","]
#[doc = "        \"nickel\","]
#[doc = "        \"sulphur\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"iron\","]
#[doc = "        \"nickel\","]
#[doc = "        \"sulphur\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Percent\": {"]
#[doc = "      \"title\": \"Percent\","]
#[doc = "      \"examples\": ["]
#[doc = "        21.92185,"]
#[doc = "        16.580769,"]
#[doc = "        15.561681"]
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
    #[serde(rename = "Percent")]
    pub percent: f64,
}
impl ::std::convert::From<&MaterialsItem> for MaterialsItem {
    fn from(value: &MaterialsItem) -> Self {
        value.clone()
    }
}
#[doc = "`ParentsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"Null\": {"]
#[doc = "      \"title\": \"Null\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1,"]
#[doc = "        2"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Planet\": {"]
#[doc = "      \"title\": \"Planet\","]
#[doc = "      \"examples\": ["]
#[doc = "        39,"]
#[doc = "        9,"]
#[doc = "        6"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Ring\": {"]
#[doc = "      \"title\": \"Ring\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        15,"]
#[doc = "        7"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Star\": {"]
#[doc = "      \"title\": \"Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        0,"]
#[doc = "        1,"]
#[doc = "        2"]
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
pub struct ParentsItem {
    #[serde(
        rename = "Null",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub null: ::std::option::Option<i64>,
    #[serde(
        rename = "Planet",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planet: ::std::option::Option<i64>,
    #[serde(
        rename = "Ring",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ring: ::std::option::Option<i64>,
    #[serde(
        rename = "Star",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub star: ::std::option::Option<i64>,
}
impl ::std::convert::From<&ParentsItem> for ParentsItem {
    fn from(value: &ParentsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ParentsItem {
    fn default() -> Self {
        Self {
            null: Default::default(),
            planet: Default::default(),
            ring: Default::default(),
            star: Default::default(),
        }
    }
}
#[doc = "`RingsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"InnerRad\","]
#[doc = "    \"MassMT\","]
#[doc = "    \"Name\","]
#[doc = "    \"OuterRad\","]
#[doc = "    \"RingClass\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"InnerRad\": {"]
#[doc = "      \"title\": \"InnerRad\","]
#[doc = "      \"examples\": ["]
#[doc = "        356060000.0,"]
#[doc = "        745750000.0,"]
#[doc = "        29865000000.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"MassMT\": {"]
#[doc = "      \"title\": \"MassMT\","]
#[doc = "      \"description\": \"In megatons\","]
#[doc = "      \"examples\": ["]
#[doc = "        48319000000000.0,"]
#[doc = "        76821000000000.0,"]
#[doc = "        3572400000000000.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Turdet C A Belt\","]
#[doc = "        \"Latorioson A Belt\","]
#[doc = "        \"Latorioson B Belt\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"OuterRad\": {"]
#[doc = "      \"title\": \"OuterRad\","]
#[doc = "      \"examples\": ["]
#[doc = "        1325200000.0,"]
#[doc = "        1746100000.0,"]
#[doc = "        189630000000.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"RingClass\": {"]
#[doc = "      \"title\": \"RingClass\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"eRingClass_MetalRich\","]
#[doc = "        \"eRingClass_Metalic\","]
#[doc = "        \"eRingClass_Icy\""]
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
pub struct RingsItem {
    #[serde(rename = "InnerRad")]
    pub inner_rad: f64,
    #[serde(rename = "MassMT")]
    pub mass_mt: f64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(rename = "OuterRad")]
    pub outer_rad: f64,
    #[serde(rename = "RingClass")]
    pub ring_class: ::std::string::String,
}
impl ::std::convert::From<&RingsItem> for RingsItem {
    fn from(value: &RingsItem) -> Self {
        value.clone()
    }
}
#[doc = "When Written: basic or detailed discovery scan of a star, planet or moon. This is also generated when scanning a navigation beacon in a populated system, to record info about all the bodies in the system."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: basic or detailed discovery scan of a star, planet or moon. This is also generated when scanning a navigation beacon in a populated system, to record info about all the bodies in the system.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"BodyID\","]
#[doc = "    \"BodyName\","]
#[doc = "    \"DistanceFromArrivalLS\","]
#[doc = "    \"ScanType\","]
#[doc = "    \"StarSystem\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"WasDiscovered\","]
#[doc = "    \"WasMapped\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"AbsoluteMagnitude\": {"]
#[doc = "      \"title\": \"AbsoluteMagnitude\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        10.60585,"]
#[doc = "        8.327072,"]
#[doc = "        18.502441"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Age_MY\": {"]
#[doc = "      \"title\": \"Age_MY\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        206,"]
#[doc = "        3280,"]
#[doc = "        1142"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"AscendingNode\": {"]
#[doc = "      \"title\": \"AscendingNode\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        170.049891,"]
#[doc = "        171.684212,"]
#[doc = "        -133.96371"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Atmosphere\": {"]
#[doc = "      \"title\": \"Atmosphere\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"hot thick water atmosphere\","]
#[doc = "        \"\","]
#[doc = "        \"thin nitrogen atmosphere\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"AtmosphereComposition\": {"]
#[doc = "      \"title\": \"AtmosphereComposition\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Name\","]
#[doc = "          \"Percent\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Water\","]
#[doc = "              \"Nitrogen\","]
#[doc = "              \"Ammonia\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Percent\": {"]
#[doc = "            \"title\": \"Percent\","]
#[doc = "            \"examples\": ["]
#[doc = "              99.124542,"]
#[doc = "              0.492685,"]
#[doc = "              0.302259"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"AtmosphereType\": {"]
#[doc = "      \"title\": \"AtmosphereType\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Water\","]
#[doc = "        \"None\","]
#[doc = "        \"Nitrogen\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"AxialTilt\": {"]
#[doc = "      \"title\": \"AxialTilt\","]
#[doc = "      \"description\": \"If rotating. Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.243948,"]
#[doc = "        0.0,"]
#[doc = "        0.110391"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"BodyID\": {"]
#[doc = "      \"title\": \"BodyID\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        10,"]
#[doc = "        0,"]
#[doc = "        1"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"BodyName\": {"]
#[doc = "      \"title\": \"BodyName\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno 2\","]
#[doc = "        \"Tascheter Sector EL-Y b5\","]
#[doc = "        \"LHS 1443 A\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Composition\": {"]
#[doc = "      \"title\": \"Composition\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Ice\","]
#[doc = "        \"Metal\","]
#[doc = "        \"Rock\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Ice\": {"]
#[doc = "          \"title\": \"Ice\","]
#[doc = "          \"examples\": ["]
#[doc = "            0.008881,"]
#[doc = "            0.0,"]
#[doc = "            0.825387"]
#[doc = "          ],"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"Metal\": {"]
#[doc = "          \"title\": \"Metal\","]
#[doc = "          \"examples\": ["]
#[doc = "            0.329764,"]
#[doc = "            0.327762,"]
#[doc = "            0.015514"]
#[doc = "          ],"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"Rock\": {"]
#[doc = "          \"title\": \"Rock\","]
#[doc = "          \"examples\": ["]
#[doc = "            0.661355,"]
#[doc = "            0.672238,"]
#[doc = "            0.159098"]
#[doc = "          ],"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"DistanceFromArrivalLS\": {"]
#[doc = "      \"title\": \"DistanceFromArrivalLS\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        8370.586818,"]
#[doc = "        0.0,"]
#[doc = "        24.357122"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Eccentricity\": {"]
#[doc = "      \"title\": \"Eccentricity\","]
#[doc = "      \"description\": \"Orbital Parameters for any Star/Planet/Moon (except main star of single-star system)\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.798885,"]
#[doc = "        0.193603,"]
#[doc = "        0.083087"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Landable\": {"]
#[doc = "      \"title\": \"Landable\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Luminosity\": {"]
#[doc = "      \"title\": \"Luminosity\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"VI\","]
#[doc = "        \"V\","]
#[doc = "        \"Va\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"MassEM\": {"]
#[doc = "      \"title\": \"MassEM\","]
#[doc = "      \"description\": \"Body mass. Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        4.880573,"]
#[doc = "        0.257221,"]
#[doc = "        0.284099"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Materials\": {"]
#[doc = "      \"title\": \"Materials\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Name\","]
#[doc = "          \"Percent\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"iron\","]
#[doc = "              \"nickel\","]
#[doc = "              \"sulphur\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"iron\","]
#[doc = "              \"nickel\","]
#[doc = "              \"sulphur\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Percent\": {"]
#[doc = "            \"title\": \"Percent\","]
#[doc = "            \"examples\": ["]
#[doc = "              21.92185,"]
#[doc = "              16.580769,"]
#[doc = "              15.561681"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"MeanAnomaly\": {"]
#[doc = "      \"title\": \"MeanAnomaly\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        313.759234,"]
#[doc = "        172.208739,"]
#[doc = "        357.274807"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"OrbitalInclination\": {"]
#[doc = "      \"title\": \"OrbitalInclination\","]
#[doc = "      \"description\": \"Orbital Parameters for any Star/Planet/Moon (except main star of single-star system)\","]
#[doc = "      \"examples\": ["]
#[doc = "        -58.522447,"]
#[doc = "        90.915675,"]
#[doc = "        58.871234"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"OrbitalPeriod\": {"]
#[doc = "      \"title\": \"OrbitalPeriod\","]
#[doc = "      \"description\": \"Orbital Parameters for any Star/Planet/Moon (except main star of single-star system)\","]
#[doc = "      \"examples\": ["]
#[doc = "        1064090549.945831,"]
#[doc = "        31656362414.360046,"]
#[doc = "        676484.233141"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Parents\": {"]
#[doc = "      \"title\": \"Parents\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"Null\": {"]
#[doc = "            \"title\": \"Null\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              1,"]
#[doc = "              2"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Planet\": {"]
#[doc = "            \"title\": \"Planet\","]
#[doc = "            \"examples\": ["]
#[doc = "              39,"]
#[doc = "              9,"]
#[doc = "              6"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Ring\": {"]
#[doc = "            \"title\": \"Ring\","]
#[doc = "            \"examples\": ["]
#[doc = "              1,"]
#[doc = "              15,"]
#[doc = "              7"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Star\": {"]
#[doc = "            \"title\": \"Star\","]
#[doc = "            \"examples\": ["]
#[doc = "              0,"]
#[doc = "              1,"]
#[doc = "              2"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Periapsis\": {"]
#[doc = "      \"title\": \"Periapsis\","]
#[doc = "      \"description\": \"Orbital Parameters for any Star/Planet/Moon (except main star of single-star system)\","]
#[doc = "      \"examples\": ["]
#[doc = "        10.00731,"]
#[doc = "        282.962726,"]
#[doc = "        155.180146"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"PlanetClass\": {"]
#[doc = "      \"title\": \"PlanetClass\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"High metal content body\","]
#[doc = "        \"Icy body\","]
#[doc = "        \"Sudarsky class I gas giant\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Radius\": {"]
#[doc = "      \"title\": \"Radius\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        9575746.0,"]
#[doc = "        360973152,"]
#[doc = "        400260544"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ReserveLevel\": {"]
#[doc = "      \"title\": \"ReserveLevel\","]
#[doc = "      \"description\": \"Written for Planet/Moon. If rings present.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"MajorResources\","]
#[doc = "        \"PristineResources\","]
#[doc = "        \"CommonResources\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Rings\": {"]
#[doc = "      \"title\": \"Rings\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon. If rings present.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"InnerRad\","]
#[doc = "          \"MassMT\","]
#[doc = "          \"Name\","]
#[doc = "          \"OuterRad\","]
#[doc = "          \"RingClass\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"InnerRad\": {"]
#[doc = "            \"title\": \"InnerRad\","]
#[doc = "            \"examples\": ["]
#[doc = "              356060000.0,"]
#[doc = "              745750000.0,"]
#[doc = "              29865000000.0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"MassMT\": {"]
#[doc = "            \"title\": \"MassMT\","]
#[doc = "            \"description\": \"In megatons\","]
#[doc = "            \"examples\": ["]
#[doc = "              48319000000000.0,"]
#[doc = "              76821000000000.0,"]
#[doc = "              3572400000000000.0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Turdet C A Belt\","]
#[doc = "              \"Latorioson A Belt\","]
#[doc = "              \"Latorioson B Belt\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"OuterRad\": {"]
#[doc = "            \"title\": \"OuterRad\","]
#[doc = "            \"examples\": ["]
#[doc = "              1325200000.0,"]
#[doc = "              1746100000.0,"]
#[doc = "              189630000000.0"]
#[doc = "            ],"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"RingClass\": {"]
#[doc = "            \"title\": \"RingClass\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"eRingClass_MetalRich\","]
#[doc = "              \"eRingClass_Metalic\","]
#[doc = "              \"eRingClass_Icy\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"RotationPeriod\": {"]
#[doc = "      \"title\": \"RotationPeriod\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon. If rotating (in seconds)\","]
#[doc = "      \"examples\": ["]
#[doc = "        168284.111267,"]
#[doc = "        151337.187838,"]
#[doc = "        202116.091725"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"ScanType\": {"]
#[doc = "      \"title\": \"ScanType\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"AutoScan\","]
#[doc = "        \"Detailed\","]
#[doc = "        \"NavBeaconDetail\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SemiMajorAxis\": {"]
#[doc = "      \"title\": \"SemiMajorAxis\","]
#[doc = "      \"description\": \"Orbital Parameters for any Star/Planet/Moon (except main star of single-star system)\","]
#[doc = "      \"examples\": ["]
#[doc = "        2441834926605.2246,"]
#[doc = "        3927611351013.184,"]
#[doc = "        6066894054.412842"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"StarSystem\": {"]
#[doc = "      \"title\": \"StarSystem\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Celaeno\","]
#[doc = "        \"Tascheter Sector EL-Y b5\","]
#[doc = "        \"LHS 1443\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StarType\": {"]
#[doc = "      \"title\": \"StarType\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"TTS\","]
#[doc = "        \"M\","]
#[doc = "        \"T\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"StellarMass\": {"]
#[doc = "      \"title\": \"StellarMass\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        0.3125,"]
#[doc = "        0.417969,"]
#[doc = "        0.035156"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"Subclass\": {"]
#[doc = "      \"title\": \"Subclass\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        6,"]
#[doc = "        8"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"SurfaceGravity\": {"]
#[doc = "      \"title\": \"SurfaceGravity\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        21.214608,"]
#[doc = "        6.358215,"]
#[doc = "        6.599853"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"SurfacePressure\": {"]
#[doc = "      \"title\": \"SurfacePressure\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        5377658368,"]
#[doc = "        0.0,"]
#[doc = "        1030.144409"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"SurfaceTemperature\": {"]
#[doc = "      \"title\": \"SurfaceTemperature\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        1442.12085,"]
#[doc = "        2122.0,"]
#[doc = "        3404.0"]
#[doc = "      ],"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"description\": \"Written for Star\","]
#[doc = "      \"examples\": ["]
#[doc = "        198875014308,"]
#[doc = "        11665802143105,"]
#[doc = "        5068732442009"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"TerraformState\": {"]
#[doc = "      \"title\": \"TerraformState\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"\","]
#[doc = "        \"Terraformable\","]
#[doc = "        \"Terraformed\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TidalLock\": {"]
#[doc = "      \"title\": \"TidalLock\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        false,"]
#[doc = "        true"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"Volcanism\": {"]
#[doc = "      \"title\": \"Volcanism\","]
#[doc = "      \"description\": \"Written for Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"major silicate vapour geysers volcanism\","]
#[doc = "        \"\","]
#[doc = "        \"major rocky magma volcanism\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"WasDiscovered\": {"]
#[doc = "      \"title\": \"WasDiscovered\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"WasFootfalled\": {"]
#[doc = "      \"title\": \"WasFootfalled\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
#[doc = "      ],"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"WasMapped\": {"]
#[doc = "      \"title\": \"WasMapped\","]
#[doc = "      \"description\": \"Written for Star/Planet/Moon\","]
#[doc = "      \"examples\": ["]
#[doc = "        true,"]
#[doc = "        false"]
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
pub struct Scan {
    #[serde(
        rename = "AbsoluteMagnitude",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub absolute_magnitude: ::std::option::Option<f64>,
    #[doc = "Written for Star"]
    #[serde(
        rename = "Age_MY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub age_my: ::std::option::Option<i64>,
    #[serde(
        rename = "AscendingNode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ascending_node: ::std::option::Option<f64>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "Atmosphere",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub atmosphere: ::std::option::Option<::std::string::String>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "AtmosphereComposition",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub atmosphere_composition: ::std::vec::Vec<AtmosphereCompositionItem>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "AtmosphereType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub atmosphere_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "AxialTilt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub axial_tilt: ::std::option::Option<f64>,
    #[doc = "Written for Star/Planet/Moon"]
    #[serde(rename = "BodyID")]
    pub body_id: i64,
    #[doc = "Written for Star/Planet/Moon"]
    #[serde(rename = "BodyName")]
    pub body_name: ::std::string::String,
    #[serde(
        rename = "Composition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub composition: ::std::option::Option<Composition>,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,
    #[serde(
        rename = "Eccentricity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub eccentricity: ::std::option::Option<f64>,
    #[serde(skip_deserializing, default = "Scan::event_value")]
    pub event: String,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "Landable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub landable: ::std::option::Option<bool>,
    #[doc = "Written for Star"]
    #[serde(
        rename = "Luminosity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub luminosity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "MassEM",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mass_em: ::std::option::Option<f64>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "Materials",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub materials: ::std::vec::Vec<MaterialsItem>,
    #[serde(
        rename = "MeanAnomaly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mean_anomaly: ::std::option::Option<f64>,
    #[serde(
        rename = "OrbitalInclination",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub orbital_inclination: ::std::option::Option<f64>,
    #[serde(
        rename = "OrbitalPeriod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub orbital_period: ::std::option::Option<f64>,
    #[doc = "Written for Star/Planet/Moon"]
    #[serde(
        rename = "Parents",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub parents: ::std::vec::Vec<ParentsItem>,
    #[serde(
        rename = "Periapsis",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub periapsis: ::std::option::Option<f64>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "PlanetClass",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planet_class: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Radius",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub radius: ::std::option::Option<f64>,
    #[doc = "Written for Planet/Moon. If rings present."]
    #[serde(
        rename = "ReserveLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reserve_level: ::std::option::Option<::std::string::String>,
    #[doc = "Written for Star/Planet/Moon. If rings present."]
    #[serde(
        rename = "Rings",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub rings: ::std::vec::Vec<RingsItem>,
    #[serde(
        rename = "RotationPeriod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rotation_period: ::std::option::Option<f64>,
    #[doc = "Written for Star/Planet/Moon"]
    #[serde(rename = "ScanType")]
    pub scan_type: ::std::string::String,
    #[serde(
        rename = "SemiMajorAxis",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub semi_major_axis: ::std::option::Option<f64>,
    #[doc = "Written for Star"]
    #[serde(rename = "StarSystem")]
    pub star_system: ::std::string::String,
    #[doc = "Written for Star"]
    #[serde(
        rename = "StarType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub star_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "StellarMass",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stellar_mass: ::std::option::Option<f64>,
    #[doc = "Written for Star"]
    #[serde(
        rename = "Subclass",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subclass: ::std::option::Option<i64>,
    #[serde(
        rename = "SurfaceGravity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surface_gravity: ::std::option::Option<f64>,
    #[serde(
        rename = "SurfacePressure",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surface_pressure: ::std::option::Option<f64>,
    #[serde(
        rename = "SurfaceTemperature",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surface_temperature: ::std::option::Option<f64>,
    #[doc = "Written for Star"]
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "TerraformState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terraform_state: ::std::option::Option<::std::string::String>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "TidalLock",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tidal_lock: ::std::option::Option<bool>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Written for Planet/Moon"]
    #[serde(
        rename = "Volcanism",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub volcanism: ::std::option::Option<::std::string::String>,
    #[doc = "Written for Star/Planet/Moon"]
    #[serde(rename = "WasDiscovered")]
    pub was_discovered: bool,
    #[serde(
        rename = "WasFootfalled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub was_footfalled: ::std::option::Option<bool>,
    #[doc = "Written for Star/Planet/Moon"]
    #[serde(rename = "WasMapped")]
    pub was_mapped: bool,
}
impl ::std::convert::From<&Scan> for Scan {
    fn from(value: &Scan) -> Self {
        value.clone()
    }
}

impl Scan {
    pub fn event_value() -> ::std::string::String {
        "Scan".to_string()
    }
}
