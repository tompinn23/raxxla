#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize, Serializer};

pub mod approach_body;
pub use approach_body::ApproachBody;

pub mod approach_settlement;
pub use approach_settlement::ApproachSettlement;

pub mod asteroid_cracked;
pub use asteroid_cracked::AsteroidCracked;

pub mod backpack;
pub use backpack::Backpack;

pub mod backpack_change;
pub use backpack_change::BackpackChange;

pub mod book_dropship;
pub use book_dropship::BookDropship;

pub mod book_taxi;
pub use book_taxi::BookTaxi;

pub mod bounty;
pub use bounty::Bounty;

pub mod buy_ammo;
pub use buy_ammo::BuyAmmo;

pub mod buy_drones;
pub use buy_drones::BuyDrones;

pub mod buy_exploration_data;
pub use buy_exploration_data::BuyExplorationData;

pub mod buy_micro_resources;
pub use buy_micro_resources::BuyMicroResources;

pub mod buy_suit;
pub use buy_suit::BuySuit;

pub mod buy_trade_data;
pub use buy_trade_data::BuyTradeData;

pub mod buy_weapon;
pub use buy_weapon::BuyWeapon;

pub mod cancelled_squadron_application;
pub use cancelled_squadron_application::CancelledSquadronApplication;

pub mod cancel_dropship;
pub use cancel_dropship::CancelDropship;

pub mod cancel_taxi;
pub use cancel_taxi::CancelTaxi;

pub mod cap_ship_bond;
pub use cap_ship_bond::CapShipBond;

pub mod cargo;
pub use cargo::Cargo;

pub mod cargo_depot;
pub use cargo_depot::CargoDepot;

pub mod cargo_transfer;
pub use cargo_transfer::CargoTransfer;

pub mod carrier_bank_transfer;
pub use carrier_bank_transfer::CarrierBankTransfer;

pub mod carrier_buy;
pub use carrier_buy::CarrierBuy;

pub mod carrier_cancel_decommission;
pub use carrier_cancel_decommission::CarrierCancelDecommission;

pub mod carrier_crew_services;
pub use carrier_crew_services::CarrierCrewServices;

pub mod carrier_decommission;
pub use carrier_decommission::CarrierDecommission;

pub mod carrier_deposit_fuel;
pub use carrier_deposit_fuel::CarrierDepositFuel;

pub mod carrier_docking_permission;
pub use carrier_docking_permission::CarrierDockingPermission;

pub mod carrier_finance;
pub use carrier_finance::CarrierFinance;

pub mod carrier_jump;
pub use carrier_jump::CarrierJump;

pub mod carrier_jump_cancelled;
pub use carrier_jump_cancelled::CarrierJumpCancelled;

pub mod carrier_jump_request;
pub use carrier_jump_request::CarrierJumpRequest;

pub mod carrier_location;
pub use carrier_location::CarrierLocation;

pub mod carrier_module_pack;
pub use carrier_module_pack::CarrierModulePack;

pub mod carrier_name_change;
pub use carrier_name_change::CarrierNameChange;

pub mod carrier_ship_pack;
pub use carrier_ship_pack::CarrierShipPack;

pub mod carrier_stats;
pub use carrier_stats::CarrierStats;

pub mod carrier_trade_order;
pub use carrier_trade_order::CarrierTradeOrder;

pub mod change_crew_role;
pub use change_crew_role::ChangeCrewRole;

pub mod clear_impound;
pub use clear_impound::ClearImpound;

pub mod clear_saved_game;
pub use clear_saved_game::ClearSavedGame;

pub mod cockpit_breached;
pub use cockpit_breached::CockpitBreached;

pub mod codex_entry;
pub use codex_entry::CodexEntry;

pub mod collect_cargo;
pub use collect_cargo::CollectCargo;

pub mod collect_items;
pub use collect_items::CollectItems;

pub mod colonisation_beacon_deployed;
pub use colonisation_beacon_deployed::ColonisationBeaconDeployed;

pub mod colonisation_construction_depot;
pub use colonisation_construction_depot::ColonisationConstructionDepot;

pub mod colonisation_contribution;
pub use colonisation_contribution::ColonisationContribution;

pub mod colonisation_system_claim;
pub use colonisation_system_claim::ColonisationSystemClaim;

pub mod colonisation_system_claim_release;
pub use colonisation_system_claim_release::ColonisationSystemClaimRelease;

pub mod commander;
pub use commander::Commander;

pub mod commit_crime;
pub use commit_crime::CommitCrime;

pub mod community_goal;
pub use community_goal::CommunityGoal;

pub mod community_goal_discard;
pub use community_goal_discard::CommunityGoalDiscard;

pub mod community_goal_join;
pub use community_goal_join::CommunityGoalJoin;

pub mod community_goal_reward;
pub use community_goal_reward::CommunityGoalReward;

pub mod complete_construction;
pub use complete_construction::CompleteConstruction;

pub mod continued;
pub use continued::Continued;

pub mod create_suit_loadout;
pub use create_suit_loadout::CreateSuitLoadout;

pub mod crew_assign;
pub use crew_assign::CrewAssign;

pub mod crew_fire;
pub use crew_fire::CrewFire;

pub mod crew_hire;
pub use crew_hire::CrewHire;

pub mod crew_launch_fighter;
pub use crew_launch_fighter::CrewLaunchFighter;

pub mod crew_member_joins;
pub use crew_member_joins::CrewMemberJoins;

pub mod crew_member_quits;
pub use crew_member_quits::CrewMemberQuits;

pub mod crew_member_role_change;
pub use crew_member_role_change::CrewMemberRoleChange;

pub mod crime_victim;
pub use crime_victim::CrimeVictim;

pub mod datalink_scan;
pub use datalink_scan::DatalinkScan;

pub mod datalink_voucher;
pub use datalink_voucher::DatalinkVoucher;

pub mod data_scanned;
pub use data_scanned::DataScanned;

pub mod delete_suit_loadout;
pub use delete_suit_loadout::DeleteSuitLoadout;

pub mod deliver_power_micro_resources;
pub use deliver_power_micro_resources::DeliverPowerMicroResources;

pub mod died;
pub use died::Died;

pub mod disbanded_squadron;
pub use disbanded_squadron::DisbandedSquadron;

pub mod discovery_scan;
pub use discovery_scan::DiscoveryScan;

pub mod disembark;
pub use disembark::Disembark;

pub mod docked;
pub use docked::Docked;

pub mod docking_cancelled;
pub use docking_cancelled::DockingCancelled;

pub mod docking_denied;
pub use docking_denied::DockingDenied;

pub mod docking_granted;
pub use docking_granted::DockingGranted;

pub mod docking_requested;
pub use docking_requested::DockingRequested;

pub mod docking_timeout;
pub use docking_timeout::DockingTimeout;

pub mod dock_fighter;
pub use dock_fighter::DockFighter;

pub mod dock_srv;
pub use dock_srv::DockSRV;

pub mod dropship_deploy;
pub use dropship_deploy::DropshipDeploy;

pub mod drop_items;
pub use drop_items::DropItems;

pub mod eject_cargo;
pub use eject_cargo::EjectCargo;

pub mod embark;
pub use embark::Embark;

pub mod end_crew_session;
pub use end_crew_session::EndCrewSession;

pub mod engineer_contribution;
pub use engineer_contribution::EngineerContribution;

pub mod engineer_craft;
pub use engineer_craft::EngineerCraft;

pub mod engineer_progress;
pub use engineer_progress::EngineerProgress;

pub mod escape_interdiction;
pub use escape_interdiction::EscapeInterdiction;

pub mod faction_kill_bond;
pub use faction_kill_bond::FactionKillBond;

pub mod fc_materials;
pub use fc_materials::FCMaterials;

pub mod fetch_remote_module;
pub use fetch_remote_module::FetchRemoteModule;

pub mod fighter_destroyed;
pub use fighter_destroyed::FighterDestroyed;

pub mod fighter_rebuilt;
pub use fighter_rebuilt::FighterRebuilt;

pub mod fileheader;
pub use fileheader::Fileheader;

pub mod friends;
pub use friends::Friends;

pub mod fsd_jump;
pub use fsd_jump::FSDJump;

pub mod fsd_target;
pub use fsd_target::FSDTarget;

pub mod fss_all_bodies_found;
pub use fss_all_bodies_found::FSSAllBodiesFound;

pub mod fss_body_signals;
pub use fss_body_signals::FSSBodySignals;

pub mod fss_discovery_scan;
pub use fss_discovery_scan::FSSDiscoveryScan;

pub mod fss_signal_discovered;
pub use fss_signal_discovered::FSSSignalDiscovered;

pub mod fuel_scoop;
pub use fuel_scoop::FuelScoop;

pub mod heat_damage;
pub use heat_damage::HeatDamage;

pub mod heat_warning;
pub use heat_warning::HeatWarning;

pub mod holoscreen_hacked;
pub use holoscreen_hacked::HoloscreenHacked;

pub mod hull_damage;
pub use hull_damage::HullDamage;

pub mod interdicted;
pub use interdicted::Interdicted;

pub mod interdiction;
pub use interdiction::Interdiction;

