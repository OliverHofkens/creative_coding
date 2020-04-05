from math import atan, inf
from typing import Sequence, Tuple

import numpy as np


def slope(p1: Sequence[float], p2: Sequence[float]) -> float:
    try:
        return (p2[1] - p1[1]) / (p2[0] - p1[0])
    except ZeroDivisionError:
        # No change in x means the slope is infinite:
        return inf


def distance(p1: np.array, p2: np.array) -> float:
    return np.linalg.norm(p1 - p2)


def angle_between_points(p1: np.array, p2: np.array) -> float:
    tan_alpha = slope(p1, p2)

    return atan(tan_alpha)


def circle_from_3_points(
    p1: np.array, p2: np.array, p3: np.array
) -> Tuple[np.array, float]:
    """
    See http://paulbourke.net/geometry/circlesphere/

    returns (center, radius)
    """
    # Calculate center:
    slope_a = slope(p1, p2)
    slope_b = slope(p2, p3)

    center_x = (
        (slope_a * slope_b * (p1[1] - p3[1]))
        + (slope_b * (p1[0] + p2[0]))
        - (slope_a * (p2[0] + p3[0]))
    ) / (2 * (slope_b - slope_a))

    center_y = (-1 / slope_a) * (center_x - ((p1[0] + p2[0]) / 2)) + (
        (p1[1] + p2[1]) / 2
    )

    center = np.array([center_x, center_y])

    # Calculate radius:
    radius = distance(p1, center)

    return (center, radius)
