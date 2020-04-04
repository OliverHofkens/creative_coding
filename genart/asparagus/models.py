from dataclasses import dataclass
from math import sqrt
from typing import Iterator

import cairo
import numpy as np


@dataclass
class Branch:
    start: np.array
    end: np.array

    def draw(self, ctx: cairo.Context):
        ctx.move_to(self.start[0], self.start[1])
        ctx.line_to(self.end[0], self.end[1])
        ctx.stroke()

    def walk_along(self, step_size: float) -> Iterator[np.array]:
        diff_x = self.start[0] - self.end[0]
        diff_y = self.start[1] - self.end[1]
        total_distance = sqrt(diff_x ** 2 + diff_y ** 2)

        steps_taken = 0

        while True:
            next_offset = steps_taken * step_size
            if next_offset > total_distance:
                break

            next_x = self.start[0] - next_offset * diff_x / total_distance
            next_y = self.start[1] - next_offset * diff_y / total_distance

            yield np.array([next_x, next_y])

            steps_taken += 1
