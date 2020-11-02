import math
from dataclasses import fields
from typing import Optional

import numpy as np
from numpy.random import Generator

from genart import color

from . import models, palette

PUPIL_CHOICES = [models.Pupil, *models.Pupil.__subclasses__()]


def random_eye(rng: Generator, pos: np.ndarray, size: float) -> models.Eye:
    iris = random_or_no_iris(rng, pos, size)
    max_pupil_size = iris.size if iris else size
    pupil = random_pupil(rng, pos, max_pupil_size)
    color_ = color.Color(1, 1, 1)
    rotation = rng.uniform(0.0, math.pi)
    eyelids = random_or_no_eyelids(rng, pos, size)

    return models.Eye(pos, size, color_, pupil, iris, eyelids, rotation)


def random_pupil(rng: Generator, pos: np.ndarray, max_size: float) -> models.Pupil:
    cls = rng.choice(PUPIL_CHOICES)
    flds = {f.name for f in fields(cls)}

    if cls is models.SlitPupil:
        min_size = 0.5 * max_size
        size = rng.triangular(min_size, (min_size + max_size) / 2, max_size)
    else:
        min_size = 1.0
        max_size = 0.8 * max_size
        size = rng.triangular(min_size, (min_size + max_size) / 2, max_size)

    kwargs = dict(pos=pos, size=size)

    if "width" in flds:
        kwargs["width"] = rng.uniform(1.0, size)

    return cls(**kwargs)


def random_or_no_iris(
    rng: Generator, pos: np.ndarray, max_size: float
) -> Optional[models.Iris]:
    has_iris = rng.uniform(0, 1) > 0.5
    if not has_iris:
        return None

    size = rng.triangular(max_size / 2.0, 0.75 * max_size, max_size)
    color = random_radial_gradient(rng)
    return models.Iris(pos, size, color)


def random_or_no_eyelids(
    rng: Generator, pos: np.ndarray, max_size: float
) -> Optional[models.Eyelids]:
    has_eyelids = rng.uniform(0, 1) > 0.75
    if not has_eyelids:
        return None

    size = max_size + rng.uniform(0.0, 0.5 * max_size)
    opening = rng.uniform(0.5 * max_size, max_size)
    color = palette.FLESH_COLOR

    return models.Eyelids(pos, size, opening, color)


def random_radial_gradient(rng: Generator):
    stops = [
        color.Color(rng.random(), rng.random(), rng.random()),
        color.Color(rng.random(), rng.random(), rng.random()),
    ]
    return color.RadialGradient(stops)