pub mod invited_to_squadron;
pub use invited_to_squadron::InvitedToSquadron;

pub mod jet_cone_boost;
pub use jet_cone_boost::JetConeBoost;

pub mod jet_cone_damage;
pub use jet_cone_damage::JetConeDamage;

pub mod joined_squadron;
pub use joined_squadron::JoinedSquadron;

pub mod join_a_crew;
pub use join_a_crew::JoinACrew;

pub mod kicked_from_squadron;
pub use kicked_from_squadron::KickedFromSquadron;

pub mod kick_crew_member;
pub use kick_crew_member::KickCrewMember;

pub mod launch_drone;
pub use launch_drone::LaunchDrone;

pub mod launch_fighter;
pub use launch_fighter::LaunchFighter;

pub mod launch_srv;
pub use launch_srv::LaunchSRV;

pub mod leave_body;
pub use leave_body::LeaveBody;

pub mod left_squadron;
pub use left_squadron::LeftSquadron;

pub mod liftoff;
pub use liftoff::Liftoff;

pub mod loadout;
pub use loadout::Loadout;

pub mod loadout_equip_module;
pub use loadout_equip_module::LoadoutEquipModule;

pub mod loadout_remove_module;
pub use loadout_remove_module::LoadoutRemoveModule;

pub mod load_game;
pub use load_game::LoadGame;

pub mod location;
pub use location::Location;

pub mod market;
pub use market::Market;

pub mod market_buy;
pub use market_buy::MarketBuy;

pub mod market_sell;
pub use market_sell::MarketSell;

pub mod mass_module_store;
pub use mass_module_store::MassModuleStore;

pub mod materials;
pub use materials::Materials;

pub mod material_collected;
pub use material_collected::MaterialCollected;

pub mod material_discarded;
pub use material_discarded::MaterialDiscarded;

pub mod material_discovered;
pub use material_discovered::MaterialDiscovered;

pub mod material_trade;
pub use material_trade::MaterialTrade;

pub mod mining_refined;
pub use mining_refined::MiningRefined;

pub mod missions;
pub use missions::Missions;

pub mod mission_abandoned;
pub use mission_abandoned::MissionAbandoned;

pub mod mission_accepted;
pub use mission_accepted::MissionAccepted;

pub mod mission_completed;
pub use mission_completed::MissionCompleted;

pub mod mission_failed;
pub use mission_failed::MissionFailed;

pub mod mission_redirected;
pub use mission_redirected::MissionRedirected;

pub mod module_buy;
pub use module_buy::ModuleBuy;

pub mod module_buy_and_store;
pub use module_buy_and_store::ModuleBuyAndStore;

pub mod module_info;
pub use module_info::ModuleInfo;

pub mod module_retrieve;
pub use module_retrieve::ModuleRetrieve;

pub mod module_sell;
pub use module_sell::ModuleSell;

pub mod module_sell_remote;
pub use module_sell_remote::ModuleSellRemote;

pub mod module_store;
pub use module_store::ModuleStore;

pub mod module_swap;
pub use module_swap::ModuleSwap;

pub mod multi_sell_exploration_data;
pub use multi_sell_exploration_data::MultiSellExplorationData;

pub mod music;
pub use music::Music;

pub mod nav_beacon_scan;
pub use nav_beacon_scan::NavBeaconScan;

pub mod nav_route;
pub use nav_route::NavRoute;

pub mod nav_route_clear;
pub use nav_route_clear::NavRouteClear;

pub mod new_commander;
pub use new_commander::NewCommander;

pub mod npc_crew_paid_wage;
pub use npc_crew_paid_wage::NpcCrewPaidWage;

pub mod npc_crew_rank;
pub use npc_crew_rank::NpcCrewRank;

pub mod outfitting;
pub use outfitting::Outfitting;

pub mod passengers;
pub use passengers::Passengers;

pub mod pay_bounties;
pub use pay_bounties::PayBounties;

pub mod pay_fines;
pub use pay_fines::PayFines;

pub mod powerplay;
pub use powerplay::Powerplay;

pub mod powerplay_collect;
pub use powerplay_collect::PowerplayCollect;

pub mod powerplay_defect;
pub use powerplay_defect::PowerplayDefect;

pub mod powerplay_deliver;
pub use powerplay_deliver::PowerplayDeliver;

pub mod powerplay_fast_track;
pub use powerplay_fast_track::PowerplayFastTrack;

pub mod powerplay_join;
pub use powerplay_join::PowerplayJoin;

pub mod powerplay_leave;
pub use powerplay_leave::PowerplayLeave;

pub mod powerplay_merits;
pub use powerplay_merits::PowerplayMerits;

pub mod powerplay_rank;
pub use powerplay_rank::PowerplayRank;

pub mod powerplay_salary;
pub use powerplay_salary::PowerplaySalary;

pub mod powerplay_vote;
pub use powerplay_vote::PowerplayVote;

pub mod powerplay_voucher;
pub use powerplay_voucher::PowerplayVoucher;

pub mod progress;
pub use progress::Progress;

pub mod promotion;
pub use promotion::Promotion;

pub mod prospected_asteroid;
pub use prospected_asteroid::ProspectedAsteroid;

pub mod pvp_kill;
pub use pvp_kill::PVPKill;

pub mod quit_a_crew;
pub use quit_a_crew::QuitACrew;

pub mod rank;
pub use rank::Rank;

pub mod reboot_repair;
pub use reboot_repair::RebootRepair;

pub mod receive_text;
pub use receive_text::ReceiveText;

pub mod redeem_voucher;
pub use redeem_voucher::RedeemVoucher;

pub mod refuel_all;
pub use refuel_all::RefuelAll;

pub mod refuel_partial;
pub use refuel_partial::RefuelPartial;

pub mod rename_suit_loadout;
pub use rename_suit_loadout::RenameSuitLoadout;

pub mod repair;
pub use repair::Repair;

pub mod repair_all;
pub use repair_all::RepairAll;

pub mod repair_drone;
pub use repair_drone::RepairDrone;

pub mod reputation;
pub use reputation::Reputation;

pub mod request_power_micro_resources;
pub use request_power_micro_resources::RequestPowerMicroResources;

pub mod reservoir_replenished;
pub use reservoir_replenished::ReservoirReplenished;

pub mod restock_vehicle;
pub use restock_vehicle::RestockVehicle;

pub mod resupply;
pub use resupply::Resupply;

pub mod resurrect;
pub use resurrect::Resurrect;

pub mod saa_scan_complete;
pub use saa_scan_complete::SAAScanComplete;

pub mod saa_signals_found;
pub use saa_signals_found::SAASignalsFound;

pub mod scan;
pub use scan::Scan;

pub mod scanned;
pub use scanned::Scanned;

pub mod scan_bary_centre;
pub use scan_bary_centre::ScanBaryCentre;

pub mod scan_organic;
pub use scan_organic::ScanOrganic;

pub mod scientific_research;
pub use scientific_research::ScientificResearch;

pub mod screenshot;
pub use screenshot::Screenshot;

pub mod search_and_rescue;
pub use search_and_rescue::SearchAndRescue;

pub mod self_destruct;
pub use self_destruct::SelfDestruct;

pub mod sell_drones;
pub use sell_drones::SellDrones;

pub mod sell_exploration_data;
pub use sell_exploration_data::SellExplorationData;

pub mod sell_micro_resources;
pub use sell_micro_resources::SellMicroResources;

pub mod sell_organic_data;
pub use sell_organic_data::SellOrganicData;

pub mod sell_ship_on_rebuy;
pub use sell_ship_on_rebuy::SellShipOnRebuy;

pub mod sell_suit;
pub use sell_suit::SellSuit;

pub mod sell_weapon;
pub use sell_weapon::SellWeapon;

pub mod send_text;
pub use send_text::SendText;

pub mod set_user_ship_name;
pub use set_user_ship_name::SetUserShipName;

pub mod shared_bookmark_to_squadron;
pub use shared_bookmark_to_squadron::SharedBookmarkToSquadron;

pub mod shield_state;
pub use shield_state::ShieldState;

pub mod shipyard;
pub use shipyard::Shipyard;

pub mod shipyard_bank_deposit;
pub use shipyard_bank_deposit::ShipyardBankDeposit;

pub mod shipyard_buy;
pub use shipyard_buy::ShipyardBuy;

pub mod shipyard_new;
pub use shipyard_new::ShipyardNew;

pub mod shipyard_redeem;
pub use shipyard_redeem::ShipyardRedeem;

pub mod shipyard_sell;
pub use shipyard_sell::ShipyardSell;

pub mod shipyard_swap;
pub use shipyard_swap::ShipyardSwap;

pub mod shipyard_transfer;
pub use shipyard_transfer::ShipyardTransfer;

pub mod ship_locker;
pub use ship_locker::ShipLocker;

pub mod ship_locker_backpack;
pub use ship_locker_backpack::ShipLockerBackpack;

pub mod ship_locker_materials;
pub use ship_locker_materials::ShipLockerMaterials;

pub mod ship_redeemed;
pub use ship_redeemed::ShipRedeemed;

pub mod ship_targeted;
pub use ship_targeted::ShipTargeted;

