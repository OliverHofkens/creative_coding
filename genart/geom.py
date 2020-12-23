from math import atan2, cos, sin
from typing import Iterator, Sequence, Tuple

import numpy as np


def slope(p1: Sequence[float], p2: Sequence[float]) -> float:
    return (p2[1] - p1[1]) / (p2[0] - p1[0])


def distance(p1: np.array, p2: np.array) -> float:
    return np.linalg.norm(p1 - p2)


def angle(p1: Sequence[float], p2: Sequence[float]) -> float:
    return atan2(p2[1] - p1[1], p2[0] - p1[0])


def unit_vector(p1: np.array, p2: np.array) -> np.array:
    diff = p1 - p2
    return diff / np.linalg.norm(diff)


def points_along_arc(
    center_x: float,
    center_y: float,
    radius: float,
    start_at: float,
    end_at: float,
    steps: int,
) -> Iterator[Tuple[float, float]]:
    angle_per_step = (end_at - start_at) / steps

    for i in range(steps):
        angle = start_at + i * angle_per_step
        x = center_x + cos(angle) * radius
        y = center_y + sin(angle) * radius
        yield (x, y)


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
