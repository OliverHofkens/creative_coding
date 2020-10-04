import random
from dataclasses import dataclass, field
from functools import cached_property
from numbers import Real
from typing import Sequence

import numpy as np


@dataclass
class Particle:
    position: Sequence[float]
    velocity: Sequence[float]
    charge: float
    mass: float
    decays_after: float = field(init=False)
    lifetime: float = 0.0
    is_alive: bool = True
    is_dirty: bool = True

    def __post_init__(self):
        if not isinstance(self.position, np.ndarray):
            self.position = np.array(self.position)

        if not isinstance(self.velocity, np.ndarray):
            self.velocity = np.array(self.velocity)

        # Generate a decay time for this particle:
        self.decays_after = random.expovariate(0.2)


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
    def row_height(self) -> int:
        rows = len(self.chambers)
        return self.height // rows

    @cached_property
    def col_width(self) -> int:
        cols = len(self.chambers[0])
        return self.width // cols

    def chamber_at(self, x: Real, y: Real) -> BubbleChamber:
        row = int(y // self.row_height)
        col = int(x // self.col_width)

        try:
            return self.chambers[row][col]
        except IndexError:
            return self.default_chamber
