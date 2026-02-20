from typing import Generator, Any, TYPE_CHECKING, Union


from ._rust_watcher import Watcher, GameState

if TYPE_CHECKING:
    import anyio
    import asyncio
    from typing import Protocol

    import trio

    AnyEvent = Union[anyio.Event, asyncio.Event, trio.Event]

    class AbstractEvent(Protocol):
        def is_set(self) -> bool: ...


def watch(paths: list[str], timeout_ms: int = 200, stop_event: 'AbstractEvent | None' = None) -> Generator[tuple[GameState, dict[str, Any]]]:
    with Watcher(paths) as watcher:
        while True:
            ev = watcher.watch(timeout_ms, stop_event)
            if ev == 'timeout':
                continue
            if ev == 'stop':
                break
            yield ev
