import random
from collections import namedtuple
from typing import Dict, List, Optional, Sequence

from .models import BubbleChamber, Particle, SuperChamber

EMPTY_CHAMBER = BubbleChamber(1.0, 1.0)


def make_superchamber(
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
                c = random_chamber()
                chamber_cache[symbol] = c

            chambers[-1].append(c)

    return SuperChamber(width, height, chambers)


def random_chamber() -> BubbleChamber:
    magnetic_field = random.lognormvariate(1.0, 0.2)
    friction = random.uniform(0.2, 0.5)

    return BubbleChamber(magnetic_field, friction)


ParticleSetup = namedtuple("ParticleSetup", ("octant", "charge", "mass"))


def generate_particles(chamber: SuperChamber) -> Sequence[Particle]:
    particle_cache: Dict[int, List[ParticleSetup]] = {}
    colwidth = chamber.col_width
    rowheight = chamber.row_height

    results = []

    for i in range(chamber.rows):
        for j, col in enumerate(chamber.chambers[i]):
            if col is EMPTY_CHAMBER:
                continue

            chamber_min_x = j * colwidth
            chamber_min_y = i * rowheight
            chamber_max_x = chamber_min_x + colwidth
            chamber_max_y = chamber_min_y + rowheight

            try:
                particles = particle_cache[id(col)]
            except KeyError:
                particles = [
                    ParticleSetup(
                        random.randint(1, 8),
                        random.normalvariate(4.0, 1.0) * random.choice([-1.0, 1.0]),
                        1.0 + random.lognormvariate(1.0, 3.0),
                    )
                    for _ in range(random.randint(1, 3))
                ]
                particle_cache[id(col)] = particles

            for particle in particles:
                octant, charge, mass = particle

                # Generate a particle in the outer band of the chamber
                if octant in (1, 4, 5, 8):
                    offset_x_range = 0.2
                    offset_y_range = 0.4
                else:
                    offset_x_range = 0.4
                    offset_y_range = 0.2

                offset_x = offset_x_range * colwidth * random.random()
                pos_x = (
                    (chamber_min_x + offset_x)
                    if octant in (3, 4, 5, 6)
                    else (chamber_max_x - offset_x)
                )

                offset_y = offset_y_range * rowheight * random.random()
                pos_y = (
                    (chamber_min_y + offset_y)
                    if octant <= 4
                    else (chamber_max_y - offset_y)
                )

                # With a velocity pointing inward, more or less to the center.
                velo_x = random.normalvariate(colwidth / 2.0, colwidth / 50.0) * (
                    1.0 if octant in (3, 4, 5, 6) else -1.0
                )
                velo_y = random.normalvariate(rowheight / 2.0, rowheight / 50.0) * (
                    1.0 if octant <= 4 else -1.0
                )

                results.append(
                    Particle(
                        (pos_x, pos_y),
                        (velo_x, velo_y),
                        charge,
                        mass,
                        random.uniform(0.7, 3.5),
                    )
                )

    return results
