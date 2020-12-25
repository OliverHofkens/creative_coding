from dataclasses import dataclass
from enum import Enum
from itertools import cycle
from math import cos, pi, sin, tau
from typing import Iterator, Sequence, Tuple

import cairo
from numpy.random import Generator

from genart.cairoctx import rotation, source, translation
from genart.color import Color
from genart.geom import angle, distance
from genart.jitter import jitter_points

Point = Tuple[float, float]
Circle = Tuple[float, float, float]


def fill_orthogonal(
    rng: Generator,
    start_bound: Point,
    end_bound: Point,
    start_grad: Point,
    end_grad: Point,
    dot_r: float,
) -> Iterator[Circle]:
    rows = int((end_bound[1] - start_bound[1]) // (2 * dot_r))
    cols = int((end_bound[0] - start_bound[0]) // (2 * dot_r))

    for row in range(rows + 1):
        density = 1.0 - (row / rows)
        cy = start_bound[1] + (row * 2 * dot_r)

        for col in range(cols + 1):
            if rng.random() > density:
                continue

            cx = start_bound[0] + (col * 2 * dot_r)
            yield (cx, cy, dot_r)


def fill_orthogonal_jitter(
    rng,
    start_bound: Point,
    end_bound: Point,
    start_grad: Point,
    end_grad: Point,
    dot_r: float,
) -> Iterator[Circle]:
    yield from jitter_points(
        fill_orthogonal(rng, start_bound, end_bound, start_grad, end_grad, dot_r),
        rng,
        (dot_r / 3.0, dot_r / 3.0, dot_r / 5.0),
    )


def fill_packed(
    rng: Generator,
    start_bound: Point,
    end_bound: Point,
    start_grad: Point,
    end_grad: Point,
    dot_r: float,
) -> Iterator[Circle]:
    # A hexagonal pattern is the most closely packed arrangement of circles.
    # So a hexagon with side r describes the centers of 7 circles
    hex_side = 2 * dot_r
    colwidth = hex_side
    rowheight = sin(pi / 3) * hex_side
    stagger_x = cos(pi / 3) * hex_side
    rows = int((end_bound[1] - start_bound[1]) // rowheight)
    cols = int((end_bound[0] - start_bound[0]) // colwidth)

    for row, stagger in zip(range(rows + 1), cycle((0, stagger_x))):
        density = 1.0 - (row / rows)
        cy = start_bound[1] + (row * rowheight)

        for col in range(cols + 1):
            if rng.random() > density:
                continue

            cx = start_bound[0] + (col * colwidth) + stagger
            yield (cx, cy, dot_r)


def fill_packed_jitter(
    rng: Generator,
    start_bound: Point,
    end_bound: Point,
    start_grad: Point,
    end_grad: Point,
    dot_r: float,
) -> Iterator[Circle]:
    yield from jitter_points(
        fill_packed(rng, start_bound, end_bound, start_grad, end_grad, dot_r),
        rng,
        (dot_r / 4.0, dot_r / 4.0, dot_r / 5.0),
    )


class Pattern(Enum):
    ORTHO = "fill_orthogonal"
    ORTHO_JITTER = "fill_orthogonal_jitter"
    PACKED = "fill_packed"
    PACKED_JITTER = "fill_packed_jitter"

    @property
    def func(self):
        funcname = self.value
        return globals()[funcname]


@dataclass
class PointLinearGradient:
    stops: Sequence[Color]
    pattern: Pattern = Pattern.ORTHO
    dot_radius: float = 3.0

    def fill(
        self,
        ctx: cairo.Context,
        rng: Generator,
        x1: float,
        y1: float,
        x2: float,
        y2: float,
    ):
        with source(ctx, self.stops[-1].to_pattern()):
            ctx.fill_preserve()

        # Orient the canvas so that our gradient goes straight in direction of +Y.
        gradient_angle = angle((x1, y1), (x2, y2))
        with translation(ctx, -x1, -y1), rotation(ctx, gradient_angle), source(
            ctx, self.stops[0].to_pattern()
        ):
            # We translated and rotated the canvas, so our gradient control
            # vector is now from (0, 0) straight up:
            grad_control_y = distance((x1, y1), (x2, y2))
            # Get a bounding box of what needs to be filled:
            start_x, start_y, end_x, end_y = ctx.fill_extents()

            for cx, cy, cr in self.pattern.func(
                rng,
                (start_x, start_y),
                (end_x, end_y),
                (0, 0),
                (0, grad_control_y),
                self.dot_radius,
            ):
                ctx.new_path()
                ctx.arc(cx, cy, cr, 0, tau)
                ctx.fill()
