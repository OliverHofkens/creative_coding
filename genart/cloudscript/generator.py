import random
from typing import Dict, List, Optional, Sequence

from .models import BubbleChamber, Particle, SuperChamber

EMPTY_CHAMBER = BubbleChamber(1.0, 1.0)


def make_superchamber(
    width: int,
    height: int,
    layout: Sequence[Sequence[Optional[str]]],
    magnetic_field_stddev: float,
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
                c = random_chamber(magnetic_field_stddev)
                chamber_cache[symbol] = c

            chambers[-1].append(c)

    return SuperChamber(width, height, chambers)


def random_chamber(magnetic_field_stddev: float) -> BubbleChamber:
    magnetic_field = (
        1.0 + random.lognormvariate(1.0, magnetic_field_stddev)
    ) * random.choice([1, -1])
    friction = random.uniform(0.1, 0.7)

    return BubbleChamber(magnetic_field, friction)


def generate_particles(chamber: SuperChamber) -> Sequence[Particle]:
    colwidth = chamber.col_width
    rowheight = chamber.row_height

    results = []

    for i in range(chamber.rows):
        for j, col in enumerate(chamber.chambers[i]):
            if col is EMPTY_CHAMBER:
                continue

            # Generate a particle in the outer band of the chamber
            quadrant = random.randint(1, 4)
            chamber_min_x = j * colwidth
            chamber_min_y = i * rowheight
            chamber_max_x = chamber_min_x + colwidth
            chamber_max_y = chamber_min_y + rowheight

            offset_x = 0.2 * colwidth * random.random()
            pos_x = (
                (chamber_min_x + offset_x)
                if quadrant in (2, 4)
                else (chamber_max_x - offset_x)
            )
            offset_y = 0.2 * rowheight * random.random()
            pos_y = (
                (chamber_min_y + offset_y)
                if quadrant <= 2
                else (chamber_max_y - offset_y)
            )

            # With a velocity pointing inward, more or less to the center.
            velo_x = random.normalvariate(colwidth / 2.0, colwidth / 100.0) * (
                1.0 if quadrant in (3, 4) else -1.0
            )
            velo_y = random.normalvariate(rowheight / 2.0, rowheight / 100.0) * (
                1.0 if quadrant <= 2 else -1.0
            )

            results.append(
                Particle(
                    (pos_x, pos_y),
                    (velo_x, velo_y),
                    random.normalvariate(5.0, 5.0) * random.choice([-1, 1]),
                    1.0 + random.lognormvariate(1.0, 3.0),
                    random.gammavariate(2.0, 1.0),
                )
            )

    return results
