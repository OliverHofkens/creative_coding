"""Based on https://www.youtube.com/watch?v=QHEQuoIKgNE"""
import random
from dataclasses import dataclass
from typing import List, Optional

import numpy as np

from genart.fps import FPSCounter
from genart.geom import distance


@dataclass
class Circle:
    pos: np.array
    r: float = 7.5
    growing: bool = True

    def grow(self, width: float, height: float, circles: List["Circle"]):
        # Check if we go out of bounds:
        if (
            self.pos[0] + self.r >= width
            or self.pos[0] - self.r <= 0
            or self.pos[1] + self.r >= height
            or self.pos[1] - self.r <= 0
        ):
            self.growing = False
            return

        for c in circles:
            if c is self:
                continue
            if distance(self.pos, c.pos) <= self.r + c.r:
                self.growing = False
                return

        self.r += 1.0


def pack(width: float, height: float) -> List[Circle]:
    circles: List[Circle] = []

    fps = FPSCounter()
    while True and len(circles) < 1000:
        new = new_circle(width, height, circles)
        if not new:
            return circles
        circles.append(new)

        for circle in circles:
            if circle.growing:
                circle.grow(width, height, circles)

        fps.frame_done()

    return circles


def new_circle(
    width: float, height: float, existing_circles: List[Circle]
) -> Optional[Circle]:
    attempts = 0
    new_circle = Circle(np.array([0.0, 0.0]))

    while attempts <= 1000:
        new_circle.pos[0] = random.uniform(new_circle.r, width - new_circle.r)
        new_circle.pos[1] = random.uniform(new_circle.r, height - new_circle.r)

        for circle in existing_circles:
            if distance(new_circle.pos, circle.pos) < circle.r + new_circle.r:
                break
        else:
            return new_circle

        attempts += 1

    return None
