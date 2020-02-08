import random
from dataclasses import dataclass, field
from typing import Sequence

import numpy as np


@dataclass
class Particle:
    position: Sequence[float]
    velocity: Sequence[float]
    charges: Sequence[int]
    decays_after: float = field(init=False)
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

        # Generate a decay time for this particle:
        self.decays_after = random.expovariate(0.5)

    @property
    def total_charge(self) -> int:
        return self.charges[0] - self.charges[2]

    @property
    def mass(self) -> int:
        return np.sum(self.charges)


@dataclass
class BubbleChamber:
    dimensions: np.ndarray
    magnetic_field: np.ndarray
    friction: float

    def __post_init__(self):
        if not isinstance(self.dimensions, np.ndarray):
            self.dimensions = np.array(self.dimensions)

        if not isinstance(self.magnetic_field, np.ndarray):
            self.magnetic_field = np.array(self.magnetic_field)
