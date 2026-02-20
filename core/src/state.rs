#![allow(dead_code)]

use crate::schema2::loadout::{Engineering, ModulesItem};
use crate::schema2::{engineer_craft, loadout};
use std::collections::{HashMap, HashSet};

#[cfg(feature = "python")]
use pyo3::pyclass;

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub struct ModuleModifier {
    pub label: String,
    pub less_is_good: Option<i64>,
    pub original_value: Option<f64>,
    pub value: Option<f64>,
    pub value_str: Option<String>,
    pub value_str_localised: Option<String>,
}

impl From<loadout::ModifiersItem> for ModuleModifier {
    fn from(m: loadout::ModifiersItem) -> Self {
        Self {
            label: m.label,
            less_is_good: m.less_is_good,
            original_value: m.original_value,
            value: m.value,
            value_str: m.value_str,
            value_str_localised: m.value_str_localised,
        }
    }
}

impl From<&engineer_craft::ModifiersItem> for ModuleModifier {
    fn from(m: &engineer_craft::ModifiersItem) -> Self {
        Self {
            label: m.label.clone(),
            less_is_good: Some(m.less_is_good),
            original_value: m.original_value,
            value: m.value,
            value_str: m.value_str.clone(),
            value_str_localised: m.value_str_localised.clone(),
        }
    }
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub struct ModuleEngineering {
    pub blueprint_id: i64,
    pub blueprint_name: String,
    pub engineer: Option<String>,
    pub engineer_id: i64,
    pub experimental_effect: Option<String>,
    pub experimental_effect_localised: Option<String>,
    pub level: i64,
    pub modifiers: Vec<ModuleModifier>,
    pub quality: f64,
}

impl From<Engineering> for ModuleEngineering {
    fn from(value: Engineering) -> Self {
        Self {
            blueprint_id: value.blueprint_id,
            blueprint_name: value.blueprint_name,
            engineer: value.engineer,
            engineer_id: value.engineer_id,
            experimental_effect: value.experimental_effect,
            experimental_effect_localised: value.experimental_effect_localised,
            level: value.level,
            modifiers: value
                .modifiers
                .into_iter()
                .map(ModuleModifier::from)
                .collect(),
            quality: value.quality,
        }
    }
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub struct FuelTanks {
    pub main: f64,
    pub reserve: f64,
}

impl FuelTanks {
    pub fn new(main: f64, reserve: f64) -> FuelTanks {
        Self { main, reserve }
    }
}
#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub struct Module {
    pub ammo_in_clip: Option<i64>,
    pub ammo_in_hopper: Option<i64>,
    pub engineering: Option<ModuleEngineering>,
    pub health: f64,
    pub item: String,
    pub on: bool,
    pub priority: i64,
    pub slot: String,
    pub value: Option<i64>,
}

impl From<&ModulesItem> for Module {
    fn from(value: &ModulesItem) -> Self {
        Self {
            ammo_in_clip: value.ammo_in_clip,
            ammo_in_hopper: value.ammo_in_hopper,
            engineering: value.engineering.clone().map(ModuleEngineering::from),
            health: value.health,
            item: value.item.clone(),
            on: value.on,
            priority: value.priority,
            slot: value.slot.clone(),
            value: value.value,
        }
    }
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub struct Suit {
    pub suit_id: i64,
    pub edmc_name: String,
    pub loc_name: String,
    pub name: String,
    pub mods: Vec<String>,
    pub id: Option<i32>,
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub struct SuitLoadout {
    pub loadout_slot_id: i64,
    pub current_suit: Suit,
    pub name: String,
    pub slots: HashMap<String, String>,
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug, Default)]
pub struct BackpackContents {
    pub component: HashMap<String, i32>,
    pub consumable: HashMap<String, i32>,
    pub item: HashMap<String, i32>,
    pub data: HashMap<String, i32>,
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug, Default)]
pub struct PowerplayInfo {
    pub power: Option<String>,
    pub rank: Option<i32>,
    pub merits: Option<i64>,
    pub votes: Option<i64>,
    pub time_pledged: Option<i64>,
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug)]
pub enum EngineerProgress {
    Ranked(i32, i32),
    Progress(String),
}

#[cfg_attr(feature = "python", pyclass(frozen, skip_from_py_object))]
#[derive(Clone, Debug, Default)]
pub struct GameState {
    // File header
    pub game_language: Option<String>,
    pub game_version: Option<String>,
    pub game_build: Option<String>,

    // General profile
    pub captain: Option<String>,
    pub cargo: HashMap<String, i32>,
    pub credits: i64,
    pub fid: Option<String>,
    pub horizons: Option<bool>,
    pub odyssey: bool,
    pub loan: Option<i64>,

    // Materials
    pub raw: HashMap<String, i32>,
    pub manufactured: HashMap<String, i32>,
    pub encoded: HashMap<String, i32>,

    // Misc dictionaries
    pub engineers: HashMap<String, EngineerProgress>, // variant<(i32,i32), String>
    pub rank: HashMap<String, (i32, i32)>,
    pub reputation: HashMap<String, i32>,
    pub statistics: HashMap<String, i32>,

    // Crew/role
    pub role: Option<String>,

    // Friends
    pub friends: HashSet<String>,

    // Ship details
    pub ship_id: Option<i64>,
    pub ship_ident: Option<String>,
    pub ship_name: Option<String>,
    pub ship_type: Option<String>,
    pub hull_value: Option<i64>,
    pub modules_value: Option<i64>,
    pub unladen_mass: Option<f64>,
    pub cargo_capacity: Option<i64>,
    pub max_jump_range: Option<f64>,
    pub fuel_capacity: Option<FuelTanks>,
    pub rebuy: Option<i64>,
    pub modules: HashMap<String, Module>,
    pub cargo_json: Option<String>,
    pub route: Option<String>,

    // State flags
    pub is_docked: bool,
    pub on_foot: bool,

    // Odyssey locker
    pub component: HashMap<String, i32>,
    pub item: HashMap<String, i32>,
    pub consumable: HashMap<String, i32>,
    pub data: HashMap<String, i32>,

    // Backpack
    pub backpack: BackpackContents,
    pub backpack_json: Option<String>,
    pub ship_locker_json: Option<String>,

    // Suit data
    pub suits: HashMap<i64, Suit>,
    pub suit_loadouts: HashMap<i64, SuitLoadout>,
    pub suit_current: Option<i64>,
    pub suit_loadout_current: Option<i64>,

    // Taxi/Dropship
    pub taxi: bool,
    pub dropship: bool,

    // Galaxy/system
    pub star_pos: Vec<f64>,
    pub system_address: Option<i64>,
    pub system_name: Option<String>,
    pub system_population: Option<i64>,
    pub body: Option<String>,
    pub body_id: Option<i64>,
    pub body_type: Option<String>,
    pub station_name: Option<String>,
    pub station_type: Option<String>,
    pub market_id: Option<i64>,
    pub nav_route: Option<String>,

    // Powerplay
    pub power_play: PowerplayInfo,
}
