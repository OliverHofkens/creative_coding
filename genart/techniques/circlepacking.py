"""Based on https://www.youtube.com/watch?v=QHEQuoIKgNE"""
from dataclasses import dataclass
from typing import List, Optional

import numpy as np
from numpy.random import Generator

from genart.fps import FPSCounter
from genart.geom import distance


@dataclass
class Circle:
    pos: np.array
    r: float
    growing: bool = True


def pack(
    rng: Generator,
    width: float,
    height: float,
    grow_rate: float,
    max_eyeballs: int,
    unbounded: bool = False,
) -> List[Circle]:
    circles: List[Circle] = []

    fps = FPSCounter()
    while True and len(circles) < max_eyeballs or any(c.growing for c in circles):
        if len(circles) < max_eyeballs:
            new = new_circle(rng, grow_rate, width, height, circles)
            if not new:
                return circles
            circles.append(new)

        for circle in circles:
            if circle.growing:
                grow_circle(circle, grow_rate, width, height, circles, unbounded)

        fps.frame_done()

    return circles


def new_circle(
    rng: Generator,
    radius: float,
    width: float,
    height: float,
    existing_circles: List[Circle],
) -> Optional[Circle]:
    attempts = 0
    new_circle = Circle(np.array([0.0, 0.0]), radius)

    while attempts <= 2000:
        new_circle.pos[0] = rng.uniform(new_circle.r, width - new_circle.r)
        new_circle.pos[1] = rng.uniform(new_circle.r, height - new_circle.r)

        for circle in existing_circles:
            if distance(new_circle.pos, circle.pos) < circle.r + new_circle.r:
                break
        else:
            return new_circle

        attempts += 1

    return None


def grow_circle(
    circle: Circle,
    rate: float,
    width: float,
    height: float,
    circles: List["Circle"],
    unbounded: bool = False,
):
    new_radius = circle.r + rate

    # Check if we go out of bounds:
    if not unbounded and (
        circle.pos[0] + new_radius >= width
        or circle.pos[0] - new_radius <= 0
        or circle.pos[1] + new_radius >= height
        or circle.pos[1] - new_radius <= 0
    ):
        circle.growing = False
        return

    for c in circles:
        if c is circle:
            continue
        if distance(circle.pos, c.pos) <= new_radius + c.r:
            circle.growing = False
            return

    circle.r = new_radius
