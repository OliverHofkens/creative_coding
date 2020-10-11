import random
from typing import Dict, List, Optional, Sequence

from .models import BubbleChamber, Particle, SuperChamber

EMPTY_CHAMBER = BubbleChamber(1.0, 1.0)


def make_superchamber(
    width: int,
    height: int,
    layout: Sequence[Sequence[Optional[str]]],
    magnetic_field_stddev: float,
    friction_lambda: float,
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
                c = random_chamber(magnetic_field_stddev, friction_lambda)
                chamber_cache[symbol] = c

            chambers[-1].append(c)

    return SuperChamber(width, height, chambers)


def random_chamber(
    magnetic_field_stddev: float, friction_lambda: float
) -> BubbleChamber:
    magnetic_field = (
        1.0 + random.lognormvariate(1.0, magnetic_field_stddev)
    ) * random.choice([1, -1])
    friction = random.expovariate(friction_lambda)

    return BubbleChamber(magnetic_field, friction)


def generate_particles(chamber: SuperChamber) -> Sequence[Particle]:
    colwidth = chamber.col_width
    rowheight = chamber.row_height

    return [
        Particle(
            (
                j * colwidth + random.random() * colwidth / 2,
                i * rowheight + random.random() * rowheight / 2,
            ),
            (random.normalvariate(50.0, 10.0), random.normalvariate(50.0, 10.0)),
            random.normalvariate(0.0, 5.0),
            random.lognormvariate(1.0, 3.0),
        )
        for i in range(chamber.rows)
        for j, col in enumerate(chamber.chambers[i])
        if col is not EMPTY_CHAMBER
    ]
