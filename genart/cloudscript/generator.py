import random
from typing import Sequence

from .models import BubbleChamber, Particle, SuperChamber


def make_superchamber(
    width: int,
    height: int,
    rows: int,
    columns: int,
    magnetic_field_stddev: float,
    friction_lambda: float,
) -> SuperChamber:
    return SuperChamber(
        width,
        height,
        [
            [
                random_chamber(magnetic_field_stddev, friction_lambda)
                for _ in range(columns)
            ]
            for _ in range(rows)
        ],
    )


def random_chamber(
    magnetic_field_stddev: float, friction_lambda: float
) -> BubbleChamber:
    magnetic_field = (
        1.0 + random.lognormvariate(1.0, magnetic_field_stddev)
    ) * random.choice([1, -1])
    friction = random.expovariate(friction_lambda)

    return BubbleChamber(magnetic_field, friction)


def generate_particles(chamber: SuperChamber) -> Sequence[Particle]:
    rows = len(chamber.chambers)
    cols = len(chamber.chambers[0])

    colwidth = chamber.col_width
    rowheight = chamber.row_height

    return [
        Particle(
            (
                col * colwidth + random.random() * colwidth / 2,
                row * rowheight + random.random() * rowheight / 2,
            ),
            (random.normalvariate(50.0, 10.0), random.normalvariate(50.0, 10.0)),
            random.normalvariate(0.0, 5.0),
            random.lognormvariate(1.0, 3.0),
        )
        for row in range(rows)
        for col in range(cols)
    ]
