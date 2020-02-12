import math
import random
from dataclasses import fields
from typing import Optional

import numpy as np

import cairo

from . import models

PUPIL_CHOICES = [models.Pupil, *models.Pupil.__subclasses__()]


def random_eye(pos: np.ndarray, size: float) -> models.Eye:
    iris = random_or_no_iris(pos, size)
    max_pupil_size = iris.size if iris else size
    pupil = random_pupil(pos, max_pupil_size)

    return models.Eye(pos, size, pupil, iris)


def random_pupil(pos: np.ndarray, max_size: float) -> models.Pupil:
    cls = random.choice(PUPIL_CHOICES)
    flds = {f.name for f in fields(cls)}

    if cls is models.SlitPupil:
        min_size = 0.5 * max_size
        size = random.triangular(min_size, max_size)
    else:
        min_size = 1.0
        max_size = 0.8 * max_size
        size = random.triangular(min_size, max_size)

    kwargs = dict(pos=pos, size=size)

    if "width" in flds:
        kwargs["width"] = random.uniform(1.0, size)

    if "rotation" in flds:
        kwargs["rotation"] = random.uniform(0.0, math.pi)

    return cls(**kwargs)


def random_or_no_iris(pos: np.ndarray, max_size: float) -> Optional[models.Iris]:
    has_iris = random.uniform(0, 1) > 0.5
    if not has_iris:
        return None

    size = random.triangular(max_size / 2.0, max_size)
    color = random_radial_gradient(pos, size)
    return models.Iris(pos, size, color)


def random_radial_gradient(pos: np.array, size: float):
    pat = cairo.RadialGradient(pos[0], pos[1], 1.0, pos[0], pos[1], size)
    pat.add_color_stop_rgb(1, random.random(), random.random(), random.random())
    pat.add_color_stop_rgb(0, random.random(), random.random(), random.random())
    return pat
