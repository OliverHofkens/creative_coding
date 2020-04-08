from dataclasses import dataclass, field
from math import cos, sin, sqrt
from typing import Iterator, List

import cairo
import numpy as np
from genart.geom import angle_between_points, distance


@dataclass
class Branch:
    start: np.array
    end: np.array
    thickness: float = 2.0
    children: List["Branch"] = field(default_factory=list)

    def draw(self, ctx: cairo.Context):
        ctx.move_to(self.start[0], self.start[1])
        ctx.set_line_width(self.thickness)
        ctx.line_to(self.end[0], self.end[1])
        ctx.stroke()

        for child in self.children:
            child.draw(ctx)

    def angle_at(self, at: np.array) -> float:
        """Find the angle of the branch at the given point"""
        # Optimization: for a straight branch, this angle is always the same
        return angle_between_points(self.start, self.end)

    def length(self) -> float:
        return distance(self.start, self.end)

    def walk_along(self, step_size: float) -> Iterator[np.array]:
        diff_x = self.start[0] - self.end[0]
        diff_y = self.start[1] - self.end[1]

        # Could use geom.distance here but we already have part of the calculation done...
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

    def branch_at(
        self, point: np.array, length: float, angle_to_self: float
    ) -> "Branch":

        # Add the rotation of the parent to angle_to_self
        global_angle = self.angle_at(point) + angle_to_self

        # Calculate the next point relative to the branching point
        diff_x = cos(global_angle) * length
        diff_y = sin(global_angle) * length

        # Convert the relative point to the absolute position
        endpoint = point + np.array([diff_x, diff_y])

        return Branch(point, endpoint, 0.75 * self.thickness)
