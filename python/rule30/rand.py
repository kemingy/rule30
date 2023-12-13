from __future__ import annotations

from random import Random
from time import time
from typing import Any

from .rule30py import Rule30

_default_ca = Rule30(int(time()))


def random():
    """Return a random number between 0 and 1."""
    return _default_ca.next_random()


class Rule30Random(Random):
    """A random number generator based on Rule 30."""

    def __init__(self, seed: int | None = None):
        """Create a new random number generator.

        If seed is not specified, use the current time as seed.
        """
        if seed is None:
            seed = int(time())
        elif seed < 0:
            raise ValueError("Seed must be a non-negative integer")
        self._ca = Rule30(seed)

    def seed(
        self, a: int | float | str | bytes | bytearray | None = None, version: int = 2
    ) -> None:
        return super().seed(a, version)

    def getstate(self) -> tuple[Any, ...]:
        return self._ca.get_state()

    def setstate(self, state: tuple[Any, ...]) -> None:
        self._ca.set_state(state)

    def getrandbits(self, k: int) -> int:
        if k < 0:
            raise ValueError("number of bits must be non-negative")
        value = 0
        round = (k + 63) // 64
        for _ in range(round):
            value = (value << 64) | self._ca.next_u64()
        x = int.from_bytes(value.to_bytes(round * 8, "big"), "big")
        return x >> (round * 64 - k)

    def random(self):
        """Return a random number between 0 and 1."""
        return self._ca.next_random()
