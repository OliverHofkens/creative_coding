from typing import Tuple

import numpy as np
import pytest

from genart import geom


@pytest.mark.parametrize(
    "p1, p2, exp_slope",
    [
        ((0.0, 0.0), (1.0, 1.0), 1.0),
        ((0.0, 1.0), (1.0, 0.0), -1.0),
        ((0.0, 0.0), (1.0, 100.0), 100.0),
    ],
)
def test_slope(p1: Tuple[float, float], p2: Tuple[float, float], exp_slope: float):
    res = geom.slope(p1, p2)

    assert res == exp_slope


@pytest.mark.parametrize(
    "p1, p2, exp_distance",
    [((0.0, 0.0), (3.0, 4.0), 5.0)],
)
def test_distance(
    p1: Tuple[float, float], p2: Tuple[float, float], exp_distance: float
):
    res = geom.distance(np.array(p1), np.array(p2))

    assert res == exp_distance


@pytest.mark.parametrize(
    "p1, p2, p3, exp_center, exp_radius",
    [((0.0, 1.0), (1.0, 0.0), (0.0, -1.0), (0.0, 0.0), 1.0)],
)
def test_circle_from_3_points(
    p1: Tuple[float, float],
    p2: Tuple[float, float],
    p3: Tuple[float, float],
    exp_center: float,
    exp_radius: float,
):
    res_center, res_radius = geom.circle_from_3_points(p1, p2, p3)

    np.testing.assert_array_equal(res_center, exp_center)
    assert res_radius == exp_radius
