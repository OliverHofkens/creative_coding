import random

import numpy as np

from genart.wael.circlepacker import Circle, grow_circle, new_circle, pack


def test_new_circle_on_blank_canvas():
    res = new_circle(1.0, 10.0, 10.0, [])

    assert isinstance(res, Circle)


def test_new_circle_on_full_canvas():
    existing_circle = Circle(np.array([5.0, 5.0]), 10.0)
    res = new_circle(1.0, 10.0, 10.0, [existing_circle])

    assert res is None


def test_grow_circle_on_black_canvas():
    circle = Circle(np.array([5.0, 5.0]), 1.0)

    grow_circle(circle, 1.0, 10.0, 10.0, [])

    assert circle.r == 2.0
    assert circle.growing


def test_grow_circle_on_full_canvas():
    existing_circle = Circle(np.array([5.0, 5.0]), 10.0)
    circle = Circle(np.array([5.0, 5.0]), 1.0)

    grow_circle(circle, 1.0, 10.0, 10.0, [existing_circle])

    assert circle.r == 1.0
    assert not circle.growing


def test_pack():
    random.seed(0)

    # On a 100x100 canvas, we expect to easily hit the 10 circle limit:
    circles = pack(100.0, 100.0, 1.0, 10)

    assert len(circles) == 10
