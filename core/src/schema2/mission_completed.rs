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
#[doc = "`CommodityRewardItem`"]
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
#[doc = "        5,"]
#[doc = "        2,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"ModularTerminals\","]
#[doc = "        \"Bauxite\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Modular Terminals\""]
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
pub struct CommodityRewardItem {
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
impl ::std::convert::From<&CommodityRewardItem> for CommodityRewardItem {
    fn from(value: &CommodityRewardItem) -> Self {
        value.clone()
    }
}
#[doc = "`EffectsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Effect\","]
#[doc = "    \"Trend\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Effect\": {"]
#[doc = "      \"title\": \"Effect\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$MISSIONUTIL_Interaction_Summary_EP_up;\","]
#[doc = "        \"$MISSIONUTIL_Interaction_Summary_SP_up;\","]
#[doc = "        \"$MISSIONUTIL_Interaction_Summary_EP_down;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Effect_Localised\": {"]
#[doc = "      \"title\": \"Effect_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"The economic status of $#MinorFaction; has improved in the $#System; system.\","]
#[doc = "        \"The security status of $#MinorFaction; has improved in the $#System; system.\","]
#[doc = "        \"The economic status of $#MinorFaction; has declined in the $#System; system.\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Trend\": {"]
#[doc = "      \"title\": \"Trend\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"UpGood\","]
#[doc = "        \"DownBad\","]
#[doc = "        \"DownGood\""]
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
pub struct EffectsItem {
    #[serde(rename = "Effect")]
    pub effect: ::std::string::String,
    #[serde(
        rename = "Effect_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub effect_localised: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Trend")]
    pub trend: ::std::string::String,
}
impl ::std::convert::From<&EffectsItem> for EffectsItem {
    fn from(value: &EffectsItem) -> Self {
        value.clone()
    }
}
#[doc = "`FactionEffectsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Effects\","]
#[doc = "    \"Faction\","]
#[doc = "    \"Influence\","]
#[doc = "    \"Reputation\","]
#[doc = "    \"ReputationTrend\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Effects\": {"]
#[doc = "      \"title\": \"Effects\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Effect\","]
#[doc = "          \"Trend\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Effect\": {"]
#[doc = "            \"title\": \"Effect\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$MISSIONUTIL_Interaction_Summary_EP_up;\","]
#[doc = "              \"$MISSIONUTIL_Interaction_Summary_SP_up;\","]
#[doc = "              \"$MISSIONUTIL_Interaction_Summary_EP_down;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Effect_Localised\": {"]
#[doc = "            \"title\": \"Effect_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"The economic status of $#MinorFaction; has improved in the $#System; system.\","]
#[doc = "              \"The security status of $#MinorFaction; has improved in the $#System; system.\","]
#[doc = "              \"The economic status of $#MinorFaction; has declined in the $#System; system.\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Trend\": {"]
#[doc = "            \"title\": \"Trend\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"UpGood\","]
#[doc = "              \"DownBad\","]
#[doc = "              \"DownGood\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asooraja Netcoms Inc\","]
#[doc = "        \"Alliance of Wadjang\","]
#[doc = "        \"Asooraja Gold Dragons\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Influence\": {"]
#[doc = "      \"title\": \"Influence\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Influence\","]
#[doc = "          \"SystemAddress\","]
#[doc = "          \"Trend\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Influence\": {"]
#[doc = "            \"title\": \"Influence\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"++\","]
#[doc = "              \"+\","]
#[doc = "              \"+++\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"SystemAddress\": {"]
#[doc = "            \"title\": \"SystemAddress\","]
#[doc = "            \"examples\": ["]
#[doc = "              3107442332354,"]
#[doc = "              908620337866,"]
#[doc = "              2557820834522"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Trend\": {"]
#[doc = "            \"title\": \"Trend\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"UpGood\","]
#[doc = "              \"DownBad\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Reputation\": {"]
#[doc = "      \"title\": \"Reputation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"++\","]
#[doc = "        \"+\","]
#[doc = "        \"++++\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ReputationTrend\": {"]
#[doc = "      \"title\": \"ReputationTrend\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"UpGood\","]
#[doc = "        \"DownBad\""]
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
pub struct FactionEffectsItem {
    #[serde(rename = "Effects")]
    pub effects: ::std::vec::Vec<EffectsItem>,
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
    #[serde(rename = "Influence")]
    pub influence: ::std::vec::Vec<InfluenceItem>,
    #[serde(rename = "Reputation")]
    pub reputation: ::std::string::String,
    #[serde(rename = "ReputationTrend")]
    pub reputation_trend: ::std::string::String,
}
impl ::std::convert::From<&FactionEffectsItem> for FactionEffectsItem {
    fn from(value: &FactionEffectsItem) -> Self {
        value.clone()
    }
}
#[doc = "`InfluenceItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Influence\","]
#[doc = "    \"SystemAddress\","]
#[doc = "    \"Trend\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Influence\": {"]
#[doc = "      \"title\": \"Influence\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"++\","]
#[doc = "        \"+\","]
#[doc = "        \"+++\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"SystemAddress\": {"]
#[doc = "      \"title\": \"SystemAddress\","]
#[doc = "      \"examples\": ["]
#[doc = "        3107442332354,"]
#[doc = "        908620337866,"]
#[doc = "        2557820834522"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Trend\": {"]
#[doc = "      \"title\": \"Trend\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"UpGood\","]
#[doc = "        \"DownBad\""]
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
pub struct InfluenceItem {
    #[serde(rename = "Influence")]
    pub influence: ::std::string::String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[serde(rename = "Trend")]
    pub trend: ::std::string::String,
}
impl ::std::convert::From<&InfluenceItem> for InfluenceItem {
    fn from(value: &InfluenceItem) -> Self {
        value.clone()
    }
}
#[doc = "`MaterialsRewardItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Category\","]
#[doc = "    \"Count\","]
#[doc = "    \"Name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Category\": {"]
#[doc = "      \"title\": \"Category\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$MICRORESOURCE_CATEGORY_Data;\","]
#[doc = "        \"$MICRORESOURCE_CATEGORY_Encoded;\","]
#[doc = "        \"$MICRORESOURCE_CATEGORY_Manufactured;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Category_Localised\": {"]
#[doc = "      \"title\": \"Category_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Data\","]
#[doc = "        \"Encoded\","]
#[doc = "        \"Données\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        5,"]
#[doc = "        2,"]
#[doc = "        3"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"ChemicalInventory\","]
#[doc = "        \"PatrolRoutes\","]
#[doc = "        \"PharmaceuticalPatents\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Name_Localised\": {"]
#[doc = "      \"title\": \"Name_Localised\","]
#[doc = "      \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Chemical Inventory\","]
#[doc = "        \"Patrol Routes\","]
#[doc = "        \"Pharmaceutical Patents\""]
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
pub struct MaterialsRewardItem {
    #[serde(rename = "Category")]
    pub category: ::std::string::String,
    #[serde(
        rename = "Category_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub category_localised: ::std::option::Option<::std::string::String>,
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
impl ::std::convert::From<&MaterialsRewardItem> for MaterialsRewardItem {
    fn from(value: &MaterialsRewardItem) -> Self {
        value.clone()
    }
}
#[doc = "When Written: when a mission is completed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"When Written: when a mission is completed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"Faction\","]
#[doc = "    \"MissionID\","]
#[doc = "    \"Name\","]
#[doc = "    \"event\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"Commodity\": {"]
#[doc = "      \"title\": \"Commodity\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$PatrolRoutes_Name;\","]
#[doc = "        \"$Kompromat_Name;\","]
#[doc = "        \"$EmployeeGeneticData_Name;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"CommodityReward\": {"]
#[doc = "      \"title\": \"CommodityReward\","]
#[doc = "      \"description\": \"Names and counts of any commodity rewards\","]
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
#[doc = "              5,"]
#[doc = "              2,"]
#[doc = "              3"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"ModularTerminals\","]
#[doc = "              \"Bauxite\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Modular Terminals\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Commodity_Localised\": {"]
#[doc = "      \"title\": \"Commodity_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Patrol Routes\","]
#[doc = "        \"Kompromat\","]
#[doc = "        \"Employee Genetic Data\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Count\": {"]
#[doc = "      \"title\": \"Count\","]
#[doc = "      \"examples\": ["]
#[doc = "        1,"]
#[doc = "        195,"]
#[doc = "        220"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"DestinationSettlement\": {"]
#[doc = "      \"title\": \"DestinationSettlement\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Skoropadsky Boarding Site\","]
#[doc = "        \"Cataldo Excavation Hub\","]
#[doc = "        \"Degefa Extraction Platform\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"DestinationStation\": {"]
#[doc = "      \"title\": \"DestinationStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Ockels Relay\","]
#[doc = "        \"Jameson Memorial\","]
#[doc = "        \"Agnesi Bastion\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"DestinationSystem\": {"]
#[doc = "      \"title\": \"DestinationSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"LDS 413\","]
#[doc = "        \"Shinrarta Dezhra\","]
#[doc = "        \"LHS 317\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Donated\": {"]
#[doc = "      \"title\": \"Donated\","]
#[doc = "      \"examples\": ["]
#[doc = "        575000,"]
#[doc = "        1000000,"]
#[doc = "        450000"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Donation\": {"]
#[doc = "      \"title\": \"Donation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"575000\","]
#[doc = "        \"1000000\","]
#[doc = "        \"450000\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Faction\": {"]
#[doc = "      \"title\": \"Faction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Asooraja Netcoms Inc\","]
#[doc = "        \"Alliance of Wadjang\","]
#[doc = "        \"Asooraja Gold Dragons\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"FactionEffects\": {"]
#[doc = "      \"title\": \"FactionEffects\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Effects\","]
#[doc = "          \"Faction\","]
#[doc = "          \"Influence\","]
#[doc = "          \"Reputation\","]
#[doc = "          \"ReputationTrend\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Effects\": {"]
#[doc = "            \"title\": \"Effects\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"Effect\","]
#[doc = "                \"Trend\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"Effect\": {"]
#[doc = "                  \"title\": \"Effect\","]
#[doc = "                  \"examples\": ["]
#[doc = "                    \"$MISSIONUTIL_Interaction_Summary_EP_up;\","]
#[doc = "                    \"$MISSIONUTIL_Interaction_Summary_SP_up;\","]
#[doc = "                    \"$MISSIONUTIL_Interaction_Summary_EP_down;\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"Effect_Localised\": {"]
#[doc = "                  \"title\": \"Effect_Localised\","]
#[doc = "                  \"examples\": ["]
#[doc = "                    \"The economic status of $#MinorFaction; has improved in the $#System; system.\","]
#[doc = "                    \"The security status of $#MinorFaction; has improved in the $#System; system.\","]
#[doc = "                    \"The economic status of $#MinorFaction; has declined in the $#System; system.\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"Trend\": {"]
#[doc = "                  \"title\": \"Trend\","]
#[doc = "                  \"examples\": ["]
#[doc = "                    \"UpGood\","]
#[doc = "                    \"DownBad\","]
#[doc = "                    \"DownGood\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"Faction\": {"]
#[doc = "            \"title\": \"Faction\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Asooraja Netcoms Inc\","]
#[doc = "              \"Alliance of Wadjang\","]
#[doc = "              \"Asooraja Gold Dragons\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Influence\": {"]
#[doc = "            \"title\": \"Influence\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"Influence\","]
#[doc = "                \"SystemAddress\","]
#[doc = "                \"Trend\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"Influence\": {"]
#[doc = "                  \"title\": \"Influence\","]
#[doc = "                  \"examples\": ["]
#[doc = "                    \"++\","]
#[doc = "                    \"+\","]
#[doc = "                    \"+++\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"SystemAddress\": {"]
#[doc = "                  \"title\": \"SystemAddress\","]
#[doc = "                  \"examples\": ["]
#[doc = "                    3107442332354,"]
#[doc = "                    908620337866,"]
#[doc = "                    2557820834522"]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"Trend\": {"]
#[doc = "                  \"title\": \"Trend\","]
#[doc = "                  \"examples\": ["]
#[doc = "                    \"UpGood\","]
#[doc = "                    \"DownBad\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"Reputation\": {"]
#[doc = "            \"title\": \"Reputation\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"++\","]
#[doc = "              \"+\","]
#[doc = "              \"++++\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"ReputationTrend\": {"]
#[doc = "            \"title\": \"ReputationTrend\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"UpGood\","]
#[doc = "              \"DownBad\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"KillCount\": {"]
#[doc = "      \"title\": \"KillCount\","]
#[doc = "      \"examples\": ["]
#[doc = "        9,"]
#[doc = "        13,"]
#[doc = "        7"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"LocalisedName\": {"]
#[doc = "      \"title\": \"LocalisedName\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Turn on power at Walter Drilling Platform\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"MaterialsReward\": {"]
#[doc = "      \"title\": \"MaterialsReward\","]
#[doc = "      \"description\": \"Name, category and count of any material rewards\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"Category\","]
#[doc = "          \"Count\","]
#[doc = "          \"Name\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"Category\": {"]
#[doc = "            \"title\": \"Category\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"$MICRORESOURCE_CATEGORY_Data;\","]
#[doc = "              \"$MICRORESOURCE_CATEGORY_Encoded;\","]
#[doc = "              \"$MICRORESOURCE_CATEGORY_Manufactured;\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Category_Localised\": {"]
#[doc = "            \"title\": \"Category_Localised\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Data\","]
#[doc = "              \"Encoded\","]
#[doc = "              \"Données\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Count\": {"]
#[doc = "            \"title\": \"Count\","]
#[doc = "            \"examples\": ["]
#[doc = "              5,"]
#[doc = "              2,"]
#[doc = "              3"]
#[doc = "            ],"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"Name\": {"]
#[doc = "            \"title\": \"Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"ChemicalInventory\","]
#[doc = "              \"PatrolRoutes\","]
#[doc = "              \"PharmaceuticalPatents\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Name_Localised\": {"]
#[doc = "            \"title\": \"Name_Localised\","]
#[doc = "            \"description\": \"The localised value will be omitted if it is exactly the same as Name\","]
#[doc = "            \"examples\": ["]
#[doc = "              \"Chemical Inventory\","]
#[doc = "              \"Patrol Routes\","]
#[doc = "              \"Pharmaceutical Patents\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"MissionID\": {"]
#[doc = "      \"title\": \"MissionID\","]
#[doc = "      \"examples\": ["]
#[doc = "        869506868,"]
#[doc = "        869506883,"]
#[doc = "        869562920"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Mission_OnFoot_Hack_Download_Offline_MB_name\","]
#[doc = "        \"Mission_OnFoot_Hack_Download_MB_name\","]
#[doc = "        \"Mission_OnFoot_Hack_Download_Covert_MB_name\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewDestinationStation\": {"]
#[doc = "      \"title\": \"NewDestinationStation\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Crown Depot\","]
#[doc = "        \"Otiman Dock\","]
#[doc = "        \"Jameson Memorial\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"NewDestinationSystem\": {"]
#[doc = "      \"title\": \"NewDestinationSystem\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Harow\","]
#[doc = "        \"Robigo\","]
#[doc = "        \"Bediae\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"PermitsAwarded\": {"]
#[doc = "      \"title\": \"PermitsAwarded\","]
#[doc = "      \"description\": \"Appears in journal manual, but not in actual events.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"examples\": ["]
#[doc = "          \"permit\","]
#[doc = "          \"permit\","]
#[doc = "          \"permit\""]
#[doc = "        ],"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Reward\": {"]
#[doc = "      \"title\": \"Reward\","]
#[doc = "      \"examples\": ["]
#[doc = "        221176,"]
#[doc = "        100656,"]
#[doc = "        238235"]
#[doc = "      ],"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"Target\": {"]
#[doc = "      \"title\": \"Target\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$MissionUtil_FactionTag_Datalink;\","]
#[doc = "        \"Karissa Kennedy\","]
#[doc = "        \"Esmeralda Garrison\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetFaction\": {"]
#[doc = "      \"title\": \"TargetFaction\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Kipsigines Vision Partners\","]
#[doc = "        \"44th Vulture Syndicate\","]
#[doc = "        \"Skeggiko O Jet Natural Inc\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetType\": {"]
#[doc = "      \"title\": \"TargetType\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"$MissionUtil_FactionTag_Skimmer;\","]
#[doc = "        \"$MissionUtil_FactionTag_AIHumanoid;\","]
#[doc = "        \"$MissionUtil_FactionTag_GuardHumanoid;\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"TargetType_Localised\": {"]
#[doc = "      \"title\": \"TargetType_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Sentry Skimmer\","]
#[doc = "        \"Faction Members\","]
#[doc = "        \"Guards\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Target_Localised\": {"]
#[doc = "      \"title\": \"Target_Localised\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Data Link\","]
#[doc = "        \"Hub Access Terminal\","]
#[doc = "        \"Fuel Depot\""]
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
pub struct MissionCompleted {
    #[serde(
        rename = "Commodity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Commodity_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub commodity_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Names and counts of any commodity rewards"]
    #[serde(
        rename = "CommodityReward",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub commodity_reward: ::std::vec::Vec<CommodityRewardItem>,
    #[serde(
        rename = "Count",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub count: ::std::option::Option<i64>,
    #[serde(
        rename = "DestinationSettlement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination_settlement: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "DestinationStation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination_station: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "DestinationSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub destination_system: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Donated",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub donated: ::std::option::Option<i64>,
    #[serde(
        rename = "Donation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub donation: ::std::option::Option<::std::string::String>,
    #[serde(skip_deserializing, default = "MissionCompleted::event_value")]
    pub event: String,
    #[serde(rename = "Faction")]
    pub faction: ::std::string::String,
    #[serde(
        rename = "FactionEffects",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub faction_effects: ::std::vec::Vec<FactionEffectsItem>,
    #[serde(
        rename = "KillCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub kill_count: ::std::option::Option<i64>,
    #[serde(
        rename = "LocalisedName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub localised_name: ::std::option::Option<::std::string::String>,
    #[doc = "Name, category and count of any material rewards"]
    #[serde(
        rename = "MaterialsReward",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub materials_reward: ::std::vec::Vec<MaterialsRewardItem>,
    #[serde(rename = "MissionID")]
    pub mission_id: i64,
    #[serde(rename = "Name")]
    pub name: ::std::string::String,
    #[serde(
        rename = "NewDestinationStation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_destination_station: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "NewDestinationSystem",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub new_destination_system: ::std::option::Option<::std::string::String>,
    #[doc = "Appears in journal manual, but not in actual events."]
    #[serde(
        rename = "PermitsAwarded",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub permits_awarded: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "Reward",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reward: ::std::option::Option<i64>,
    #[serde(
        rename = "Target",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "TargetFaction",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_faction: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Target_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_localised: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "TargetType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "TargetType_Localised",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_type_localised: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp in UTC, ISO 8601"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&MissionCompleted> for MissionCompleted {
    fn from(value: &MissionCompleted) -> Self {
        value.clone()
    }
}

impl MissionCompleted {
    pub fn event_value() -> ::std::string::String {
        "MissionCompleted".to_string()
    }
}
