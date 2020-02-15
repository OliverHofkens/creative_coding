import math
from dataclasses import dataclass
from typing import Optional

import numpy as np

import cairo
from genart.cairo_util import rotation, source, translation
from genart.geom import circle_from_3_points


@dataclass
class Flesh:
    color: cairo.Pattern

    def draw_background(self, ctx: cairo.Context):
        with source(ctx, self.color):
            ctx.paint()


@dataclass
class Pupil:
    pos: np.array
    size: float

    def draw(self, ctx: cairo.Context, relative_to=(0, 0)):
        x, y = self.pos - relative_to

        ctx.arc(x, y, self.size, 0, math.tau)
        ctx.fill()


@dataclass
class SlitPupil(Pupil):
    width: int

    def draw(self, ctx: cairo.Context, relative_to=(0, 0)):
        pos = self.pos - relative_to

        top_intersection = pos + np.array([0, self.size])
        bottom_intersection = pos - np.array([0, self.size])
        right_edge = pos + np.array([self.width / 2, 0])
        left_edge = pos - np.array([self.width / 2, 0])

        center_1, rad_1 = circle_from_3_points(
            top_intersection, right_edge, bottom_intersection
        )
        center_2, rad_2 = circle_from_3_points(
            top_intersection, left_edge, bottom_intersection
        )

        ctx.arc(center_1[0], center_1[1], rad_1, 0, math.tau)
        ctx.clip()

        ctx.arc(center_2[0], center_2[1], rad_2, 0, math.tau)
        ctx.clip()

        ctx.paint()
        ctx.reset_clip()


@dataclass
class Iris:
    pos: np.array
    size: float
    color: cairo.Pattern

    def draw(self, ctx: cairo.Context, relative_to=(0, 0)):
        x, y = self.pos - relative_to

        with source(ctx, self.color):
            ctx.arc(x, y, self.size, 0, math.tau)
            ctx.fill()


@dataclass
class Eye:
    pos: np.array
    size: float
    color: cairo.Pattern
    pupil: Pupil
    iris: Optional[Iris]
    rotation: float = 0.0

    def draw(self, ctx: cairo.Context):
        with translation(ctx, self.pos[0], self.pos[1]):
            with rotation(ctx, self.rotation):
                with source(ctx, self.color):
                    ctx.arc(0, 0, self.size, 0, math.tau)
                    ctx.fill()

                if self.iris:
                    self.iris.draw(ctx, relative_to=self.pos)

                self.pupil.draw(ctx, relative_to=self.pos)
