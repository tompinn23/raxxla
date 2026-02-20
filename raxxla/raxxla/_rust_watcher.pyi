from dataclasses import dataclass
from typing import Any, Literal, Protocol, Union

__all__ = 'Watcher', 'ModuleModifier', 'ModuleEngineering', 'FuelTanks', 'Module', 'Suit', 'SuitLoadout', 'BackpackContents', 'PowerplayInfo', 'GameState'

__version__: str
"""The package version as defined in `Cargo.toml`, modified to match raxxla's versioning semantics."""

class AbstractEvent(Protocol):
    def is_set(self) -> bool: ...

class Watcher:
    def __init__(self, dirs: list[str]) -> None: ...

    def watch(self, timeout_ms: int, stop_event: AbstractEvent | None) -> tuple[GameState, dict[str, Any]] | Literal['stop', 'timeout']: ...

@dataclass(frozen=True)
class ModuleModifier:
    label: str
    less_is_good: int | None
    original_value: float | None
    value: float | None
    value_str: str | None
    value_str_localised: str | None


@dataclass(frozen=True)
class ModuleEngineering:
    blueprint_id: int
    blueprint_name: str
    engineer: str | None
    engineer_id: int
    experimental_effect: str | None
    experimental_effect_localised: str | None
    level: int
    modifiers: list[ModuleModifier]
    quality: float


@dataclass(frozen=True)
class FuelTanks:
    main: float
    reserve: float


@dataclass(frozen=True)
class Module:
    ammo_in_clip: int | None
    ammo_in_hopper: int | None
    engineering: ModuleEngineering | None
    health: float
    item: str
    on: bool
    priority: int
    slot: str
    value: int | None


@dataclass(frozen=True)
class Suit:
    suit_id: int
    edmc_name: str
    loc_name: str
    name: str
    mods: list[str]
    id: int | None


@dataclass(frozen=True)
class SuitLoadout:
    loadout_slot_id: int
    suit: Suit
    name: str
    slots: dict[str, str]


@dataclass(frozen=True)
class BackpackContents:
    component: dict[str, int]
    consumable: dict[str, int]
    item: dict[str, int]
    data: dict[str, int]


@dataclass(frozen=True)
class PowerplayInfo:
    power: str | None
    rank: int | None
    merits: int | None
    votes: int | None
    time_pledged: int | None


@dataclass(frozen=True)
class GameState:
    game_language: str | None
    game_version: str | None
    game_build: str | None

    captain: str | None
    cargo: dict[str, int]
    credits: int
    fid: str | None
    horizons: bool | None
    odyssey: bool
    loan: int | None

    raw: dict[str, int]
    manufactured: dict[str, int]
    encoded: dict[str, int]

    engineers: dict[str, Union[tuple[int, int], str]]
    rank: dict[str, tuple[int, int]]
    reputation: dict[str, int]
    statistics: dict[str, int]

    role: str | None

    friends: set[str]

    ship_id: int | None
    ship_ident: str | None
    ship_name: str | None
    ship_type: str | None
    hull_value: int | None
    modules_value: int | None
    unladen_mass: float | None
    cargo_capacity: int | None
    max_jump_range: float | None
    fuel_capacity: FuelTanks | None
    rebuy: int | None

    modules: dict[str, Module]
    cargo_json: str | None
    route: str | None

    is_docked: bool
    on_foot: bool

    component: dict[str, int]
    item: dict[str, int]
    consumable: dict[str, int]
    data: dict[str, int]

    backpack: BackpackContents
    backpack_json: str | None
    ship_locker_json: str | None

    suits: dict[int, Suit]
    suit_loadouts: dict[int, SuitLoadout]
    suit_current: int | None
    suit_loadout_current: int | None

    taxi: bool
    dropship: bool

    star_pos: list[float]
    system_address: int | None
    system_name: str | None
    system_population: int | None
    body: str | None
    body_id: int | None
    body_type: str | None
    station_name: str | None
    station_type: str | None
    market_id: int | None

    nav_route: str | None

    power_play: PowerplayInfo