from dataclasses import dataclass, field
from functools import cached_property
from numbers import Real
from typing import Sequence

import numpy as np


@dataclass
class SplitTree:
    """Tree that represents how a Particle would split"""

    count: int
    parts: Sequence["SplitTree"]

    def __post_init__(self):
        if self.count > 1 and sum(p.count for p in self.parts) != self.count:
            raise ValueError(
                f"Invalid SplitTree: Mass of {self.count} cannot be constructed "
                f"from parts {self.parts}"
            )


@dataclass
class Particle:
    position: Sequence[float]
    velocity: Sequence[float]
    charges: Sequence[int]
    decays_after: float
    split_tree: SplitTree
    lifetime: float = 0.0
    is_alive: bool = True
    is_dirty: bool = True

    def __post_init__(self):
        if not isinstance(self.position, np.ndarray):
            self.position = np.array(self.position)

        if not isinstance(self.velocity, np.ndarray):
            self.velocity = np.array(self.velocity)

        if not isinstance(self.charges, np.ndarray):
            self.charges = np.array(self.charges)

        if not self.mass == self.split_tree.count:
            raise ValueError(f"Invalid SplitTree {self.split_tree} for particle {self}")

        self.decays_after = 1.0 * self.mass

    @property
    def total_charge(self) -> int:
        return np.sum(self.charges)

    @property
    def mass(self) -> int:
        return len(self.charges)


@dataclass
class BubbleChamber:
    magnetic_field: float
    friction: float

    _magnet_vector: np.array = field(init=False)

    def __post_init__(self):
        self._magnet_vector = np.array([self.magnetic_field, -1 * self.magnetic_field])


@dataclass
class SuperChamber:
    width: int
    height: int
    chambers: Sequence[Sequence[BubbleChamber]]
    default_chamber: BubbleChamber = BubbleChamber(0.0, 0.0)

    @cached_property
    def rows(self) -> int:
        return len(self.chambers)

    @cached_property
    def columns(self) -> int:
        return len(self.chambers[0])

    @cached_property
    def row_height(self) -> int:
        return self.height // self.rows

    @cached_property
    def col_width(self) -> int:
        return self.width // self.columns

    def chamber_at(self, x: Real, y: Real) -> BubbleChamber:
        row = int(y // self.row_height)
        col = int(x // self.col_width)

        try:
            return self.chambers[row][col]
        except IndexError:
            return self.default_chamber
