import math
from dataclasses import dataclass
from typing import Optional

import numpy as np

import cairo
from genart.geom import circle_from_3_points


@dataclass
class Pupil:
    pos: np.array
    size: float

    def draw(self, ctx: cairo.Context):
        ctx.arc(self.pos[0], self.pos[1], self.size, 0, math.tau)
        ctx.fill()


@dataclass
class SlitPupil(Pupil):
    width: int
    rotation: float = 0.0

    def draw(self, ctx: cairo.Context):
        top_intersection = np.array([0, self.size])
        bottom_intersection = np.array([0, -self.size])
        right_edge = np.array([self.width / 2, 0])
        left_edge = np.array([-self.width / 2, 0])

        center_1, rad_1 = circle_from_3_points(
            top_intersection, right_edge, bottom_intersection
        )
        center_2, rad_2 = circle_from_3_points(
            top_intersection, left_edge, bottom_intersection
        )

        ctx.translate(self.pos[0], self.pos[1])
        ctx.rotate(self.rotation)
        ctx.arc(center_1[0], center_1[1], rad_1, 0, math.tau)
        ctx.clip()

        ctx.arc(center_2[0], center_2[1], rad_2, 0, math.tau)
        ctx.clip()

        ctx.paint()
        ctx.reset_clip()
        ctx.rotate(-self.rotation)
        ctx.translate(-self.pos[0], -self.pos[1])


@dataclass
class Iris:
    pos: np.array
    size: float
    color: cairo.Pattern

    def draw(self, ctx: cairo.Context):
        ctx.set_source(self.color)

        ctx.arc(self.pos[0], self.pos[1], self.size, 0, math.tau)
        ctx.fill()

        ctx.set_source_rgb(0.0, 0.0, 0.0)


@dataclass
class Eye:
    pos: np.array
    size: float
    pupil: Pupil
    iris: Optional[Iris]

    def draw(self, ctx: cairo.Context):
        ctx.arc(self.pos[0], self.pos[1], self.size, 0, math.tau)
        ctx.stroke()

        if self.iris:
            self.iris.draw(ctx)

        self.pupil.draw(ctx)
