import random
from dataclasses import fields
from itertools import product
from typing import Optional

import numpy as np

from . import models

PUPIL_CHOICES = [models.Pupil, *models.Pupil.__subclasses__()]


def fill(width: int, height: int):
    DIVISIONS = 4
    part_width = width / DIVISIONS
    part_height = height / DIVISIONS
    part = np.array([part_width, part_height])

    max_radius = (part_width - 1) / 2

    eyes = []

    for x, y in product(range(DIVISIONS), range(DIVISIONS)):
        upper_left = [x, y] * part
        center = upper_left + 0.5 * part

        eye = random_eye(center, max_radius)
        eyes.append(eye)

    return eyes


def random_eye(pos: np.ndarray, max_size: float) -> models.Eye:
    eye_size = random.uniform(3.0, max_size)
    iris = random_or_no_iris(pos, eye_size)
    max_pupil_size = iris.size if iris else eye_size
    pupil = random_pupil(pos, max_pupil_size)

    return models.Eye(pos, eye_size, pupil, iris)


def random_pupil(pos: np.ndarray, max_size: float) -> models.Pupil:
    cls = random.choice(PUPIL_CHOICES)
    flds = {f.name for f in fields(cls)}
    size = random.uniform(1.0, max_size)
    kwargs = dict(pos=pos, size=size)

    if "width" in flds:
        kwargs["width"] = random.uniform(1.0, 2.0)

    return cls(**kwargs)


def random_or_no_iris(pos: np.ndarray, max_size: float) -> Optional[models.Iris]:
    has_iris = random.uniform(0, 1) > 0.5
    if not has_iris:
        return None

    size = random.uniform(2.0, max_size)
    return models.Iris(pos, size)
