use crate::schema2::{EngineerProgress, StartUp};
use crate::state::{FuelTanks, GameState, Module, ModuleEngineering, ModuleModifier};
use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::process;
use crate::schema2::JournalEvent;
use chrono::{Duration, TimeDelta, TimeZone, Utc};
use notify::EventKind;
use regex::Regex;
use semver::Version;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::LazyLock;
use crossbeam::channel::Sender;
use log::debug;

static RE_LOGFILE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Journal\.(\d{4}-\d{2}-\d{2}T\d{6})\.(\d{2})\.log$").unwrap());

static RE_ONFOOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(FlightSuit|UtilitySuit_Class.|TacticalSuit_Class.|ExplorationSuit_Class.)$")
        .unwrap()
});

static RE_CANONICALISE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\$(.+)_name;$").unwrap());

static RE_FC_JUMPNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^FC ([A-NP-Z0-9]{3})([A-NP-Z0-9]{3})$").unwrap());

static RE_CATEGORY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\$MICRORESOURCE_CATEGORY_(.+);$").unwrap());

#[derive(Debug, Clone, Copy)]
pub struct ShipData {
    pub hull_mass: i32,
    pub reserve_fuel_capacity: f64,
}

static SHIP_DATA: LazyLock<HashMap<&'static str, ShipData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "Adder",
        ShipData {
            hull_mass: 35,
            reserve_fuel_capacity: 0.36,
        },
    );
    m.insert(
        "Alliance Challenger",
        ShipData {
            hull_mass: 450,
            reserve_fuel_capacity: 0.77,
        },
    );
    m.insert(
        "Alliance Chieftain",
        ShipData {
            hull_mass: 400,
            reserve_fuel_capacity: 0.77,
        },
    );
    m.insert(
        "Alliance Crusader",
        ShipData {
            hull_mass: 500,
            reserve_fuel_capacity: 0.77,
        },
    );
    m.insert(
        "Anaconda",
        ShipData {
            hull_mass: 400,
            reserve_fuel_capacity: 1.07,
        },
    );
    m.insert(
        "Asp Explorer",
        ShipData {
            hull_mass: 280,
            reserve_fuel_capacity: 0.63,
        },
    );
    m.insert(
        "Asp Scout",
        ShipData {
            hull_mass: 150,
            reserve_fuel_capacity: 0.47,
        },
    );
    m.insert(
        "Beluga Liner",
        ShipData {
            hull_mass: 950,
            reserve_fuel_capacity: 0.81,
        },
    );
    m.insert(
        "Cobra MkIII",
        ShipData {
            hull_mass: 180,
            reserve_fuel_capacity: 0.49,
        },
    );
    m.insert(
        "Cobra MkIV",
        ShipData {
            hull_mass: 210,
            reserve_fuel_capacity: 0.51,
        },
    );
    m.insert(
        "Cobra Mk V",
        ShipData {
            hull_mass: 150,
            reserve_fuel_capacity: 0.49,
        },
    );
    m.insert(
        "Corsair",
        ShipData {
            hull_mass: 265,
            reserve_fuel_capacity: 0.41,
        },
    );
    m.insert(
        "Diamondback Explorer",
        ShipData {
            hull_mass: 260,
            reserve_fuel_capacity: 0.52,
        },
    );
    m.insert(
        "Diamondback Scout",
        ShipData {
            hull_mass: 170,
            reserve_fuel_capacity: 0.49,
        },
    );
    m.insert(
        "Dolphin",
        ShipData {
            hull_mass: 140,
            reserve_fuel_capacity: 0.50,
        },
    );
    m.insert(
        "Eagle",
        ShipData {
            hull_mass: 50,
            reserve_fuel_capacity: 0.34,
        },
    );
    m.insert(
        "Federal Assault Ship",
        ShipData {
            hull_mass: 480,
            reserve_fuel_capacity: 0.72,
        },
    );
    m.insert(
        "Federal Corvette",
        ShipData {
            hull_mass: 900,
            reserve_fuel_capacity: 1.13,
        },
    );
    m.insert(
        "Federal Dropship",
        ShipData {
            hull_mass: 580,
            reserve_fuel_capacity: 0.83,
        },
    );
    m.insert(
        "Federal Gunship",
        ShipData {
            hull_mass: 580,
            reserve_fuel_capacity: 0.82,
        },
    );
    m.insert(
        "Fer-de-Lance",
        ShipData {
            hull_mass: 250,
            reserve_fuel_capacity: 0.67,
        },
    );
    m.insert(
        "Hauler",
        ShipData {
            hull_mass: 14,
            reserve_fuel_capacity: 0.25,
        },
    );
    m.insert(
        "Imperial Clipper",
        ShipData {
            hull_mass: 400,
            reserve_fuel_capacity: 0.74,
        },
    );
    m.insert(
        "Imperial Courier",
        ShipData {
            hull_mass: 35,
            reserve_fuel_capacity: 0.41,
        },
    );
    m.insert(
        "Imperial Cutter",
        ShipData {
            hull_mass: 1100,
            reserve_fuel_capacity: 1.16,
        },
    );
    m.insert(
        "Imperial Eagle",
        ShipData {
            hull_mass: 50,
            reserve_fuel_capacity: 0.37,
        },
    );
    m.insert(
        "Keelback",
        ShipData {
            hull_mass: 180,
            reserve_fuel_capacity: 0.39,
        },
    );
    m.insert(
        "Krait MkII",
        ShipData {
            hull_mass: 320,
            reserve_fuel_capacity: 0.63,
        },
    );
    m.insert(
        "Krait Phantom",
        ShipData {
            hull_mass: 270,
            reserve_fuel_capacity: 0.63,
        },
    );
    m.insert(
        "Mamba",
        ShipData {
            hull_mass: 250,
            reserve_fuel_capacity: 0.50,
        },
    );
    m.insert(
        "Mandalay",
        ShipData {
            hull_mass: 230,
            reserve_fuel_capacity: 0.52,
        },
    );
    m.insert(
        "Orca",
        ShipData {
            hull_mass: 290,
            reserve_fuel_capacity: 0.79,
        },
    );
    m.insert(
        "Panther Clipper Mk II",
        ShipData {
            hull_mass: 1200,
            reserve_fuel_capacity: 1.11,
        },
    );
    m.insert(
        "Python",
        ShipData {
            hull_mass: 350,
            reserve_fuel_capacity: 0.83,
        },
    );
    m.insert(
        "Python Mk II",
        ShipData {
            hull_mass: 450,
            reserve_fuel_capacity: 0.83,
        },
    );
    m.insert(
        "Sidewinder",
        ShipData {
            hull_mass: 25,
            reserve_fuel_capacity: 0.30,
        },
    );
    m.insert(
        "Type-10 Defender",
        ShipData {
            hull_mass: 1200,
            reserve_fuel_capacity: 0.77,
        },
    );
    m.insert(
        "Type-11 Prospector",
        ShipData {
            hull_mass: 320,
            reserve_fuel_capacity: 0.60,
        },
    );
    m.insert(
        "Type-6 Transporter",
        ShipData {
            hull_mass: 155,
            reserve_fuel_capacity: 0.39,
        },
    );
    m.insert(
        "Type-7 Transporter",
        ShipData {
            hull_mass: 350,
            reserve_fuel_capacity: 0.52,
        },
    );
    m.insert(
        "Type-8 Transporter",
        ShipData {
            hull_mass: 400,
            reserve_fuel_capacity: 0.52,
        },
    );
    m.insert(
        "Type-9 Heavy",
        ShipData {
            hull_mass: 850,
            reserve_fuel_capacity: 0.77,
        },
    );
    m.insert(
        "Viper MkIII",
        ShipData {
            hull_mass: 50,
            reserve_fuel_capacity: 0.41,
        },
    );
    m.insert(
        "Viper MkIV",
        ShipData {
            hull_mass: 190,
            reserve_fuel_capacity: 0.46,
        },
    );
    m.insert(
        "Vulture",
        ShipData {
            hull_mass: 230,
            reserve_fuel_capacity: 0.57,
        },
    );
    m
});

