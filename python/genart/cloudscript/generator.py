from typing import Dict, List, Optional, Sequence

import numpy as np
from numpy.random import Generator

from genart.geom import unit_vector

from .models import BubbleChamber, Particle, SplitTree, SuperChamber

EMPTY_CHAMBER = BubbleChamber(5.0, 0.5)


def make_superchamber(
    rng: Generator,
    width: int,
    height: int,
    layout: Sequence[Sequence[Optional[str]]],
) -> SuperChamber:
    # Pre-fill the chamber cache with a default for empty chambers:
    chamber_cache: Dict[Optional[str], BubbleChamber] = {None: EMPTY_CHAMBER}
    chambers: List[List[BubbleChamber]] = []

    for row in layout:
        chambers.append([])

        for symbol in row:
            try:
                c = chamber_cache[symbol]
            except KeyError:
                c = random_chamber(rng)
                chamber_cache[symbol] = c

            chambers[-1].append(c)

    return SuperChamber(width, height, chambers)


def random_chamber(rng: Generator) -> BubbleChamber:
    magnetic_field = rng.lognormal(1.0, 0.2)
    friction = rng.uniform(0.2, 0.6)

    return BubbleChamber(magnetic_field, friction)


def random_charges(rng: Generator) -> Sequence[int]:
    return rng.choice([-2, -1, 1, 2], size=rng.integers(low=2, high=6))


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


def random_particle(
    rng: Generator,
    pos: Sequence[float],
    velocity: Sequence[float],
    lifetime: float = None,
    charges: Sequence[int] = None,
) -> Particle:
    charges = charges or random_charges(rng)
    lifetime = lifetime or rng.uniform(0.7, 3.5)
    tree = random_split_tree(rng, len(charges))

    return Particle(pos, velocity, charges, lifetime, tree)


def generate_particles(rng: Generator, chamber: SuperChamber) -> Sequence[Particle]:
    particle_cache: Dict[int, List[Particle]] = {}
    colwidth = chamber.col_width
    rowheight = chamber.row_height
    center = np.array([colwidth / 2.0, rowheight / 2.0])

    results = []

    for i in range(chamber.rows):
        for j, col in enumerate(chamber.chambers[i]):
            if col is EMPTY_CHAMBER:
                continue

            try:
                particles = particle_cache[id(col)]
            except KeyError:
                pos_x = colwidth * rng.random()
                pos_y = rowheight * rng.random()

                pos = np.array([pos_x, pos_y])
                velo = unit_vector(center, pos) * rng.normal(colwidth, colwidth / 10.0)
                particles = [
                    random_particle(rng, pos, velo) for _ in range(rng.integers(2, 4))
                ]
                particle_cache[id(col)] = particles

            chamber_corner = np.array([j * colwidth, i * rowheight])

            for particle in particles:
                abs_pos = chamber_corner + particle.position

                results.append(
                    Particle(
                        abs_pos,
                        particle.velocity.copy(),
                        particle.charges.copy(),
                        particle.decays_after,
                        particle.split_tree,
                    )
                )

    return results
