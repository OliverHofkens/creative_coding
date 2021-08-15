from functools import wraps
from math import atan2, cos, sin
from typing import Iterator, Sequence, Tuple

import numpy as np


def _ensure_ndarray(func):
    def _convert(obj) -> np.ndarray:
        if not isinstance(obj, np.ndarray):
            return np.array(obj)
        return obj

    @wraps(func)
    def wrapper(*args, **kwargs):
        args = [_convert(a) for a in args]
        kwargs = {k: _convert(a) for k, a in kwargs.items()}
        return func(*args, **kwargs)

    return wrapper


def slope(p1: Sequence[float], p2: Sequence[float]) -> float:
    return (p2[1] - p1[1]) / (p2[0] - p1[0])


@_ensure_ndarray
def distance(p1: np.ndarray, p2: np.ndarray) -> float:
    return np.linalg.norm(p1 - p2)


def angle(p1: Sequence[float], p2: Sequence[float]) -> float:
    return atan2(p2[1] - p1[1], p2[0] - p1[0])


@_ensure_ndarray
def unit_vector(p1: np.ndarray, p2: np.ndarray) -> np.ndarray:
    diff = p1 - p2
    return diff / np.linalg.norm(diff)


@_ensure_ndarray
def projected_point_on_line(
    line_start: np.ndarray, line_end: np.ndarray, p: np.ndarray
) -> np.ndarray:
    vec_line = line_end - line_start
    vec_line_len = np.linalg.norm(vec_line)
    linestart_to_p = p - line_start
    linestart_to_p_len = np.linalg.norm(linestart_to_p)
    dp = np.dot(vec_line, linestart_to_p)

    cosine = dp / (vec_line_len * linestart_to_p_len)
    proj_len = cosine * linestart_to_p_len
    proj = line_start + (proj_len * vec_line) / vec_line_len

    return proj


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
    p1: np.ndarray, p2: np.ndarray, p3: np.ndarray
) -> Tuple[np.ndarray, float]:
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