pub mod shutdown;
pub use shutdown::Shutdown;

pub mod squadron_application_approved;
pub use squadron_application_approved::SquadronApplicationApproved;

pub mod squadron_application_rejected;
pub use squadron_application_rejected::SquadronApplicationRejected;

pub mod squadron_created;
pub use squadron_created::SquadronCreated;

pub mod squadron_demotion;
pub use squadron_demotion::SquadronDemotion;

pub mod squadron_promotion;
pub use squadron_promotion::SquadronPromotion;

pub mod squadron_startup;
pub use squadron_startup::SquadronStartup;

pub mod srv_destroyed;
pub use srv_destroyed::SRVDestroyed;

pub mod start_jump;
pub use start_jump::StartJump;

mod startup;
pub use startup::StartUp;

pub mod statistics;
pub use statistics::Statistics;

pub mod status;
pub use status::Status;

pub mod stored_modules;
pub use stored_modules::StoredModules;

pub mod stored_ships;
pub use stored_ships::StoredShips;

pub mod suit_loadout;
pub use suit_loadout::SuitLoadout;

pub mod supercruise_destination_drop;
pub use supercruise_destination_drop::SupercruiseDestinationDrop;

pub mod supercruise_entry;
pub use supercruise_entry::SupercruiseEntry;

pub mod supercruise_exit;
pub use supercruise_exit::SupercruiseExit;

pub mod switch_suit_loadout;
pub use switch_suit_loadout::SwitchSuitLoadout;

pub mod synthesis;
pub use synthesis::Synthesis;

pub mod systems_shutdown;
pub use systems_shutdown::SystemsShutdown;

pub mod technology_broker;
pub use technology_broker::TechnologyBroker;

pub mod touchdown;
pub use touchdown::Touchdown;

pub mod trade_micro_resources;
pub use trade_micro_resources::TradeMicroResources;

pub mod transfer_micro_resources;
pub use transfer_micro_resources::TransferMicroResources;

pub mod under_attack;
pub use under_attack::UnderAttack;

pub mod undocked;
pub use undocked::Undocked;

pub mod upgrade_suit;
pub use upgrade_suit::UpgradeSuit;

pub mod upgrade_weapon;
pub use upgrade_weapon::UpgradeWeapon;

pub mod use_consumable;
pub use use_consumable::UseConsumable;

pub mod uss_drop;
pub use uss_drop::USSDrop;

pub mod vehicle_switch;
pub use vehicle_switch::VehicleSwitch;

pub mod wing_add;
pub use wing_add::WingAdd;

pub mod wing_invite;
pub use wing_invite::WingInvite;

pub mod wing_join;
pub use wing_join::WingJoin;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum JournalEvent {
    ApproachBody(ApproachBody),
    ApproachSettlement(ApproachSettlement),
    AsteroidCracked(AsteroidCracked),
    Backpack(Backpack),
    BackpackChange(BackpackChange),
    BookDropship(BookDropship),
    BookTaxi(BookTaxi),
    Bounty(Bounty),
    BuyAmmo(BuyAmmo),
    BuyDrones(BuyDrones),
    BuyExplorationData(BuyExplorationData),
    BuyMicroResources(BuyMicroResources),
    BuySuit(BuySuit),
    BuyTradeData(BuyTradeData),
    BuyWeapon(BuyWeapon),
    CancelledSquadronApplication(CancelledSquadronApplication),
    CancelDropship(CancelDropship),
    CancelTaxi(CancelTaxi),
    CapShipBond(CapShipBond),
    Cargo(Cargo),
    CargoDepot(CargoDepot),
    CargoTransfer(CargoTransfer),
    CarrierBankTransfer(CarrierBankTransfer),
    CarrierBuy(CarrierBuy),
    CarrierCancelDecommission(CarrierCancelDecommission),
    CarrierCrewServices(CarrierCrewServices),
    CarrierDecommission(CarrierDecommission),
    CarrierDepositFuel(CarrierDepositFuel),
    CarrierDockingPermission(CarrierDockingPermission),
    CarrierFinance(CarrierFinance),
    CarrierJump(CarrierJump),
    CarrierJumpCancelled(CarrierJumpCancelled),
    CarrierJumpRequest(CarrierJumpRequest),
    CarrierLocation(CarrierLocation),
    CarrierModulePack(CarrierModulePack),
    CarrierNameChange(CarrierNameChange),
    CarrierShipPack(CarrierShipPack),
    CarrierStats(CarrierStats),
    CarrierTradeOrder(CarrierTradeOrder),
    ChangeCrewRole(ChangeCrewRole),
    ClearImpound(ClearImpound),
    ClearSavedGame(ClearSavedGame),
    CockpitBreached(CockpitBreached),
    CodexEntry(CodexEntry),
    CollectCargo(CollectCargo),
    CollectItems(CollectItems),
    ColonisationBeaconDeployed(ColonisationBeaconDeployed),
    ColonisationConstructionDepot(ColonisationConstructionDepot),
    ColonisationContribution(ColonisationContribution),
    ColonisationSystemClaim(ColonisationSystemClaim),
    ColonisationSystemClaimRelease(ColonisationSystemClaimRelease),
    Commander(Commander),
    CommitCrime(CommitCrime),
    CommunityGoal(CommunityGoal),
    CommunityGoalDiscard(CommunityGoalDiscard),
    CommunityGoalJoin(CommunityGoalJoin),
    CommunityGoalReward(CommunityGoalReward),
    CompleteConstruction(CompleteConstruction),
    Continued(Continued),
    CreateSuitLoadout(CreateSuitLoadout),
    CrewAssign(CrewAssign),
    CrewFire(CrewFire),
    CrewHire(CrewHire),
    CrewLaunchFighter(CrewLaunchFighter),
    CrewMemberJoins(CrewMemberJoins),
    CrewMemberQuits(CrewMemberQuits),
    CrewMemberRoleChange(CrewMemberRoleChange),
    CrimeVictim(CrimeVictim),
    DatalinkScan(DatalinkScan),
    DatalinkVoucher(DatalinkVoucher),
    DataScanned(DataScanned),
    DeleteSuitLoadout(DeleteSuitLoadout),
    DeliverPowerMicroResources(DeliverPowerMicroResources),
    Died(Died),
    DisbandedSquadron(DisbandedSquadron),
    DiscoveryScan(DiscoveryScan),
    Disembark(Disembark),
    Docked(Docked),
    DockingCancelled(DockingCancelled),
    DockingDenied(DockingDenied),
    DockingGranted(DockingGranted),
    DockingRequested(DockingRequested),
    DockingTimeout(DockingTimeout),
    DockFighter(DockFighter),
    DockSRV(DockSRV),
    DropshipDeploy(DropshipDeploy),
    DropItems(DropItems),
    EjectCargo(EjectCargo),
    Embark(Embark),
    EndCrewSession(EndCrewSession),
    EngineerContribution(EngineerContribution),
    EngineerCraft(EngineerCraft),
    EngineerProgress(EngineerProgress),
    EscapeInterdiction(EscapeInterdiction),
    FactionKillBond(FactionKillBond),
    FCMaterials(FCMaterials),
    FetchRemoteModule(FetchRemoteModule),
    FighterDestroyed(FighterDestroyed),
    FighterRebuilt(FighterRebuilt),
    Fileheader(Fileheader),
    Friends(Friends),
    FSDJump(FSDJump),
    FSDTarget(FSDTarget),
    FSSAllBodiesFound(FSSAllBodiesFound),
    FSSBodySignals(FSSBodySignals),
    FSSDiscoveryScan(FSSDiscoveryScan),
    FSSSignalDiscovered(FSSSignalDiscovered),
    FuelScoop(FuelScoop),
    HeatDamage(HeatDamage),
    HeatWarning(HeatWarning),
    HoloscreenHacked(HoloscreenHacked),
    HullDamage(HullDamage),
    Interdicted(Interdicted),
    Interdiction(Interdiction),
    InvitedToSquadron(InvitedToSquadron),
    JetConeBoost(JetConeBoost),
    JetConeDamage(JetConeDamage),
    JoinedSquadron(JoinedSquadron),
    JoinACrew(JoinACrew),
    KickedFromSquadron(KickedFromSquadron),
    KickCrewMember(KickCrewMember),
    LaunchDrone(LaunchDrone),
    LaunchFighter(LaunchFighter),
    LaunchSRV(LaunchSRV),
    LeaveBody(LeaveBody),
    LeftSquadron(LeftSquadron),
    Liftoff(Liftoff),
    Loadout(Loadout),
    LoadoutEquipModule(LoadoutEquipModule),
    LoadoutRemoveModule(LoadoutRemoveModule),
    LoadGame(LoadGame),
    Location(Location),
    Market(Market),
    MarketBuy(MarketBuy),
    MarketSell(MarketSell),
    MassModuleStore(MassModuleStore),
    Materials(Materials),
    MaterialCollected(MaterialCollected),
    MaterialDiscarded(MaterialDiscarded),
    MaterialDiscovered(MaterialDiscovered),
    MaterialTrade(MaterialTrade),
    MiningRefined(MiningRefined),
    Missions(Missions),
    MissionAbandoned(MissionAbandoned),
    MissionAccepted(MissionAccepted),
    MissionCompleted(MissionCompleted),
    MissionFailed(MissionFailed),
    MissionRedirected(MissionRedirected),
    ModuleBuy(ModuleBuy),
    ModuleBuyAndStore(ModuleBuyAndStore),
    ModuleInfo(ModuleInfo),
    ModuleRetrieve(ModuleRetrieve),
    ModuleSell(ModuleSell),
    ModuleSellRemote(ModuleSellRemote),
    ModuleStore(ModuleStore),
    ModuleSwap(ModuleSwap),
    MultiSellExplorationData(MultiSellExplorationData),
    Music(Music),
    NavBeaconScan(NavBeaconScan),
    NavRoute(NavRoute),
    NavRouteClear(NavRouteClear),
    NewCommander(NewCommander),
    NpcCrewPaidWage(NpcCrewPaidWage),
    NpcCrewRank(NpcCrewRank),
    Outfitting(Outfitting),
    Passengers(Passengers),
    PayBounties(PayBounties),
    PayFines(PayFines),
    Powerplay(Powerplay),
    PowerplayCollect(PowerplayCollect),
    PowerplayDefect(PowerplayDefect),
    PowerplayDeliver(PowerplayDeliver),
    PowerplayFastTrack(PowerplayFastTrack),
    PowerplayJoin(PowerplayJoin),
    PowerplayLeave(PowerplayLeave),
    PowerplayMerits(PowerplayMerits),
    PowerplayRank(PowerplayRank),
    PowerplaySalary(PowerplaySalary),
    PowerplayVote(PowerplayVote),
    PowerplayVoucher(PowerplayVoucher),
    Progress(Progress),
    Promotion(Promotion),
    ProspectedAsteroid(ProspectedAsteroid),
    PVPKill(PVPKill),
    QuitACrew(QuitACrew),
    Rank(Rank),
    RebootRepair(RebootRepair),
    ReceiveText(ReceiveText),
    RedeemVoucher(RedeemVoucher),
    RefuelAll(RefuelAll),
    RefuelPartial(RefuelPartial),
    RenameSuitLoadout(RenameSuitLoadout),
    Repair(Repair),
    RepairAll(RepairAll),
    RepairDrone(RepairDrone),
    Reputation(Reputation),
    RequestPowerMicroResources(RequestPowerMicroResources),
    ReservoirReplenished(ReservoirReplenished),
    RestockVehicle(RestockVehicle),
    Resupply(Resupply),
    Resurrect(Resurrect),
    SAAScanComplete(SAAScanComplete),
    SAASignalsFound(SAASignalsFound),
    Scan(Scan),
    Scanned(Scanned),
    ScanBaryCentre(ScanBaryCentre),
    ScanOrganic(ScanOrganic),
    ScientificResearch(ScientificResearch),
    Screenshot(Screenshot),
    SearchAndRescue(SearchAndRescue),
    SelfDestruct(SelfDestruct),
    SellDrones(SellDrones),
    SellExplorationData(SellExplorationData),
    SellMicroResources(SellMicroResources),
    SellOrganicData(SellOrganicData),
    SellShipOnRebuy(SellShipOnRebuy),
    SellSuit(SellSuit),
    SellWeapon(SellWeapon),
    SendText(SendText),
    SetUserShipName(SetUserShipName),
    SharedBookmarkToSquadron(SharedBookmarkToSquadron),
    ShieldState(ShieldState),
    Shipyard(Shipyard),
    ShipyardBankDeposit(ShipyardBankDeposit),
    ShipyardBuy(ShipyardBuy),
    ShipyardNew(ShipyardNew),
    ShipyardRedeem(ShipyardRedeem),
    ShipyardSell(ShipyardSell),
    ShipyardSwap(ShipyardSwap),
    ShipyardTransfer(ShipyardTransfer),
    ShipLocker(ShipLocker),
    ShipLockerBackpack(ShipLockerBackpack),
    ShipLockerMaterials(ShipLockerMaterials),
    ShipRedeemed(ShipRedeemed),
    ShipTargeted(ShipTargeted),
    Shutdown(Shutdown),
    SquadronApplicationApproved(SquadronApplicationApproved),
    SquadronApplicationRejected(SquadronApplicationRejected),
    SquadronCreated(SquadronCreated),
    SquadronDemotion(SquadronDemotion),
    SquadronPromotion(SquadronPromotion),
    SquadronStartup(SquadronStartup),
    SRVDestroyed(SRVDestroyed),
    StartUp(StartUp),
    StartJump(StartJump),
    Statistics(Statistics),
    Status(Status),
    StoredModules(StoredModules),
    StoredShips(StoredShips),
    SuitLoadout(SuitLoadout),
    SupercruiseDestinationDrop(SupercruiseDestinationDrop),
    SupercruiseEntry(SupercruiseEntry),
    SupercruiseExit(SupercruiseExit),
    SwitchSuitLoadout(SwitchSuitLoadout),
    Synthesis(Synthesis),
    SystemsShutdown(SystemsShutdown),
    TechnologyBroker(TechnologyBroker),
    Touchdown(Touchdown),
    TradeMicroResources(TradeMicroResources),
    TransferMicroResources(TransferMicroResources),
    UnderAttack(UnderAttack),
    Undocked(Undocked),
    UpgradeSuit(UpgradeSuit),
    UpgradeWeapon(UpgradeWeapon),
    UseConsumable(UseConsumable),
    USSDrop(USSDrop),
    VehicleSwitch(VehicleSwitch),
    WingAdd(WingAdd),
    WingInvite(WingInvite),
    WingJoin(WingJoin),
}

