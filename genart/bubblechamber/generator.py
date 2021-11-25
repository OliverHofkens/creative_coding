from typing import Sequence

import numpy as np
from numpy.random import Generator

from .models import BubbleChamber, Particle, SplitTree


def make_chamber(
    rng: Generator, magnetic_field: float = None, friction: float = None
) -> BubbleChamber:
    return BubbleChamber(
        magnetic_field or rng.lognormal(1.0, 0.2), friction or rng.uniform(0.2, 0.6)
    )


def random_charges(rng: Generator) -> Sequence[int]:
    return rng.choice([-2, -1, 1, 2], size=rng.integers(low=5, high=100))


def random_split_tree(rng: Generator, mass: int) -> SplitTree:
    if mass == 1:
        return SplitTree(1, [])
    else:
        leftover_mass = mass
        parts = []

        while leftover_mass > 2:
            subtree_mass = rng.integers(low=1, high=leftover_mass - 1)
            parts.append(random_split_tree(rng, subtree_mass))
            leftover_mass -= subtree_mass

        for _ in range(leftover_mass):
            parts.append(SplitTree(1, []))

        return SplitTree(mass, parts)


def make_particle(
    rng: Generator,
    pos: Sequence[float],
    velocity: Sequence[float],
    charges: Sequence[int] = None,
    split_tree: SplitTree = None,
) -> Particle:
    charges = charges if charges is not None else random_charges(rng)
    lifetime = rng.exponential(2.0)
    tree = split_tree or random_split_tree(rng, len(charges))

    return Particle(pos, velocity, charges, lifetime, tree)


def generate_particles(
    rng: Generator,
    width: int,
    height: int,
    n_particles: int = None,
    allow_3d: bool = False,
) -> Sequence[Particle]:
    center = np.array([width / 2, height / 2, 0.0])
    avg_velocity = np.linalg.norm(center)

    res = []
    for _ in range(n_particles or rng.integers(low=1, high=5)):
        pos = np.copy(center)
        velo = rng.normal(0.0, avg_velocity / 5.0, size=3)
        if not allow_3d:
            velo[2] = 0.0
        res.append(make_particle(rng, pos, velo))

    return res
