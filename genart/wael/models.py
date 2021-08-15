import math
from dataclasses import dataclass
from typing import Optional

import cairo
import numpy as np

from genart import color
from genart.cairoctx import operator, rotation, source, translation
from genart.geom import circle_from_3_points


@dataclass
class Flesh:
    color: color.Color

    def draw(self, ctx: cairo.Context):
        with source(ctx, self.color.to_pattern()):
            ctx.paint()


@dataclass
class Eyelids:
    pos: np.array
    size: float
    opening: float
    color: color.Color

    def draw(self, ctx: cairo.Context, eye_radius: float, relative_to=(0, 0)):
        pos = self.pos - relative_to

        top_intersection = pos + np.array([0, self.opening / 2])
        bottom_intersection = pos - np.array([0, self.opening / 2])
        right_point = pos + np.array([self.size, 0])
        left_point = pos - np.array([self.size, 0])

        center_1, rad_1 = circle_from_3_points(
            left_point, top_intersection, right_point
        )
        center_2, rad_2 = circle_from_3_points(
            left_point, bottom_intersection, right_point
        )

        # Eyelid 1:
        ctx.push_group()
        with source(ctx, self.color.to_pattern()):
            # Restrict drawing area to eyeball:
            ctx.arc(pos[0], pos[1], eye_radius + 1, 0, math.tau)
            ctx.clip()
            ctx.paint()
            # Sub circle 1:
            with operator(ctx, cairo.Operator.CLEAR):
                ctx.arc(center_1[0], center_1[1], rad_1, 0, math.tau)
                ctx.fill()
            ctx.reset_clip()
        with source(ctx, ctx.pop_group()):
            ctx.paint()

        # Eyelid 2:
        ctx.push_group()
        with source(ctx, self.color.to_pattern()):
            # Restrict drawing area to eyeball:
            ctx.arc(pos[0], pos[1], eye_radius + 1, 0, math.tau)
            ctx.clip()
            ctx.paint()
            # Sub circle 2:
            with operator(ctx, cairo.Operator.CLEAR):
                ctx.arc(center_2[0], center_2[1], rad_2, 0, math.tau)
                ctx.fill()
            ctx.reset_clip()
        with source(ctx, ctx.pop_group()):
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
    color: color.RadialGradient

    def draw(self, ctx: cairo.Context, relative_to=(0, 0)):
        x, y = self.pos - relative_to
        pat = self.color.to_pattern(x, y, self.size)

        with source(ctx, pat):
            ctx.arc(x, y, self.size, 0, math.tau)
            ctx.fill()


@dataclass
class Eye:
    pos: np.array
    size: float
    color: color.Color
    pupil: Pupil
    iris: Optional[Iris]
    eyelids: Optional[Eyelids]
    rotation: float = 0.0

    def draw(self, ctx: cairo.Context):
        with translation(ctx, self.pos[0], self.pos[1]):
            with rotation(ctx, self.rotation):
                with source(ctx, self.color.to_pattern()):
                    ctx.arc(0, 0, self.size, 0, math.tau)
                    ctx.fill()

                if self.iris:
                    self.iris.draw(ctx, relative_to=self.pos)

                self.pupil.draw(ctx, relative_to=self.pos)

                if self.eyelids:
                    self.eyelids.draw(ctx, self.size, relative_to=self.pos)