static SHIP_NAME_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("adder", "Adder");
    m.insert("anaconda", "Anaconda");
    m.insert("asp", "Asp Explorer");
    m.insert("asp_scout", "Asp Scout");
    m.insert("belugaliner", "Beluga Liner");
    m.insert("cobramkiii", "Cobra MkIII");
    m.insert("cobramkiv", "Cobra MkIV");
    m.insert("cobramkv", "Cobra Mk V");
    m.insert("corsair", "Corsair");
    m.insert("clipper", "Panther Clipper");
    m.insert("cutter", "Imperial Cutter");
    m.insert("diamondback", "Diamondback Scout");
    m.insert("diamondbackxl", "Diamondback Explorer");
    m.insert("dolphin", "Dolphin");
    m.insert("eagle", "Eagle");
    m.insert("empire_courier", "Imperial Courier");
    m.insert("empire_eagle", "Imperial Eagle");
    m.insert("empire_fighter", "Imperial Fighter");
    m.insert("empire_trader", "Imperial Clipper");
    m.insert("federation_corvette", "Federal Corvette");
    m.insert("federation_dropship", "Federal Dropship");
    m.insert("federation_dropship_mkii", "Federal Assault Ship");
    m.insert("federation_gunship", "Federal Gunship");
    m.insert("federation_fighter", "F63 Condor");
    m.insert("ferdelance", "Fer-de-Lance");
    m.insert("hauler", "Hauler");
    m.insert("independant_trader", "Keelback");
    m.insert("independent_fighter", "Taipan Fighter");
    m.insert("krait_mkii", "Krait MkII");
    m.insert("krait_light", "Krait Phantom");
    m.insert("lakonminer", "Type-11 Prospector");
    m.insert("mamba", "Mamba");
    m.insert("mandalay", "Mandalay");
    m.insert("orca", "Orca");
    m.insert("panthermkii", "Panther Clipper Mk II");
    m.insert("raxxla", "Python");
    m.insert("python_nx", "Python Mk II");
    m.insert("scout", "Taipan Fighter");
    m.insert("sidewinder", "Sidewinder");
    m.insert("testbuggy", "Scarab");
    m.insert("type6", "Type-6 Transporter");
    m.insert("type7", "Type-7 Transporter");
    m.insert("type8", "Type-8 Transporter");
    m.insert("type9", "Type-9 Heavy");
    m.insert("type9_military", "Type-10 Defender");
    m.insert("typex", "Alliance Chieftain");
    m.insert("typex_2", "Alliance Crusader");
    m.insert("typex_3", "Alliance Challenger");
    m.insert("viper", "Viper MkIII");
    m.insert("viper_mkiv", "Viper MkIV");
    m.insert("vulture", "Vulture");
    m
});

fn coerce_semver(input: &str) -> Result<Version, anyhow::Error> {
    let mut digits = String::with_capacity(input.len());

    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' {
            digits.push(c);
        } else {
            digits.push('.');
        }
    }

    let mut nums = [0u64; 3];

    for (i, tok) in digits
        .split('.')
        .filter(|s| !s.is_empty())
        .take(3)
        .enumerate()
    {
        nums[i] = tok.parse()?;
    }

    Ok(Version {
        major: nums[0],
        minor: nums[1],
        patch: nums[2],
        pre: Default::default(),
        build: Default::default(),
    })
}

fn canonicalise(name: &str) -> String {
    if name == "" {
        return "".to_string();
    }
    if let Some(caps) = RE_CANONICALISE.captures(name) {
        return caps.get(1).unwrap().as_str().to_lowercase();
    }
    name.to_lowercase()
}

fn category(name: &str) -> String {
    if name == "" {
        return "".to_string();
    }

    if let Some(caps) = RE_CATEGORY.captures(name) {
        return caps.get(1).unwrap().as_str().to_lowercase();
    }

    name.to_lowercase()
}

#[derive(Debug)]
struct FileEvent {
    journal: Option<JournalEvent>,
    file: Option<JournalEvent>,
    delta: Duration,
}

impl FileEvent {
    pub fn new(delta: TimeDelta) -> Self {
        Self {
            journal: None,
            file: None,
            delta,
        }
    }

    pub fn on_journal(&mut self, event: JournalEvent) -> Option<JournalEvent> {
        if let Some(ev) = self.file.take() {
            if (ev.timestamp() - event.timestamp()).abs() <= self.delta {
                Some(ev)
            } else {
                self.journal = Some(event);
                self.file = Some(ev);
                None
            }
        } else {
            self.journal = Some(event);
            None
        }
    }

    pub fn on_file(&mut self, event: JournalEvent) -> Option<JournalEvent> {
        if let Some(ev) = self.journal.take() {
            if (ev.timestamp() - event.timestamp()).abs() <= self.delta {
                Some(ev)
            } else {
                self.file = Some(event);
                self.journal = Some(ev);
                None
            }
        } else {
            self.file = Some(event);
            None
        }
    }
}

#[derive(Debug)]
pub struct Journal {
    journal_dir: PathBuf,
    sender: Sender<anyhow::Result<(GameState, JournalEvent)>>,
    log_handle: Option<BufReader<File>>,
    log_file: Option<String>,

    cargo_event: FileEvent,
    backpack_event: FileEvent,
    market_event: FileEvent,
    shipyard_event: FileEvent,
    navroute_event: FileEvent,
    outfitting_event: FileEvent,
    fcmaterials_event: FileEvent,
    moduleinfo_event: FileEvent,

    live: bool,
    init: bool,
    is_beta: bool,
    replay: bool,
    game_was_running: bool,
    started: chrono::DateTime<chrono::Utc>,

    cmdr: String,
    mode: String,
    group: String,
    version: String,
    semver: Option<Version>,
    carrier_ids: HashMap<i64, String>,

    station_services: Vec<String>,
    slef: String,

    state: GameState,
}

