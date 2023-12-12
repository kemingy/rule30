from __future__ import annotations

from random import Random
from typing import Any

from rule30py import Rule30

_default_ca = Rule30()


def random():
    """Return a random number between 0 and 1."""
    return _default_ca.next_random()


class Rule30Random(Random):
    """A random number generator based on Rule 30."""

    def __init__(self, seed: int | None = None):
        """Create a new random number generator.

        If ca is not specified, use the default Rule 30 CA.
        """
        self._ca = Rule30(seed) if seed else Rule30(42)

    def seed(
        self, a: int | float | str | bytes | bytearray | None = None, version: int = 2
    ) -> None:
        return super().seed(a, version)

    def getstate(self) -> tuple[Any, ...]:
        return super().getstate()

    def setstate(self, state: tuple[Any, ...]) -> None:
        return super().setstate(state)

    def getrandbits(self, k: int) -> int:
        return super().getrandbits(k)

    def random(self):
        """Return a random number between 0 and 1."""
        return self._ca.next_random()