impl JournalEvent {
    pub fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        match self {
            JournalEvent::ApproachBody(inner) => inner.timestamp,
            JournalEvent::ApproachSettlement(inner) => inner.timestamp,
            JournalEvent::AsteroidCracked(inner) => inner.timestamp,
            JournalEvent::Backpack(inner) => inner.timestamp,
            JournalEvent::BackpackChange(inner) => inner.timestamp,
            JournalEvent::BookDropship(inner) => inner.timestamp,
            JournalEvent::BookTaxi(inner) => inner.timestamp,
            JournalEvent::Bounty(inner) => inner.timestamp,
            JournalEvent::BuyAmmo(inner) => inner.timestamp,
            JournalEvent::BuyDrones(inner) => inner.timestamp,
            JournalEvent::BuyExplorationData(inner) => inner.timestamp,
            JournalEvent::BuyMicroResources(inner) => inner.timestamp,
            JournalEvent::BuySuit(inner) => inner.timestamp,
            JournalEvent::BuyTradeData(inner) => inner.timestamp,
            JournalEvent::BuyWeapon(inner) => inner.timestamp,
            JournalEvent::CancelledSquadronApplication(inner) => inner.timestamp,
            JournalEvent::CancelDropship(inner) => inner.timestamp,
            JournalEvent::CancelTaxi(inner) => inner.timestamp,
            JournalEvent::CapShipBond(inner) => inner.timestamp,
            JournalEvent::Cargo(inner) => inner.timestamp,
            JournalEvent::CargoDepot(inner) => inner.timestamp,
            JournalEvent::CargoTransfer(inner) => inner.timestamp,
            JournalEvent::CarrierBankTransfer(inner) => inner.timestamp,
            JournalEvent::CarrierBuy(inner) => inner.timestamp,
            JournalEvent::CarrierCancelDecommission(inner) => inner.timestamp,
            JournalEvent::CarrierCrewServices(inner) => inner.timestamp,
            JournalEvent::CarrierDecommission(inner) => inner.timestamp,
            JournalEvent::CarrierDepositFuel(inner) => inner.timestamp,
            JournalEvent::CarrierDockingPermission(inner) => inner.timestamp,
            JournalEvent::CarrierFinance(inner) => inner.timestamp,
            JournalEvent::CarrierJump(inner) => inner.timestamp,
            JournalEvent::CarrierJumpCancelled(inner) => inner.timestamp,
            JournalEvent::CarrierJumpRequest(inner) => inner.timestamp,
            JournalEvent::CarrierLocation(inner) => inner.timestamp,
            JournalEvent::CarrierModulePack(inner) => inner.timestamp,
            JournalEvent::CarrierNameChange(inner) => inner.timestamp,
            JournalEvent::CarrierShipPack(inner) => inner.timestamp,
            JournalEvent::CarrierStats(inner) => inner.timestamp,
            JournalEvent::CarrierTradeOrder(inner) => inner.timestamp,
            JournalEvent::ChangeCrewRole(inner) => inner.timestamp,
            JournalEvent::ClearImpound(inner) => inner.timestamp,
            JournalEvent::ClearSavedGame(inner) => inner.timestamp,
            JournalEvent::CockpitBreached(inner) => inner.timestamp,
            JournalEvent::CodexEntry(inner) => inner.timestamp,
            JournalEvent::CollectCargo(inner) => inner.timestamp,
            JournalEvent::CollectItems(inner) => inner.timestamp,
            JournalEvent::ColonisationBeaconDeployed(inner) => inner.timestamp,
            JournalEvent::ColonisationConstructionDepot(inner) => inner.timestamp,
            JournalEvent::ColonisationContribution(inner) => inner.timestamp,
            JournalEvent::ColonisationSystemClaim(inner) => inner.timestamp,
            JournalEvent::ColonisationSystemClaimRelease(inner) => inner.timestamp,
            JournalEvent::Commander(inner) => inner.timestamp,
            JournalEvent::CommitCrime(inner) => inner.timestamp,
            JournalEvent::CommunityGoal(inner) => inner.timestamp,
            JournalEvent::CommunityGoalDiscard(inner) => inner.timestamp,
            JournalEvent::CommunityGoalJoin(inner) => inner.timestamp,
            JournalEvent::CommunityGoalReward(inner) => inner.timestamp,
            JournalEvent::CompleteConstruction(inner) => inner.timestamp,
            JournalEvent::Continued(inner) => inner.timestamp,
            JournalEvent::CreateSuitLoadout(inner) => inner.timestamp,
            JournalEvent::CrewAssign(inner) => inner.timestamp,
            JournalEvent::CrewFire(inner) => inner.timestamp,
            JournalEvent::CrewHire(inner) => inner.timestamp,
            JournalEvent::CrewLaunchFighter(inner) => inner.timestamp,
            JournalEvent::CrewMemberJoins(inner) => inner.timestamp,
            JournalEvent::CrewMemberQuits(inner) => inner.timestamp,
            JournalEvent::CrewMemberRoleChange(inner) => inner.timestamp,
            JournalEvent::CrimeVictim(inner) => inner.timestamp,
            JournalEvent::DatalinkScan(inner) => inner.timestamp,
            JournalEvent::DatalinkVoucher(inner) => inner.timestamp,
            JournalEvent::DataScanned(inner) => inner.timestamp,
            JournalEvent::DeleteSuitLoadout(inner) => inner.timestamp,
            JournalEvent::DeliverPowerMicroResources(inner) => inner.timestamp,
            JournalEvent::Died(inner) => inner.timestamp,
            JournalEvent::DisbandedSquadron(inner) => inner.timestamp,
            JournalEvent::DiscoveryScan(inner) => inner.timestamp,
            JournalEvent::Disembark(inner) => inner.timestamp,
            JournalEvent::Docked(inner) => inner.timestamp,
            JournalEvent::DockingCancelled(inner) => inner.timestamp,
            JournalEvent::DockingDenied(inner) => inner.timestamp,
            JournalEvent::DockingGranted(inner) => inner.timestamp,
            JournalEvent::DockingRequested(inner) => inner.timestamp,
            JournalEvent::DockingTimeout(inner) => inner.timestamp,
            JournalEvent::DockFighter(inner) => inner.timestamp,
            JournalEvent::DockSRV(inner) => inner.timestamp,
            JournalEvent::DropshipDeploy(inner) => inner.timestamp,
            JournalEvent::DropItems(inner) => inner.timestamp,
            JournalEvent::EjectCargo(inner) => inner.timestamp,
            JournalEvent::Embark(inner) => inner.timestamp,
            JournalEvent::EndCrewSession(inner) => inner.timestamp,
            JournalEvent::EngineerContribution(inner) => inner.timestamp,
            JournalEvent::EngineerCraft(inner) => inner.timestamp,
            JournalEvent::EngineerProgress(inner) => inner.timestamp,
            JournalEvent::EscapeInterdiction(inner) => inner.timestamp,
            JournalEvent::FactionKillBond(inner) => inner.timestamp,
            JournalEvent::FCMaterials(inner) => inner.timestamp,
            JournalEvent::FetchRemoteModule(inner) => inner.timestamp,
            JournalEvent::FighterDestroyed(inner) => inner.timestamp,
            JournalEvent::FighterRebuilt(inner) => inner.timestamp,
            JournalEvent::Fileheader(inner) => inner.timestamp,
            JournalEvent::Friends(inner) => inner.timestamp,
            JournalEvent::FSDJump(inner) => inner.timestamp,
            JournalEvent::FSDTarget(inner) => inner.timestamp,
            JournalEvent::FSSAllBodiesFound(inner) => inner.timestamp,
            JournalEvent::FSSBodySignals(inner) => inner.timestamp,
            JournalEvent::FSSDiscoveryScan(inner) => inner.timestamp,
            JournalEvent::FSSSignalDiscovered(inner) => inner.timestamp,
            JournalEvent::FuelScoop(inner) => inner.timestamp,
            JournalEvent::HeatDamage(inner) => inner.timestamp,
            JournalEvent::HeatWarning(inner) => inner.timestamp,
            JournalEvent::HoloscreenHacked(inner) => inner.timestamp,
            JournalEvent::HullDamage(inner) => inner.timestamp,
            JournalEvent::Interdicted(inner) => inner.timestamp,
            JournalEvent::Interdiction(inner) => inner.timestamp,
            JournalEvent::InvitedToSquadron(inner) => inner.timestamp,
            JournalEvent::JetConeBoost(inner) => inner.timestamp,
            JournalEvent::JetConeDamage(inner) => inner.timestamp,
            JournalEvent::JoinedSquadron(inner) => inner.timestamp,
            JournalEvent::JoinACrew(inner) => inner.timestamp,
            JournalEvent::KickedFromSquadron(inner) => inner.timestamp,
            JournalEvent::KickCrewMember(inner) => inner.timestamp,
            JournalEvent::LaunchDrone(inner) => inner.timestamp,
            JournalEvent::LaunchFighter(inner) => inner.timestamp,
            JournalEvent::LaunchSRV(inner) => inner.timestamp,
            JournalEvent::LeaveBody(inner) => inner.timestamp,
            JournalEvent::LeftSquadron(inner) => inner.timestamp,
            JournalEvent::Liftoff(inner) => inner.timestamp,
            JournalEvent::Loadout(inner) => inner.timestamp,
            JournalEvent::LoadoutEquipModule(inner) => inner.timestamp,
            JournalEvent::LoadoutRemoveModule(inner) => inner.timestamp,
            JournalEvent::LoadGame(inner) => inner.timestamp,
            JournalEvent::Location(inner) => inner.timestamp,
            JournalEvent::Market(inner) => inner.timestamp,
            JournalEvent::MarketBuy(inner) => inner.timestamp,
            JournalEvent::MarketSell(inner) => inner.timestamp,
            JournalEvent::MassModuleStore(inner) => inner.timestamp,
            JournalEvent::Materials(inner) => inner.timestamp,
            JournalEvent::MaterialCollected(inner) => inner.timestamp,
            JournalEvent::MaterialDiscarded(inner) => inner.timestamp,
            JournalEvent::MaterialDiscovered(inner) => inner.timestamp,
            JournalEvent::MaterialTrade(inner) => inner.timestamp,
            JournalEvent::MiningRefined(inner) => inner.timestamp,
            JournalEvent::Missions(inner) => inner.timestamp,
            JournalEvent::MissionAbandoned(inner) => inner.timestamp,
            JournalEvent::MissionAccepted(inner) => inner.timestamp,
            JournalEvent::MissionCompleted(inner) => inner.timestamp,
            JournalEvent::MissionFailed(inner) => inner.timestamp,
            JournalEvent::MissionRedirected(inner) => inner.timestamp,
            JournalEvent::ModuleBuy(inner) => inner.timestamp,
            JournalEvent::ModuleBuyAndStore(inner) => inner.timestamp,
            JournalEvent::ModuleInfo(inner) => inner.timestamp,
            JournalEvent::ModuleRetrieve(inner) => inner.timestamp,
            JournalEvent::ModuleSell(inner) => inner.timestamp,
            JournalEvent::ModuleSellRemote(inner) => inner.timestamp,
            JournalEvent::ModuleStore(inner) => inner.timestamp,
            JournalEvent::ModuleSwap(inner) => inner.timestamp,
            JournalEvent::MultiSellExplorationData(inner) => inner.timestamp,
            JournalEvent::Music(inner) => inner.timestamp,
            JournalEvent::NavBeaconScan(inner) => inner.timestamp,
            JournalEvent::NavRoute(inner) => inner.timestamp,
            JournalEvent::NavRouteClear(inner) => inner.timestamp,
            JournalEvent::NewCommander(inner) => inner.timestamp,
            JournalEvent::NpcCrewPaidWage(inner) => inner.timestamp,
            JournalEvent::NpcCrewRank(inner) => inner.timestamp,
            JournalEvent::Outfitting(inner) => inner.timestamp,
            JournalEvent::Passengers(inner) => inner.timestamp,
            JournalEvent::PayBounties(inner) => inner.timestamp,
            JournalEvent::PayFines(inner) => inner.timestamp,
            JournalEvent::Powerplay(inner) => inner.timestamp,
            JournalEvent::PowerplayCollect(inner) => inner.timestamp,
            JournalEvent::PowerplayDefect(inner) => inner.timestamp,
            JournalEvent::PowerplayDeliver(inner) => inner.timestamp,
            JournalEvent::PowerplayFastTrack(inner) => inner.timestamp,
            JournalEvent::PowerplayJoin(inner) => inner.timestamp,
            JournalEvent::PowerplayLeave(inner) => inner.timestamp,
            JournalEvent::PowerplayMerits(inner) => inner.timestamp,
            JournalEvent::PowerplayRank(inner) => inner.timestamp,
            JournalEvent::PowerplaySalary(inner) => inner.timestamp,
            JournalEvent::PowerplayVote(inner) => inner.timestamp,
            JournalEvent::PowerplayVoucher(inner) => inner.timestamp,
            JournalEvent::Progress(inner) => inner.timestamp,
            JournalEvent::Promotion(inner) => inner.timestamp,
            JournalEvent::ProspectedAsteroid(inner) => inner.timestamp,
            JournalEvent::PVPKill(inner) => inner.timestamp,
            JournalEvent::QuitACrew(inner) => inner.timestamp,
            JournalEvent::Rank(inner) => inner.timestamp,
            JournalEvent::RebootRepair(inner) => inner.timestamp,
            JournalEvent::ReceiveText(inner) => inner.timestamp,
            JournalEvent::RedeemVoucher(inner) => inner.timestamp,
            JournalEvent::RefuelAll(inner) => inner.timestamp,
            JournalEvent::RefuelPartial(inner) => inner.timestamp,
            JournalEvent::RenameSuitLoadout(inner) => inner.timestamp,
            JournalEvent::Repair(inner) => inner.timestamp,
            JournalEvent::RepairAll(inner) => inner.timestamp,
            JournalEvent::RepairDrone(inner) => inner.timestamp,
            JournalEvent::Reputation(inner) => inner.timestamp,
            JournalEvent::RequestPowerMicroResources(inner) => inner.timestamp,
            JournalEvent::ReservoirReplenished(inner) => inner.timestamp,
            JournalEvent::RestockVehicle(inner) => inner.timestamp,
            JournalEvent::Resupply(inner) => inner.timestamp,
            JournalEvent::Resurrect(inner) => inner.timestamp,
            JournalEvent::SAAScanComplete(inner) => inner.timestamp,
            JournalEvent::SAASignalsFound(inner) => inner.timestamp,
            JournalEvent::Scan(inner) => inner.timestamp,
            JournalEvent::Scanned(inner) => inner.timestamp,
            JournalEvent::ScanBaryCentre(inner) => inner.timestamp,
            JournalEvent::ScanOrganic(inner) => inner.timestamp,
            JournalEvent::ScientificResearch(inner) => inner.timestamp,
            JournalEvent::Screenshot(inner) => inner.timestamp,
            JournalEvent::SearchAndRescue(inner) => inner.timestamp,
            JournalEvent::SelfDestruct(inner) => inner.timestamp,
            JournalEvent::SellDrones(inner) => inner.timestamp,
            JournalEvent::SellExplorationData(inner) => inner.timestamp,
            JournalEvent::SellMicroResources(inner) => inner.timestamp,
            JournalEvent::SellOrganicData(inner) => inner.timestamp,
            JournalEvent::SellShipOnRebuy(inner) => inner.timestamp,
            JournalEvent::SellSuit(inner) => inner.timestamp,
            JournalEvent::SellWeapon(inner) => inner.timestamp,
            JournalEvent::SendText(inner) => inner.timestamp,
            JournalEvent::SetUserShipName(inner) => inner.timestamp,
            JournalEvent::SharedBookmarkToSquadron(inner) => inner.timestamp,
            JournalEvent::ShieldState(inner) => inner.timestamp,
            JournalEvent::Shipyard(inner) => inner.timestamp,
            JournalEvent::ShipyardBankDeposit(inner) => inner.timestamp,
            JournalEvent::ShipyardBuy(inner) => inner.timestamp,
            JournalEvent::ShipyardNew(inner) => inner.timestamp,
            JournalEvent::ShipyardRedeem(inner) => inner.timestamp,
            JournalEvent::ShipyardSell(inner) => inner.timestamp,
            JournalEvent::ShipyardSwap(inner) => inner.timestamp,
            JournalEvent::ShipyardTransfer(inner) => inner.timestamp,
            JournalEvent::ShipLocker(inner) => inner.timestamp,
            JournalEvent::ShipLockerBackpack(inner) => inner.timestamp,
            JournalEvent::ShipLockerMaterials(inner) => inner.timestamp,
            JournalEvent::ShipRedeemed(inner) => inner.timestamp,
            JournalEvent::ShipTargeted(inner) => inner.timestamp,
            JournalEvent::Shutdown(inner) => inner.timestamp,
            JournalEvent::SquadronApplicationApproved(inner) => inner.timestamp,
            JournalEvent::SquadronApplicationRejected(inner) => inner.timestamp,
            JournalEvent::SquadronCreated(inner) => inner.timestamp,
            JournalEvent::SquadronDemotion(inner) => inner.timestamp,
            JournalEvent::SquadronPromotion(inner) => inner.timestamp,
            JournalEvent::SquadronStartup(inner) => inner.timestamp,
            JournalEvent::SRVDestroyed(inner) => inner.timestamp,
            JournalEvent::StartUp(inner) => inner.timestamp,
            JournalEvent::StartJump(inner) => inner.timestamp,
            JournalEvent::Statistics(inner) => inner.timestamp,
            JournalEvent::Status(inner) => inner.timestamp,
            JournalEvent::StoredModules(inner) => inner.timestamp,
            JournalEvent::StoredShips(inner) => inner.timestamp,
            JournalEvent::SuitLoadout(inner) => inner.timestamp,
            JournalEvent::SupercruiseDestinationDrop(inner) => inner.timestamp,
            JournalEvent::SupercruiseEntry(inner) => inner.timestamp,
            JournalEvent::SupercruiseExit(inner) => inner.timestamp,
            JournalEvent::SwitchSuitLoadout(inner) => inner.timestamp,
            JournalEvent::Synthesis(inner) => inner.timestamp,
            JournalEvent::SystemsShutdown(inner) => inner.timestamp,
            JournalEvent::TechnologyBroker(inner) => inner.timestamp,
            JournalEvent::Touchdown(inner) => inner.timestamp,
            JournalEvent::TradeMicroResources(inner) => inner.timestamp,
            JournalEvent::TransferMicroResources(inner) => inner.timestamp,
            JournalEvent::UnderAttack(inner) => inner.timestamp,
            JournalEvent::Undocked(inner) => inner.timestamp,
            JournalEvent::UpgradeSuit(inner) => inner.timestamp,
            JournalEvent::UpgradeWeapon(inner) => inner.timestamp,
            JournalEvent::UseConsumable(inner) => inner.timestamp,
            JournalEvent::USSDrop(inner) => inner.timestamp,
            JournalEvent::VehicleSwitch(inner) => inner.timestamp,
            JournalEvent::WingAdd(inner) => inner.timestamp,
            JournalEvent::WingInvite(inner) => inner.timestamp,
            JournalEvent::WingJoin(inner) => inner.timestamp,
        }
    }
}