impl Journal {
    pub fn new(
        journal_dir: PathBuf,
        delta: Duration,
        sender: Sender<anyhow::Result<(GameState, JournalEvent)>>,
    ) -> Self {
        Self {
            journal_dir,
            sender,
            log_handle: None,
            log_file: None,
            cargo_event: FileEvent::new(delta),
            backpack_event: FileEvent::new(delta),
            market_event: FileEvent::new(delta),
            shipyard_event: FileEvent::new(delta),
            navroute_event: FileEvent::new(delta),
            outfitting_event: FileEvent::new(delta),
            fcmaterials_event: FileEvent::new(delta),
            moduleinfo_event: FileEvent::new(delta),
            live: false,
            init: false,
            is_beta: false,
            replay: false,
            game_was_running: false,
            started: chrono::Utc.timestamp_nanos(0),
            cmdr: "".to_string(),
            mode: "".to_string(),
            group: "".to_string(),
            version: "".to_string(),
            semver: None,
            station_services: vec![],
            state: Default::default(),
            carrier_ids: HashMap::new(),
            slef: "".to_string(),
        }
    }

    fn newest_journal(&self) -> Option<String> {

        let mut files: Vec<PathBuf> = Vec::new();

        let entries = match fs::read_dir(&self.journal_dir) {
            Ok(e) => e,
            Err(_) => return None,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if RE_LOGFILE.is_match(name) {
                    files.push(path);
                }
            }
        }

        match files.into_iter().max() {
            Some(latest) => latest
                .file_name()
                .and_then(|n| n.to_str()).map(|n| n.to_string()),
            None => None
        }
    }

    pub fn init(&mut self) {
        if self.init {
            return;
        }

        if !std::path::Path::new(&self.journal_dir).is_dir() {
            return;
        }

        self.log_file = self.newest_journal();

        if let Some(log_file) = self.log_file.clone() {
            log::debug!("Replaying journal file {}", log_file);

            let path = std::path::Path::new(&self.journal_dir).join(&log_file);

            if let Ok(file) = File::open(path) {
                self.log_handle = Some(BufReader::new(file));
                self.replay = true;

                let mut line = String::new();

                while let Some(reader) = self.log_handle.as_mut() {
                    line.clear();

                    match reader.read_line(&mut line) {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            if let Ok(ev) = serde_json::from_str::<JournalEvent>(&line) {
                                self.parse_entry(ev);
                            }
                        }
                        Err(_) => break,
                    }
                }
            }

            self.replay = false;
            log::debug!("finished journal file {}", log_file);
        }



        self.game_was_running = process::running(self.journal_dir.to_str().unwrap());

        if self.live {
            if self.game_was_running {
                log::debug!("game was/is running synthesizing StartUp event");
                let entry = self.synthesize_startup_event();
                let _ = self.sender.send(Ok((self.state.clone(), entry)));
            } else {
                self.live = false;
            }
        }

        self.init = true;
    }

    fn synthesize_startup_event(&self) -> JournalEvent {
        JournalEvent::StartUp(StartUp {
            timestamp: Utc::now(),
            event: "StartUp".to_string(),
            star_system: self.state.system_name.clone().unwrap_or("".to_string()),
            star_pos: self.state.star_pos.clone(),
            system_address: self.state.system_address.unwrap_or(0),
            system_population: self.state.system_population.unwrap_or(0),
            body: self.state.body.clone(),
            body_id: self.state.body_id,
            body_type: self.state.body_type.clone(),
            docked: self.state.is_docked,
            market_id: self.state.market_id,
            station_name: self.state.station_name.clone(),
            station_type: self.state.station_type.clone(),
        })
    }

    pub fn handle_event(&mut self, event: notify::Event) {
        debug!("{:?}", event);
        match event.kind {
            EventKind::Create(_) => {
                if event.paths.is_empty() {
                    return;
                }

                let path = &event.paths[0];
                let file_name = match path.file_name().and_then(|f| f.to_str()) {
                    Some(name) => name,
                    None => return,
                };

                // Compare string slices
                if RE_LOGFILE.is_match(file_name) && self.log_file.as_deref() != Some(file_name) {
                    self.log_handle = None;

                    let path = self.journal_dir.join(file_name);
                    if let Ok(f) = File::open(&path) {
                        self.log_handle = Some(BufReader::new(f));
                        self.log_file = Some(file_name.to_string());
                    }
                }
            }
            EventKind::Modify(_) => {
                let path = &event.paths[0];
                let file_name = match path.file_name().and_then(|f| f.to_str()) {
                    Some(name) => name,
                    None => return,
                };
                if self.log_file.as_deref() == Some(file_name) {
                    if let Some(mut buf) = self.log_handle.take() {
                        let mut line = String::new();

                        loop {
                            line.clear();

                            match buf.read_line(&mut line) {
                                Ok(0) => break,
                                Ok(_) => {
                                    match serde_json::from_str::<JournalEvent>(&line) {
                                        Ok(e) => {
                                            if let Some(x) = self.parse_entry(e) {
                                                let _ = self.sender.send(Ok((self.state.clone(), x)));
                                            }
                                        }
                                        Err(err) => {
                                            let _ = self.sender.send(Err(err.into()));
                                        }
                                    }
                                }
                                Err(_) => break,
                            }
                        }

                        self.log_handle = Some(buf);
                    }
                } else {
                    match file_name {
                        "Cargo.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.cargo_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "Backpack.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.backpack_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "Market.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.market_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "Shipyard.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.shipyard_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "NavRoute.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.navroute_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "Outfitting.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.outfitting_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "FCMaterials.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.fcmaterials_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        "ModulesInfo.json" => {
                            let result = match fs::File::open(path) {
                                Ok(mut f) => match serde_json::from_reader::<_, JournalEvent>(&mut f) {
                                    Ok(ev) => {
                                        if let Some(x) = self.moduleinfo_event.on_file(ev) {
                                            Ok((self.state.clone(), x))
                                        } else {
                                            return;
                                        }
                                    }
                                    Err(e) => Err(e.into()),
                                },
                                Err(e) => Err(e.into()),
                            };

                            let _ = self.sender.send(result);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn parse_entry(&mut self, mut event: JournalEvent) -> Option<JournalEvent> {
        match &mut event {
            JournalEvent::Fileheader(_) => {
                self.live = false;
                self.cmdr.clear();
                self.mode.clear();
                self.group.clear();
                self.station_services.clear();
                self.started = chrono::Utc.timestamp_nanos(0);

                self.state = GameState::default();
                self.populate_version_info(&event);
            }
            JournalEvent::Commander(ev) => {
                self.live = true;
                self.cmdr = ev.name.clone();
                self.state.fid = Some(ev.fid.clone());
            }
            JournalEvent::LoadGame(ev) => {
                self.cmdr = ev.commander.clone();
                if (ev.ship.is_some() && ev.game_mode.is_none())
                    || ev.game_mode.as_deref().unwrap_or("") == "cqc"
                {
                    self.mode = "CQC".to_string();
                } else {
                    self.mode = match &ev.game_mode {
                        Some(v) => v.clone(),
                        None => "".to_string(),
                    }
                }

                self.group = ev.group.clone().unwrap_or("".to_string());
                self.state.system_address = None;
                self.state.system_name = None;
                self.state.system_population = None;
                self.state.star_pos.clear();
                self.state.body = None;
                self.state.body_id = None;
                self.state.body_type = None;
                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;

                self.station_services.clear();

                self.started = ev.timestamp.clone();

                self.state.captain = None;
                self.state.credits = ev.credits;
                self.state.fid = Some(ev.fid.clone());
                self.state.horizons = Some(ev.horizons);
                self.state.odyssey = ev.odyssey.unwrap_or(false);
                self.state.loan = Some(ev.loan);

                // Statistics reset
                self.state.statistics = Default::default(); // empty struct/map

                self.state.role = None;
                self.state.taxi = false;
                self.state.dropship = false;

                self.state.on_foot =
                    ev.ship.is_some() && RE_ONFOOT.is_match(ev.ship.as_deref().unwrap().as_ref());
                self.populate_version_info(&event);
            }
            JournalEvent::NewCommander(ev) => {
                self.cmdr = ev.name.clone();
                self.group.clear();
            }
            JournalEvent::SetUserShipName(ev) => {
                self.state.ship_id = Some(ev.ship_id.clone());
                self.state.ship_ident = Some(ev.user_ship_id.clone());
                self.state.ship_name = Some(ev.user_ship_name.clone());
                self.state.ship_type = Some(canonicalise(ev.ship.as_str()));
            }
            JournalEvent::ShipyardBuy(ev) => {
                self.state.ship_id = None;
                self.state.ship_ident = None;
                self.state.ship_name = None;
                self.state.ship_type = Some(canonicalise(ev.ship_type.as_str()));

                self.state.hull_value = None;
                self.state.modules_value = None;
                self.state.rebuy = None;
                self.state.modules.clear();

                self.state.credits -= ev.ship_price;
            }
            JournalEvent::ShipyardSwap(ev) => {
                self.state.ship_id = Some(ev.ship_id.clone());
                self.state.ship_ident = None;
                self.state.ship_name = None;
                self.state.ship_type = Some(canonicalise(ev.ship_type.as_str()));

                self.state.hull_value = None;
                self.state.modules_value = None;
                self.state.rebuy = None;
                self.state.modules.clear();
            }
            JournalEvent::CarrierStats(ev) => {
                self.carrier_ids.insert(ev.carrier_id, ev.callsign.clone());
            }
            JournalEvent::CarrierJumpRequest(ev) => {
                if let Some(carrier_id) = self.carrier_ids.get(&ev.carrier_id) {
                    ev.callsign = carrier_id.clone();
                }
            }
            JournalEvent::CarrierJumpCancelled(ev) => {
                if let Some(carrier_id) = self.carrier_ids.get(&ev.carrier_id) {
                    ev.callsign = carrier_id.clone();
                }
            }
            JournalEvent::Loadout(ev) => {
                let ship_type = canonicalise(&ev.ship);
                if !ship_type.contains("fighter") && !ship_type.contains("buggy") {
                    self.state.ship_id = Some(ev.ship_id.clone());
                    self.state.ship_ident = Some(ev.ship.clone());

                    if let Some(name) = &self.state.ship_name {
                        if !name.is_empty() && name != " " {
                            self.state.ship_name = Some(ev.ship_name.clone());
                        }
                    }

                    self.state.ship_type = Some(ship_type.clone());
                    self.state.hull_value = ev.hull_value;
                    self.state.modules_value = ev.modules_value;
                    self.state.unladen_mass = Some(ev.unladen_mass);
                    self.state.cargo_capacity = Some(ev.cargo_capacity);
                    self.state.max_jump_range = Some(ev.max_jump_range);
                    self.state.fuel_capacity = Some(FuelTanks::new(
                        ev.fuel_capacity.main,
                        ev.fuel_capacity.reserve,
                    ));
                    self.state.rebuy = Some(ev.rebuy);

                    // clear modules in event
                    ev.modules.clear();

                    // repopulate modules
                    for module in &ev.modules {
                        let mut mod_copy = Module::from(module);
                        let is_hardpoint = module.slot.contains("Hardpoint")
                            && !module.slot.starts_with("TinyHardpoint");
                        let ammo_clip_is_one =
                            module.ammo_in_clip == Some(1) && module.ammo_in_hopper == Some(1);

                        if is_hardpoint && ammo_clip_is_one {
                            mod_copy.ammo_in_clip = None;
                            mod_copy.ammo_in_hopper = None;
                        }
                        self.state.modules.insert(module.slot.clone(), mod_copy);
                    }

                    // JSON output
                    let mut initial = serde_json::json!({
                        "header": {
                            "appName": "hugh",
                            "appVersion": "0.1.0"
                        }
                    });

                    let mut data = serde_json::Map::new();

                    for module in &ev.modules {
                        if module.slot == "FuelTank" {
                            if let Some(pos) = module.item.find("size") {
                                let after_size = &module.item[pos + 4..];
                                if let Some(underscore) = after_size.find('_') {
                                    let size_num = &after_size[..underscore];
                                    if let Ok(exponent) = size_num.parse::<u32>() {
                                        let cap = 1 << exponent;
                                        let ship_name = &ev.ship_name;
                                        let ship_key = SHIP_NAME_MAP
                                            .get(ship_name.as_str())
                                            .map_or_else(|| ship_name.clone(), |t| t.to_string());

                                        let fuel = serde_json::json!({
                                            "Main": cap,
                                            "Reserve": SHIP_DATA.get(ship_key.as_str()).map_or_else(|| 0.0, |d| d.reserve_fuel_capacity),
                                        });

                                        data.insert("FuelCapacity".to_string(), fuel);
                                    }
                                }
                            }
                        }
                    }

                    data.insert(
                        "Ship".to_string(),
                        serde_json::Value::String(ev.ship.clone()),
                    );
                    data.insert("ShipName".to_string(), Value::String(ev.ship_name.clone()));
                    data.insert(
                        "ShipIdent".to_string(),
                        Value::String(ev.ship_ident.clone()),
                    );
                    if let Some(hull_value) = ev.hull_value {
                        data.insert("HullValue".to_string(), json!(hull_value));
                    }
                    if let Some(modules_value) = ev.modules_value {
                        data.insert("ModulesValue".to_string(), json!(modules_value));
                    }
                    data.insert("Rebuy".to_string(), Value::Number(ev.rebuy.into()));
                    data.insert("MaxJumpRange".to_string(), json!(ev.max_jump_range));
                    data.insert("UnladenMass".to_string(), json!(ev.unladen_mass));
                    data.insert(
                        "CargoCapacity".to_string(),
                        Value::Number(ev.cargo_capacity.into()),
                    );
                    data.insert(
                        "Modules".to_string(),
                        serde_json::to_value(&ev.modules).unwrap(),
                    );

                    initial["schema"] = serde_json::Value::Object(data);

                    let output = serde_json::to_string_pretty(&initial).unwrap();
                    self.slef = format!("[{}]", output);
                }
            }
            JournalEvent::ModuleBuy(ev) => {
                self.state.modules.insert(
                    ev.slot.clone(),
                    Module {
                        ammo_in_clip: None,
                        ammo_in_hopper: None,
                        engineering: None,
                        health: 1.0,
                        item: canonicalise(&ev.buy_item),
                        on: true,
                        priority: 1,
                        slot: ev.slot.clone(),
                        value: Some(ev.buy_price),
                    },
                );
                self.state.credits -= ev.buy_price;
            }

            JournalEvent::ModuleSell(ev) => {
                self.state.modules.remove(&ev.slot);
                self.state.credits += ev.sell_price;
            }

            JournalEvent::ModuleSellRemote(ev) => {
                self.state.credits += ev.sell_price;
            }

            JournalEvent::ModuleStore(ev) => {
                self.state.modules.remove(&ev.slot);
                self.state.credits -= ev.cost.unwrap_or(0);
            }

            JournalEvent::ModuleSwap(ev) => {
                let modules = &mut self.state.modules;

                if let Some(from_mod) = modules.remove(&ev.from_slot) {
                    if let Some(to_mod) = modules.remove(&ev.to_slot) {
                        // Both slots exist → swap them
                        modules.insert(ev.from_slot.clone(), to_mod);
                        modules.insert(ev.to_slot.clone(), from_mod);
                    } else {
                        // Only from_slot exists → move it to to_slot
                        modules.insert(ev.to_slot.clone(), from_mod);
                    }
                }
            }
            JournalEvent::Undocked(_) => {
                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;
                self.station_services.clear();
                self.state.is_docked = false;
            }

            JournalEvent::Embark(ev) => {
                self.state.station_name = None;
                self.state.market_id = None;
                if ev.on_station {
                    self.state.station_name = ev.station_name.clone();
                    self.state.market_id = ev.market_id.clone();
                }
                self.state.on_foot = false;
                self.state.taxi = ev.taxi;

                self.state.backpack.data.clear();
                self.state.backpack.component.clear();
                self.state.backpack.item.clear();
                self.state.backpack.consumable.clear();
            }

            JournalEvent::Disembark(ev) => {
                if !ev.on_station {
                    self.state.station_name = ev.station_name.clone();
                } else {
                    self.state.station_name = None;
                }

                self.state.on_foot = true;
                if !self.state.taxi && self.state.taxi != ev.taxi {
                    // warning: disembarked from a taxi, but we didn't know!
                }

                self.state.taxi = false;
                self.state.dropship = false;
            }

            JournalEvent::DropshipDeploy(_) => {
                self.state.on_foot = true;
                self.state.taxi = false;
                self.state.dropship = false;
            }

            JournalEvent::SupercruiseExit(ev) => {
                if ev.body_type == "Station" {
                    self.state.body = None;
                    self.state.body_id = None;
                }
            }
            JournalEvent::Docked(ev) => {
                self.state.is_docked = true;
                self.state.station_name = Some(ev.station_name.clone());
                self.state.market_id = Some(ev.market_id.clone());
                self.state.station_type = Some(ev.station_type.clone());
                self.station_services = ev.station_services.clone();
            }
            /*
                Notes on tracking of a player's location.

                Body
                ---
                There are some caveats about tracking Body name, ID and type,
                mostly due to close-orbiting binary planets/moons.

                Presence on or near a Body is indicated in several scenarios:

                1. When the player logs in.
                2. When the player's location changes due to being docked
                  on a Fleet Carrier when it jumps.
                3. When the player flies within Orbital Cruise range of a
                  Body.

                For the first case this will always be a 'Location' event.
                If landed on a Body, or docked at a surface port then this
                will be indicated.  However, if docked at an orbital station
                the 'Body' is the name of that station, with 'BodyType' having
                'Station' as its value.

                In the second case although it *should* be a 'CarrierJump'
                event, for a while now it's actually been a 'Location' event.
                This should follow the same rules as being docked at an
                orbital station.

                For the last case there are some caveats to do with close
                orbiting binary bodies:

                1. 'ApproachBody' indicates presence near the Body in question.
                2. 'LeaveBody' indicates the player is no longer considered
                  to be near the Body.  This is specifically when no longer
                  in Orbital Cruise around the Body such that the HUD for that
                  has been switched out for the normal SuperCruise one.
                3. 'SupercruiseExit' does not indicate any change of presence
                  near a Body.
                4. 'SupercruiseEntry' *also* **DOES NOT** indicate that the
                  player is no longer near the Body.  They can easily utilise
                  Orbital Cruise to rapidly travel around the Body and then
                  land on it again **without a fresh 'ApproachBody'** event.

                  The only way to check for this is to utilise the Body (name)
                  present in `Status.json` schema, as this *will* correctly
                  reflect the second Body.
            */
            JournalEvent::Location(e) => {
                // Body fields
                self.state.body = Some(e.body.clone());
                self.state.body_id = Some(e.body_id);
                self.state.body_type = Some(e.body_type.clone());

                // Docking
                self.state.is_docked = e.docked;

                // System fields
                if e.star_pos.len() == 3 {
                    self.state.star_pos = Vec::from(e.star_pos);
                }

                self.state.system_address = Some(e.system_address);
                self.state.system_population = Some(e.population);

                self.state.system_name = Some(if e.star_system == "ProvingGround" {
                    "CQC".to_string()
                } else {
                    e.star_system.clone()
                });

                // Station fields
                self.state.station_name = e.station_name.clone();
                if e.body_type == "Station" {
                    self.state.station_name = Some(e.body.clone());
                }

                self.state.market_id = e.market_id.clone();
                self.state.station_type = e.station_type.clone();
                self.station_services.clone_from(&e.station_services);

                // Taxi / Dropship
                self.state.taxi = e.taxi.unwrap_or(false);
                if !self.state.taxi {
                    self.state.dropship = false;
                }
            }
            JournalEvent::CarrierJump(ev) => {
                if let Some(caps) = RE_FC_JUMPNAME.captures(&ev.station_name) {
                    let one = caps.get(1).unwrap().as_str();
                    let two = caps.get(2).unwrap().as_str();
                    ev.callsign = format!("{}-{}", one, two);
                }

                // Body fields
                self.state.body = Some(ev.body.clone());
                self.state.body_id = Some(ev.body_id.clone());
                self.state.body_type = Some(ev.body_type.clone());

                // System fields
                if ev.star_pos.len() == 3 {
                    self.state.star_pos = Vec::from(ev.star_pos);
                }

                self.state.system_address = Some(ev.system_address);
                self.state.system_population = Some(ev.population);

                self.state.system_name = Some(if ev.star_system == "ProvingGround" {
                    "CQC".to_string()
                } else {
                    ev.star_system.clone()
                });

                // Station reset
                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;
                self.station_services.clear();

                // Taxi / Dropship
                self.state.taxi = ev.taxi.unwrap_or(false);
                if !self.state.taxi {
                    self.state.dropship = false;
                }
            }
            JournalEvent::FSDJump(e) => {
                // Body cleared
                self.state.body = None;
                self.state.body_id = None;
                self.state.body_type = None;

                // System fields
                if e.star_pos.len() == 3 {
                    self.state.star_pos = Vec::from(e.star_pos);
                }

                self.state.system_address = Some(e.system_address);
                self.state.system_population = Some(e.population);

                self.state.system_name = Some(if e.star_system == "ProvingGround" {
                    "CQC".to_string()
                } else {
                    e.star_system.clone()
                });

                // Station reset
                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;
                self.station_services.clear();

                // Taxi / Dropship
                self.state.taxi = e.taxi.unwrap_or(false);
                if !self.state.taxi {
                    self.state.dropship = false;
                }
            }
            JournalEvent::ApproachBody(ev) => {
                self.state.body = Some(ev.body.clone());
                self.state.body_id = Some(ev.body_id.clone());
                self.state.body_type = Some("Planet".to_string());
            }

            JournalEvent::LeaveBody(_) => {
                self.state.body = None;
                self.state.body_id = None;
                self.state.body_type = None;
            }

            JournalEvent::SupercruiseEntry(_) => {
                if self.state.body_type.as_deref() == Some("Station") {
                    self.state.body = None;
                    self.state.body_id = None;
                    self.state.body_type = None;
                }

                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;
                self.station_services.clear();
            }

            JournalEvent::Music(ev) => {
                if ev.music_track == "MainMenu" {
                    self.state.body = None;
                    self.state.body_id = None;
                    self.state.body_type = None;
                }
            }

            JournalEvent::Rank(_) | JournalEvent::Promotion(_) => {
                let json = json!(event);
                for (key, value) in json.as_object().unwrap() {
                    if key == "event" || key == "timestamp" {
                        continue;
                    }
                    let score = value.as_i64().unwrap_or(0) as i32;
                    self.state.rank.insert(key.clone(), (score, 0));
                }
            }
            JournalEvent::Progress(_) => {
                let json = json!(event);
                for (key, value) in json.as_object().unwrap() {
                    if self.state.rank.contains_key(key) {
                        let (score, _) = self.state.rank.get(key).unwrap();
                        self.state.rank.insert(
                            key.clone(),
                            (*score, min(value.as_i64().unwrap() as i32, 100)),
                        );
                    }
                }
            }
            JournalEvent::EngineerProgress(ev) => {
                if self.valid_engineer_progress(ev) {
                    if !ev.engineers.is_empty() {
                        self.state.engineers.clear();
                        for e in &ev.engineers {
                            if let Some(rank) = e.rank {
                                self.state.engineers.insert(
                                    e.engineer.clone(),
                                    crate::state::EngineerProgress::Ranked(
                                        rank as i32,
                                        e.rank_progress.unwrap_or(0) as i32,
                                    ),
                                );
                            } else {
                                self.state.engineers.insert(
                                    e.engineer.clone(),
                                    crate::state::EngineerProgress::Progress(e.progress.clone()),
                                );
                            }
                        }
                    } else {
                        if let Some(engineer) = &ev.engineer {
                            if let Some(rank) = ev.rank {
                                self.state.engineers.insert(
                                    engineer.clone(),
                                    crate::state::EngineerProgress::Ranked(
                                        rank as i32,
                                        ev.rank_progress.unwrap_or(0) as i32,
                                    ),
                                );
                            } else {
                                self.state.engineers.insert(
                                    engineer.clone(),
                                    crate::state::EngineerProgress::Progress(
                                        ev.progress.clone().unwrap_or("".to_string()),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            JournalEvent::Cargo(ev) => if ev.vessel == "Ship" {},
            JournalEvent::CargoTransfer(ev) => {
                for e in &ev.transfers {
                    let name = canonicalise(&e.type_);
                    let val = self.state.cargo.get(&name).cloned().unwrap_or(0);
                    if e.direction == "toship" {
                        self.state.cargo.insert(name, val + e.count as i32);
                    } else {
                        self.state.cargo.insert(name, val - e.count as i32);
                    }
                }
            }
            JournalEvent::ShipLocker(_) => {
                //TODO: fill in
            }
            JournalEvent::Backpack(_) | JournalEvent::Resupply(_) => {}
            JournalEvent::BackpackChange(ev) => {
                for entry in &ev.added {
                    let cat = category(&entry.type_);
                    let name = canonicalise(&entry.name);

                    if let Some(map) = match cat.as_str() {
                        "Data" => Some(&mut self.state.backpack.data),
                        "Component" => Some(&mut self.state.backpack.component),
                        "Item" => Some(&mut self.state.backpack.item),
                        "Consumable" => Some(&mut self.state.backpack.consumable),
                        _ => None,
                    } {
                        let val = map.get(&name).cloned().unwrap_or(0);
                        map.insert(name, val + entry.count as i32);
                    }
                }
                for entry in &ev.removed {
                    let cat = category(&entry.type_);
                    let name = canonicalise(&entry.name);

                    if let Some(map) = match cat.as_str() {
                        "Data" => Some(&mut self.state.backpack.data),
                        "Component" => Some(&mut self.state.backpack.component),
                        "Item" => Some(&mut self.state.backpack.item),
                        "Consumable" => Some(&mut self.state.backpack.consumable),
                        _ => None,
                    } {
                        let val = map.get(&name).cloned().unwrap_or(0);
                        map.insert(name, val - entry.count as i32);
                    }
                }
            }
            JournalEvent::BuyMicroResources(ev) => {
                self.state.credits -= ev.price;
            }
            JournalEvent::SellMicroResources(ev) => {
                self.state.credits += ev.price;
            }
            JournalEvent::BookDropship(ev) => {
                self.state.credits -= ev.cost;
                self.state.dropship = true;
            }
            JournalEvent::BookTaxi(ev) => {
                self.state.credits -= ev.cost;
            }
            JournalEvent::CancelDropship(ev) => {
                self.state.credits += ev.refund;
                self.state.dropship = false;
                self.state.taxi = false;
            }
            JournalEvent::CancelTaxi(ev) => {
                self.state.credits += ev.refund;
                self.state.taxi = false;
            }
            JournalEvent::Market(_) => {
                if !self.replay {
                    return self.market_event.on_journal(event);
                }
            }
            JournalEvent::Shipyard(_) => {
                if !self.replay {
                    return self.shipyard_event.on_journal(event);
                }
            }
            JournalEvent::NavRoute(_) => {
                if !self.replay {
                    return self.navroute_event.on_journal(event);
                }
            }
            JournalEvent::Outfitting(_) => {
                if !self.replay {
                    return self.outfitting_event.on_journal(event);
                }
            }
            JournalEvent::FCMaterials(_) => {
                if !self.replay {
                    return self.fcmaterials_event.on_journal(event);
                }
            }
            JournalEvent::ModuleInfo(_) => {
                if !self.replay {
                    return self.moduleinfo_event.on_journal(event);
                }
            }
            JournalEvent::CollectCargo(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity).or_insert(0) += 1;
            }
            JournalEvent::MarketBuy(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity).or_insert(0) += ev.count as i32;
                self.state.credits -= ev.total_cost;
            }
            JournalEvent::BuyDrones(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity).or_insert(0) += ev.count as i32;
                self.state.credits -= ev.total_cost;
            }
            JournalEvent::MiningRefined(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity).or_insert(0) += 1;
            }
            JournalEvent::EjectCargo(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity.clone()).or_insert(0) -= ev.count as i32;

                if self.state.cargo[&commodity] <= 0 {
                    self.state.cargo.remove(&commodity);
                }
            }
            JournalEvent::MarketSell(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity.clone()).or_insert(0) -= ev.count as i32;
                self.state.credits += ev.total_sale; // use the correct field from the event
                if self.state.cargo[&commodity] <= 0 {
                    self.state.cargo.remove(&commodity);
                }
            }
            JournalEvent::SellDrones(ev) => {
                let commodity = canonicalise(&ev.type_);
                *self.state.cargo.entry(commodity.clone()).or_insert(0) -= ev.count as i32;
                self.state.credits += ev.total_sale; // use the correct field from the event
                if self.state.cargo[&commodity] <= 0 {
                    self.state.cargo.remove(&commodity);
                }
            }
            JournalEvent::SearchAndRescue(ev) => {
                let commodity = canonicalise(&ev.name);
                *self.state.cargo.entry(commodity.clone()).or_insert(0) -= ev.count as i32;

                if self.state.cargo[&commodity] <= 0 {
                    self.state.cargo.remove(&commodity);
                }
            }

            JournalEvent::Materials(ev) => {
                for i in &ev.raw {
                    self.state.raw.insert(canonicalise(&i.name), i.count as i32);
                }
                for i in &ev.manufactured {
                    self.state
                        .manufactured
                        .insert(canonicalise(&i.name), i.count as i32);
                }
                for i in &ev.encoded {
                    self.state
                        .encoded
                        .insert(canonicalise(&i.name), i.count as i32);
                }
            }
            JournalEvent::MaterialCollected(ev) => {
                let commodity = canonicalise(&ev.name);
                match ev.category.as_str() {
                    "Raw" => *self.state.raw.entry(commodity).or_insert(0) += ev.count as i32,
                    "Manufactured" => {
                        *self.state.manufactured.entry(commodity).or_insert(0) += ev.count as i32
                    }
                    "Encoded" => {
                        *self.state.encoded.entry(commodity).or_insert(0) += ev.count as i32
                    }
                    _ => {}
                }
            }
            JournalEvent::MaterialDiscarded(ev) => {
                let commodity = canonicalise(&ev.name);
                match ev.category.as_str() {
                    "Raw" => *self.state.raw.entry(commodity).or_insert(0) -= ev.count as i32,
                    "Manufactured" => {
                        *self.state.manufactured.entry(commodity).or_insert(0) -= ev.count as i32
                    }
                    "Encoded" => {
                        *self.state.encoded.entry(commodity).or_insert(0) -= ev.count as i32
                    }
                    _ => {}
                }
            }
            JournalEvent::ScientificResearch(ev) => {
                let commodity = canonicalise(&ev.name);
                match ev.category.as_str() {
                    "Raw" => *self.state.raw.entry(commodity).or_insert(0) -= ev.count as i32,
                    "Manufactured" => {
                        *self.state.manufactured.entry(commodity).or_insert(0) -= ev.count as i32
                    }
                    "Encoded" => {
                        *self.state.encoded.entry(commodity).or_insert(0) -= ev.count as i32
                    }
                    _ => {}
                }
            }
            JournalEvent::Synthesis(ev) => {
                for i in &ev.materials {
                    let commodity = canonicalise(&i.name);
                    let remove_from = |map: &mut HashMap<String, i32>| {
                        if let Some(v) = map.get_mut(&commodity) {
                            *v -= i.count as i32;
                            if *v <= 0 {
                                map.remove(&commodity);
                            }
                        }
                    };

                    remove_from(&mut self.state.raw);
                    remove_from(&mut self.state.manufactured);
                    remove_from(&mut self.state.encoded);
                }
            }
            JournalEvent::MaterialTrade(ev) => {
                let cat = category(&ev.paid.material);
                let remove_from = |map: &mut HashMap<String, i32>| {
                    let entry = map.entry(ev.paid.material.clone()).or_insert(0);
                    *entry -= ev.paid.quantity as i32;
                    if *entry <= 0 {
                        map.remove(&ev.paid.material);
                    }
                };
                match cat.as_str() {
                    "Raw" => remove_from(&mut self.state.raw),
                    "Manufactured" => remove_from(&mut self.state.manufactured),
                    "Encoded" => remove_from(&mut self.state.encoded),
                    _ => {}
                }
                let cat = category(&ev.received.material);
                let add_to = |map: &mut HashMap<String, i32>| {
                    let entry = map.entry(ev.received.material.clone()).or_insert(0);
                    *entry += ev.received.quantity as i32;
                };
                match cat.as_str() {
                    "Raw" => add_to(&mut self.state.raw),
                    "Manufactured" => add_to(&mut self.state.manufactured),
                    "Encoded" => add_to(&mut self.state.encoded),
                    _ => {}
                }
            }
            JournalEvent::EngineerCraft(ev) => {
                for i in &ev.ingredients {
                    let commodity = canonicalise(&i.name);
                    let remove_from = |map: &mut HashMap<String, i32>| {
                        if let Some(v) = map.get_mut(&commodity) {
                            *v -= i.count as i32;
                            if *v <= 0 {
                                map.remove(&commodity);
                            }
                        }
                    };

                    remove_from(&mut self.state.raw);
                    remove_from(&mut self.state.manufactured);
                    remove_from(&mut self.state.encoded);
                }

                if let Some(module) = self.state.modules.get_mut(&ev.slot) {
                    if module.item != canonicalise(&ev.module) {
                        log::warn!("module is not the crafted module");
                        return Some(event);
                    }

                    let mut mods = Vec::new();
                    for m in &ev.modifiers {
                        mods.push(ModuleModifier::from(m));
                    }
                    module.engineering = Some(ModuleEngineering {
                        blueprint_id: ev.blueprint_id,
                        blueprint_name: ev.blueprint_name.clone(),
                        engineer: ev.engineer.clone(),
                        engineer_id: ev.engineer_id,
                        experimental_effect: ev.experimental_effect.clone(),
                        experimental_effect_localised: ev.experimental_effect_localised.clone(),
                        level: ev.level,
                        modifiers: mods,
                        quality: ev.quality,
                    });
                }
            }
            JournalEvent::MissionCompleted(ev) => {
                self.state.credits += ev.reward.unwrap_or(0);
                for r in &ev.commodity_reward {
                    let commodity = canonicalise(&r.name);
                    *(self.state.cargo.entry(commodity).or_insert(0)) += r.count as i32;
                }
                for r in &ev.materials_reward {
                    let category = category(&r.category);
                    let material = canonicalise(&r.name);

                    match category.as_str() {
                        "Elements" | "Raw" => {
                            *self.state.raw.entry(material).or_insert(0) += r.count as i32;
                        }
                        "Encoded" => {
                            *self.state.encoded.entry(material).or_insert(0) += r.count as i32;
                        }
                        "Manufactured" => {
                            *self.state.manufactured.entry(material).or_insert(0) += r.count as i32;
                        }
                        _ => {}
                    }
                }
            }
            JournalEvent::EngineerContribution(ev) => {
                if let Some(c) = ev.commodity.as_ref() {
                    let c = canonicalise(c);
                    if let Some(val) = self.state.cargo.get_mut(&c) {
                        *val -= ev.quantity as i32;
                        if *val <= 0 {
                            self.state.cargo.remove(&c);
                        }
                    }
                }

                if let Some(m) = ev.material.as_ref() {
                    let m = canonicalise(m);
                    let remove_from = |map: &mut HashMap<String, i32>| {
                        if let Some(val) = map.get_mut(&m) {
                            *val -= ev.quantity as i32;
                            if *val <= 0 {
                                map.remove(&m);
                            }
                        }
                    };
                    remove_from(&mut self.state.raw);
                    remove_from(&mut self.state.manufactured);
                    remove_from(&mut self.state.encoded);
                }
            }

            JournalEvent::TechnologyBroker(ev) => {
                for i in &ev.commodities {
                    let c = canonicalise(&i.name);
                    if let Some(val) = self.state.cargo.get_mut(&c) {
                        *val -= i.count as i32;
                        if *val <= 0 {
                            self.state.cargo.remove(&c);
                        }
                    }
                }

                for i in &ev.materials {
                    let m = canonicalise(&i.name);
                    let map = match i.category.as_str() {
                        "Raw" => &mut self.state.raw,
                        "Manufactured" => &mut self.state.manufactured,
                        "Encoded" => &mut self.state.encoded,
                        _ => continue,
                    };
                    if let Some(val) = map.get_mut(&m) {
                        *val -= i.count as i32;
                        if *val <= 0 {
                            map.remove(&m);
                        }
                    }
                }
            }

            JournalEvent::JoinACrew(ev) => {
                self.state.captain = Some(ev.captain.clone());
                self.state.role = Some("Idle".to_string());
                self.state.star_pos.clear();
                self.state.system_name = None;
                self.state.system_address = None;
                self.state.system_population = None;
                self.state.body = None;
                self.state.body_id = None;
                self.state.body_type = None;
                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;
                self.station_services.clear();
                self.state.on_foot = false;
            }

            JournalEvent::ChangeCrewRole(ev) => {
                self.state.role = Some(ev.role.clone());
            }

            JournalEvent::QuitACrew(_) => {
                self.state.captain = None;
                self.state.role = None;
                self.state.system_name = None;
                self.state.system_address = None;
                self.state.system_population = None;
                self.state.star_pos.clear();
                self.state.body = None;
                self.state.body_id = None;
                self.state.body_type = None;
                self.state.station_name = None;
                self.state.market_id = None;
                self.state.station_type = None;
                self.station_services.clear();
            }

            JournalEvent::Friends(ev) => match ev.status.as_str() {
                "Online" | "Added" => {
                    self.state.friends.insert(ev.name.clone());
                }
                _ => {
                    self.state.friends.remove(&ev.name);
                }
            },

            JournalEvent::SellExplorationData(ev) => self.state.credits += ev.total_earnings,
            JournalEvent::BuyExplorationData(ev) => self.state.credits -= ev.cost,
            JournalEvent::BuyTradeData(ev) => self.state.credits -= ev.cost,
            JournalEvent::BuyAmmo(ev) => self.state.credits -= ev.cost,
            JournalEvent::CrewHire(ev) => self.state.credits -= ev.cost,
            JournalEvent::FetchRemoteModule(ev) => self.state.credits -= ev.transfer_cost,
            JournalEvent::RefuelAll(ev) => self.state.credits -= ev.cost,
            JournalEvent::Repair(ev) => self.state.credits -= ev.cost,
            JournalEvent::RestockVehicle(ev) => self.state.credits -= ev.cost,
            JournalEvent::CarrierBuy(ev) => self.state.credits -= ev.price,
            JournalEvent::PowerplayFastTrack(ev) => self.state.credits -= ev.cost,
            JournalEvent::Resurrect(ev) => self.state.credits -= ev.cost,
            JournalEvent::CommunityGoalReward(ev) => self.state.credits += ev.reward,
            JournalEvent::RedeemVoucher(ev) => self.state.credits += ev.amount,
            JournalEvent::SellShipOnRebuy(ev) => self.state.credits += ev.ship_price,
            JournalEvent::PowerplaySalary(ev) => self.state.credits += ev.amount,

            JournalEvent::ShipyardTransfer(ev) => self.state.credits -= ev.transfer_price,
            JournalEvent::PayBounties(ev) => self.state.credits -= ev.amount,

            JournalEvent::Powerplay(ev) => {
                self.state.power_play.power = Some(ev.power.clone());
                self.state.power_play.rank = Some(ev.rank as i32);
                self.state.power_play.merits = Some(ev.merits);
                self.state.power_play.votes = ev.votes;
                self.state.power_play.time_pledged = Some(ev.time_pledged.clone());
            }
            _ => {}
        }

        Some(event)
    }

    fn valid_engineer_progress(&self, ev: &EngineerProgress) -> bool {
        let engineers_present = ev.engineers.is_empty();
        let progress_present = ev.progress.is_some();

        // must have one of Engineers / Progress
        if !(engineers_present || progress_present) {
            log::warn!("EngineerProgress missing both 'Engineers' and 'Progress'");
            return false;
        }

        // cannot have both
        if engineers_present && progress_present {
            log::warn!("EngineerProgress has BOTH 'Engineers' and 'Progress'");
            return false;
        }

        if ev.engineers.is_empty() {
            log::warn!("EngineerProgress 'Engineers' list is empty");
            return false;
        }

        for eng in &ev.engineers {
            let invited_or_known = matches!(eng.progress.as_str(), "Invited" | "Known");

            let missing_engineer = eng.engineer.is_empty();
            let missing_engineerid = eng.engineer_id == 0;
            let missing_progress = eng.progress.is_empty();
            let mut missing_rank = eng.rank.is_none();
            let mut missing_rankprog = eng.rank_progress.is_none();

            if invited_or_known {
                missing_rank = false;
                missing_rankprog = false;
            }

            if missing_engineer {
                log::warn!("Engineer entry missing 'Engineer'");
                return false;
            }
            if missing_engineerid {
                log::warn!("Engineer entry missing 'EngineerID'");
                return false;
            }
            if missing_progress {
                log::warn!("Engineer entry missing 'Progress'");
                return false;
            }
            if missing_rank {
                log::warn!("Engineer entry missing 'Rank'");
                return false;
            }
            if missing_rankprog {
                log::warn!("Engineer entry missing 'RankProgress'");
                return false;
            }
        }

        // ---------- Progress version (single engineer) ----------
        if let Some(progress) = &ev.progress {
            let invited_or_known = progress == "Invited" || progress == "Known";

            let missing_engineer = ev.engineer.is_none();
            let missing_engineerid = ev.engineer_id.is_none();
            let mut missing_rank = ev.rank.is_none();
            let mut missing_rankprog = ev.rank_progress.is_none();

            if invited_or_known {
                missing_rank = false;
                missing_rankprog = false;
            }

            if missing_engineer {
                log::warn!("Progress event missing 'Engineer'");
                return false;
            }
            if missing_engineerid {
                log::warn!("Progress event missing 'EngineerID'");
                return false;
            }
            if missing_rank {
                log::warn!("Progress event missing 'Rank'");
                return false;
            }
            if missing_rankprog {
                log::warn!("Progress event missing 'RankProgress'");
                return false;
            }
        }

        true
    }
    fn populate_version_info(&mut self, event: &JournalEvent) {
        match event {
            JournalEvent::Fileheader(ev) => {
                self.state.game_language = Some(ev.language.clone());
                self.state.game_version = Some(ev.gameversion.clone());
                self.state.game_build = Some(ev.build.clone());
            }
            JournalEvent::LoadGame(ev) => {
                if let Some(lang) = &ev.language {
                    self.state.game_language = Some(lang.clone());
                }
                if let Some(version) = &ev.gameversion {
                    self.state.game_version = Some(version.clone());
                }
                if let Some(build) = &ev.build {
                    self.state.game_build = Some(build.clone());
                }
            }
            _ => {}
        };
        if let Some(version) = &self.state.game_version {
            self.version = version.clone();
            self.semver = match coerce_semver(version.as_str()) {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            self.is_beta = version.contains("alpha") || version.contains("beta");
        }
    }
}