impl Serialize for JournalEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JournalEvent::ApproachBody(inner) => inner.serialize(serializer),
            JournalEvent::ApproachSettlement(inner) => inner.serialize(serializer),
            JournalEvent::AsteroidCracked(inner) => inner.serialize(serializer),
            JournalEvent::Backpack(inner) => inner.serialize(serializer),
            JournalEvent::BackpackChange(inner) => inner.serialize(serializer),
            JournalEvent::BookDropship(inner) => inner.serialize(serializer),
            JournalEvent::BookTaxi(inner) => inner.serialize(serializer),
            JournalEvent::Bounty(inner) => inner.serialize(serializer),
            JournalEvent::BuyAmmo(inner) => inner.serialize(serializer),
            JournalEvent::BuyDrones(inner) => inner.serialize(serializer),
            JournalEvent::BuyExplorationData(inner) => inner.serialize(serializer),
            JournalEvent::BuyMicroResources(inner) => inner.serialize(serializer),
            JournalEvent::BuySuit(inner) => inner.serialize(serializer),
            JournalEvent::BuyTradeData(inner) => inner.serialize(serializer),
            JournalEvent::BuyWeapon(inner) => inner.serialize(serializer),
            JournalEvent::CancelledSquadronApplication(inner) => inner.serialize(serializer),
            JournalEvent::CancelDropship(inner) => inner.serialize(serializer),
            JournalEvent::CancelTaxi(inner) => inner.serialize(serializer),
            JournalEvent::CapShipBond(inner) => inner.serialize(serializer),
            JournalEvent::Cargo(inner) => inner.serialize(serializer),
            JournalEvent::CargoDepot(inner) => inner.serialize(serializer),
            JournalEvent::CargoTransfer(inner) => inner.serialize(serializer),
            JournalEvent::CarrierBankTransfer(inner) => inner.serialize(serializer),
            JournalEvent::CarrierBuy(inner) => inner.serialize(serializer),
            JournalEvent::CarrierCancelDecommission(inner) => inner.serialize(serializer),
            JournalEvent::CarrierCrewServices(inner) => inner.serialize(serializer),
            JournalEvent::CarrierDecommission(inner) => inner.serialize(serializer),
            JournalEvent::CarrierDepositFuel(inner) => inner.serialize(serializer),
            JournalEvent::CarrierDockingPermission(inner) => inner.serialize(serializer),
            JournalEvent::CarrierFinance(inner) => inner.serialize(serializer),
            JournalEvent::CarrierJump(inner) => inner.serialize(serializer),
            JournalEvent::CarrierJumpCancelled(inner) => inner.serialize(serializer),
            JournalEvent::CarrierJumpRequest(inner) => inner.serialize(serializer),
            JournalEvent::CarrierLocation(inner) => inner.serialize(serializer),
            JournalEvent::CarrierModulePack(inner) => inner.serialize(serializer),
            JournalEvent::CarrierNameChange(inner) => inner.serialize(serializer),
            JournalEvent::CarrierShipPack(inner) => inner.serialize(serializer),
            JournalEvent::CarrierStats(inner) => inner.serialize(serializer),
            JournalEvent::CarrierTradeOrder(inner) => inner.serialize(serializer),
            JournalEvent::ChangeCrewRole(inner) => inner.serialize(serializer),
            JournalEvent::ClearImpound(inner) => inner.serialize(serializer),
            JournalEvent::ClearSavedGame(inner) => inner.serialize(serializer),
            JournalEvent::CockpitBreached(inner) => inner.serialize(serializer),
            JournalEvent::CodexEntry(inner) => inner.serialize(serializer),
            JournalEvent::CollectCargo(inner) => inner.serialize(serializer),
            JournalEvent::CollectItems(inner) => inner.serialize(serializer),
            JournalEvent::ColonisationBeaconDeployed(inner) => inner.serialize(serializer),
            JournalEvent::ColonisationConstructionDepot(inner) => inner.serialize(serializer),
            JournalEvent::ColonisationContribution(inner) => inner.serialize(serializer),
            JournalEvent::ColonisationSystemClaim(inner) => inner.serialize(serializer),
            JournalEvent::ColonisationSystemClaimRelease(inner) => inner.serialize(serializer),
            JournalEvent::Commander(inner) => inner.serialize(serializer),
            JournalEvent::CommitCrime(inner) => inner.serialize(serializer),
            JournalEvent::CommunityGoal(inner) => inner.serialize(serializer),
            JournalEvent::CommunityGoalDiscard(inner) => inner.serialize(serializer),
            JournalEvent::CommunityGoalJoin(inner) => inner.serialize(serializer),
            JournalEvent::CommunityGoalReward(inner) => inner.serialize(serializer),
            JournalEvent::CompleteConstruction(inner) => inner.serialize(serializer),
            JournalEvent::Continued(inner) => inner.serialize(serializer),
            JournalEvent::CreateSuitLoadout(inner) => inner.serialize(serializer),
            JournalEvent::CrewAssign(inner) => inner.serialize(serializer),
            JournalEvent::CrewFire(inner) => inner.serialize(serializer),
            JournalEvent::CrewHire(inner) => inner.serialize(serializer),
            JournalEvent::CrewLaunchFighter(inner) => inner.serialize(serializer),
            JournalEvent::CrewMemberJoins(inner) => inner.serialize(serializer),
            JournalEvent::CrewMemberQuits(inner) => inner.serialize(serializer),
            JournalEvent::CrewMemberRoleChange(inner) => inner.serialize(serializer),
            JournalEvent::CrimeVictim(inner) => inner.serialize(serializer),
            JournalEvent::DatalinkScan(inner) => inner.serialize(serializer),
            JournalEvent::DatalinkVoucher(inner) => inner.serialize(serializer),
            JournalEvent::DataScanned(inner) => inner.serialize(serializer),
            JournalEvent::DeleteSuitLoadout(inner) => inner.serialize(serializer),
            JournalEvent::DeliverPowerMicroResources(inner) => inner.serialize(serializer),
            JournalEvent::Died(inner) => inner.serialize(serializer),
            JournalEvent::DisbandedSquadron(inner) => inner.serialize(serializer),
            JournalEvent::DiscoveryScan(inner) => inner.serialize(serializer),
            JournalEvent::Disembark(inner) => inner.serialize(serializer),
            JournalEvent::Docked(inner) => inner.serialize(serializer),
            JournalEvent::DockingCancelled(inner) => inner.serialize(serializer),
            JournalEvent::DockingDenied(inner) => inner.serialize(serializer),
            JournalEvent::DockingGranted(inner) => inner.serialize(serializer),
            JournalEvent::DockingRequested(inner) => inner.serialize(serializer),
            JournalEvent::DockingTimeout(inner) => inner.serialize(serializer),
            JournalEvent::DockFighter(inner) => inner.serialize(serializer),
            JournalEvent::DockSRV(inner) => inner.serialize(serializer),
            JournalEvent::DropshipDeploy(inner) => inner.serialize(serializer),
            JournalEvent::DropItems(inner) => inner.serialize(serializer),
            JournalEvent::EjectCargo(inner) => inner.serialize(serializer),
            JournalEvent::Embark(inner) => inner.serialize(serializer),
            JournalEvent::EndCrewSession(inner) => inner.serialize(serializer),
            JournalEvent::EngineerContribution(inner) => inner.serialize(serializer),
            JournalEvent::EngineerCraft(inner) => inner.serialize(serializer),
            JournalEvent::EngineerProgress(inner) => inner.serialize(serializer),
            JournalEvent::EscapeInterdiction(inner) => inner.serialize(serializer),
            JournalEvent::FactionKillBond(inner) => inner.serialize(serializer),
            JournalEvent::FCMaterials(inner) => inner.serialize(serializer),
            JournalEvent::FetchRemoteModule(inner) => inner.serialize(serializer),
            JournalEvent::FighterDestroyed(inner) => inner.serialize(serializer),
            JournalEvent::FighterRebuilt(inner) => inner.serialize(serializer),
            JournalEvent::Fileheader(inner) => inner.serialize(serializer),
            JournalEvent::Friends(inner) => inner.serialize(serializer),
            JournalEvent::FSDJump(inner) => inner.serialize(serializer),
            JournalEvent::FSDTarget(inner) => inner.serialize(serializer),
            JournalEvent::FSSAllBodiesFound(inner) => inner.serialize(serializer),
            JournalEvent::FSSBodySignals(inner) => inner.serialize(serializer),
            JournalEvent::FSSDiscoveryScan(inner) => inner.serialize(serializer),
            JournalEvent::FSSSignalDiscovered(inner) => inner.serialize(serializer),
            JournalEvent::FuelScoop(inner) => inner.serialize(serializer),
            JournalEvent::HeatDamage(inner) => inner.serialize(serializer),
            JournalEvent::HeatWarning(inner) => inner.serialize(serializer),
            JournalEvent::HoloscreenHacked(inner) => inner.serialize(serializer),
            JournalEvent::HullDamage(inner) => inner.serialize(serializer),
            JournalEvent::Interdicted(inner) => inner.serialize(serializer),
            JournalEvent::Interdiction(inner) => inner.serialize(serializer),
            JournalEvent::InvitedToSquadron(inner) => inner.serialize(serializer),
            JournalEvent::JetConeBoost(inner) => inner.serialize(serializer),
            JournalEvent::JetConeDamage(inner) => inner.serialize(serializer),
            JournalEvent::JoinedSquadron(inner) => inner.serialize(serializer),
            JournalEvent::JoinACrew(inner) => inner.serialize(serializer),
            JournalEvent::KickedFromSquadron(inner) => inner.serialize(serializer),
            JournalEvent::KickCrewMember(inner) => inner.serialize(serializer),
            JournalEvent::LaunchDrone(inner) => inner.serialize(serializer),
            JournalEvent::LaunchFighter(inner) => inner.serialize(serializer),
            JournalEvent::LaunchSRV(inner) => inner.serialize(serializer),
            JournalEvent::LeaveBody(inner) => inner.serialize(serializer),
            JournalEvent::LeftSquadron(inner) => inner.serialize(serializer),
            JournalEvent::Liftoff(inner) => inner.serialize(serializer),
            JournalEvent::Loadout(inner) => inner.serialize(serializer),
            JournalEvent::LoadoutEquipModule(inner) => inner.serialize(serializer),
            JournalEvent::LoadoutRemoveModule(inner) => inner.serialize(serializer),
            JournalEvent::LoadGame(inner) => inner.serialize(serializer),
            JournalEvent::Location(inner) => inner.serialize(serializer),
            JournalEvent::Market(inner) => inner.serialize(serializer),
            JournalEvent::MarketBuy(inner) => inner.serialize(serializer),
            JournalEvent::MarketSell(inner) => inner.serialize(serializer),
            JournalEvent::MassModuleStore(inner) => inner.serialize(serializer),
            JournalEvent::Materials(inner) => inner.serialize(serializer),
            JournalEvent::MaterialCollected(inner) => inner.serialize(serializer),
            JournalEvent::MaterialDiscarded(inner) => inner.serialize(serializer),
            JournalEvent::MaterialDiscovered(inner) => inner.serialize(serializer),
            JournalEvent::MaterialTrade(inner) => inner.serialize(serializer),
            JournalEvent::MiningRefined(inner) => inner.serialize(serializer),
            JournalEvent::Missions(inner) => inner.serialize(serializer),
            JournalEvent::MissionAbandoned(inner) => inner.serialize(serializer),
            JournalEvent::MissionAccepted(inner) => inner.serialize(serializer),
            JournalEvent::MissionCompleted(inner) => inner.serialize(serializer),
            JournalEvent::MissionFailed(inner) => inner.serialize(serializer),
            JournalEvent::MissionRedirected(inner) => inner.serialize(serializer),
            JournalEvent::ModuleBuy(inner) => inner.serialize(serializer),
            JournalEvent::ModuleBuyAndStore(inner) => inner.serialize(serializer),
            JournalEvent::ModuleInfo(inner) => inner.serialize(serializer),
            JournalEvent::ModuleRetrieve(inner) => inner.serialize(serializer),
            JournalEvent::ModuleSell(inner) => inner.serialize(serializer),
            JournalEvent::ModuleSellRemote(inner) => inner.serialize(serializer),
            JournalEvent::ModuleStore(inner) => inner.serialize(serializer),
            JournalEvent::ModuleSwap(inner) => inner.serialize(serializer),
            JournalEvent::MultiSellExplorationData(inner) => inner.serialize(serializer),
            JournalEvent::Music(inner) => inner.serialize(serializer),
            JournalEvent::NavBeaconScan(inner) => inner.serialize(serializer),
            JournalEvent::NavRoute(inner) => inner.serialize(serializer),
            JournalEvent::NavRouteClear(inner) => inner.serialize(serializer),
            JournalEvent::NewCommander(inner) => inner.serialize(serializer),
            JournalEvent::NpcCrewPaidWage(inner) => inner.serialize(serializer),
            JournalEvent::NpcCrewRank(inner) => inner.serialize(serializer),
            JournalEvent::Outfitting(inner) => inner.serialize(serializer),
            JournalEvent::Passengers(inner) => inner.serialize(serializer),
            JournalEvent::PayBounties(inner) => inner.serialize(serializer),
            JournalEvent::PayFines(inner) => inner.serialize(serializer),
            JournalEvent::Powerplay(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayCollect(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayDefect(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayDeliver(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayFastTrack(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayJoin(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayLeave(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayMerits(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayRank(inner) => inner.serialize(serializer),
            JournalEvent::PowerplaySalary(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayVote(inner) => inner.serialize(serializer),
            JournalEvent::PowerplayVoucher(inner) => inner.serialize(serializer),
            JournalEvent::Progress(inner) => inner.serialize(serializer),
            JournalEvent::Promotion(inner) => inner.serialize(serializer),
            JournalEvent::ProspectedAsteroid(inner) => inner.serialize(serializer),
            JournalEvent::PVPKill(inner) => inner.serialize(serializer),
            JournalEvent::QuitACrew(inner) => inner.serialize(serializer),
            JournalEvent::Rank(inner) => inner.serialize(serializer),
            JournalEvent::RebootRepair(inner) => inner.serialize(serializer),
            JournalEvent::ReceiveText(inner) => inner.serialize(serializer),
            JournalEvent::RedeemVoucher(inner) => inner.serialize(serializer),
            JournalEvent::RefuelAll(inner) => inner.serialize(serializer),
            JournalEvent::RefuelPartial(inner) => inner.serialize(serializer),
            JournalEvent::RenameSuitLoadout(inner) => inner.serialize(serializer),
            JournalEvent::Repair(inner) => inner.serialize(serializer),
            JournalEvent::RepairAll(inner) => inner.serialize(serializer),
            JournalEvent::RepairDrone(inner) => inner.serialize(serializer),
            JournalEvent::Reputation(inner) => inner.serialize(serializer),
            JournalEvent::RequestPowerMicroResources(inner) => inner.serialize(serializer),
            JournalEvent::ReservoirReplenished(inner) => inner.serialize(serializer),
            JournalEvent::RestockVehicle(inner) => inner.serialize(serializer),
            JournalEvent::Resupply(inner) => inner.serialize(serializer),
            JournalEvent::Resurrect(inner) => inner.serialize(serializer),
            JournalEvent::SAAScanComplete(inner) => inner.serialize(serializer),
            JournalEvent::SAASignalsFound(inner) => inner.serialize(serializer),
            JournalEvent::Scan(inner) => inner.serialize(serializer),
            JournalEvent::Scanned(inner) => inner.serialize(serializer),
            JournalEvent::ScanBaryCentre(inner) => inner.serialize(serializer),
            JournalEvent::ScanOrganic(inner) => inner.serialize(serializer),
            JournalEvent::ScientificResearch(inner) => inner.serialize(serializer),
            JournalEvent::Screenshot(inner) => inner.serialize(serializer),
            JournalEvent::SearchAndRescue(inner) => inner.serialize(serializer),
            JournalEvent::SelfDestruct(inner) => inner.serialize(serializer),
            JournalEvent::SellDrones(inner) => inner.serialize(serializer),
            JournalEvent::SellExplorationData(inner) => inner.serialize(serializer),
            JournalEvent::SellMicroResources(inner) => inner.serialize(serializer),
            JournalEvent::SellOrganicData(inner) => inner.serialize(serializer),
            JournalEvent::SellShipOnRebuy(inner) => inner.serialize(serializer),
            JournalEvent::SellSuit(inner) => inner.serialize(serializer),
            JournalEvent::SellWeapon(inner) => inner.serialize(serializer),
            JournalEvent::SendText(inner) => inner.serialize(serializer),
            JournalEvent::SetUserShipName(inner) => inner.serialize(serializer),
            JournalEvent::SharedBookmarkToSquadron(inner) => inner.serialize(serializer),
            JournalEvent::ShieldState(inner) => inner.serialize(serializer),
            JournalEvent::Shipyard(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardBankDeposit(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardBuy(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardNew(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardRedeem(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardSell(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardSwap(inner) => inner.serialize(serializer),
            JournalEvent::ShipyardTransfer(inner) => inner.serialize(serializer),
            JournalEvent::ShipLocker(inner) => inner.serialize(serializer),
            JournalEvent::ShipLockerBackpack(inner) => inner.serialize(serializer),
            JournalEvent::ShipLockerMaterials(inner) => inner.serialize(serializer),
            JournalEvent::ShipRedeemed(inner) => inner.serialize(serializer),
            JournalEvent::ShipTargeted(inner) => inner.serialize(serializer),
            JournalEvent::Shutdown(inner) => inner.serialize(serializer),
            JournalEvent::SquadronApplicationApproved(inner) => inner.serialize(serializer),
            JournalEvent::SquadronApplicationRejected(inner) => inner.serialize(serializer),
            JournalEvent::SquadronCreated(inner) => inner.serialize(serializer),
            JournalEvent::SquadronDemotion(inner) => inner.serialize(serializer),
            JournalEvent::SquadronPromotion(inner) => inner.serialize(serializer),
            JournalEvent::SquadronStartup(inner) => inner.serialize(serializer),
            JournalEvent::SRVDestroyed(inner) => inner.serialize(serializer),
            JournalEvent::StartUp(inner) => inner.serialize(serializer),
            JournalEvent::StartJump(inner) => inner.serialize(serializer),
            JournalEvent::Statistics(inner) => inner.serialize(serializer),
            JournalEvent::Status(inner) => inner.serialize(serializer),
            JournalEvent::StoredModules(inner) => inner.serialize(serializer),
            JournalEvent::StoredShips(inner) => inner.serialize(serializer),
            JournalEvent::SuitLoadout(inner) => inner.serialize(serializer),
            JournalEvent::SupercruiseDestinationDrop(inner) => inner.serialize(serializer),
            JournalEvent::SupercruiseEntry(inner) => inner.serialize(serializer),
            JournalEvent::SupercruiseExit(inner) => inner.serialize(serializer),
            JournalEvent::SwitchSuitLoadout(inner) => inner.serialize(serializer),
            JournalEvent::Synthesis(inner) => inner.serialize(serializer),
            JournalEvent::SystemsShutdown(inner) => inner.serialize(serializer),
            JournalEvent::TechnologyBroker(inner) => inner.serialize(serializer),
            JournalEvent::Touchdown(inner) => inner.serialize(serializer),
            JournalEvent::TradeMicroResources(inner) => inner.serialize(serializer),
            JournalEvent::TransferMicroResources(inner) => inner.serialize(serializer),
            JournalEvent::UnderAttack(inner) => inner.serialize(serializer),
            JournalEvent::Undocked(inner) => inner.serialize(serializer),
            JournalEvent::UpgradeSuit(inner) => inner.serialize(serializer),
            JournalEvent::UpgradeWeapon(inner) => inner.serialize(serializer),
            JournalEvent::UseConsumable(inner) => inner.serialize(serializer),
            JournalEvent::USSDrop(inner) => inner.serialize(serializer),
            JournalEvent::VehicleSwitch(inner) => inner.serialize(serializer),
            JournalEvent::WingAdd(inner) => inner.serialize(serializer),
            JournalEvent::WingInvite(inner) => inner.serialize(serializer),
            JournalEvent::WingJoin(inner) => inner.serialize(serializer),
        }
    }
}
