from dataclasses import dataclass
from enum import Enum
from math import tau
from typing import Iterator, Sequence, Tuple

import cairo
from numpy.random import Generator

from genart.cairoctx import rotation, source
from genart.color import Color
from genart.geom import angle
from genart.jitter import jitter_points

Circle = Tuple[float, float, float]


def fill_orthogonal(
    rng: Generator, x1: float, y1: float, x2: float, y2: float, dot_r: float
) -> Iterator[Circle]:
    rows = int((y2 - y1) // (2 * dot_r))
    cols = int((x2 - x1) // (2 * dot_r))

    for row in range(rows + 1):
        density = 1.0 - (row / rows)
        cy = y1 + (row * 2 * dot_r)

        for col in range(cols + 1):
            if rng.random() > density:
                continue

            cx = x1 + (col * 2 * dot_r)
            yield (cx, cy, dot_r)


def fill_orthogonal_jitter(
    rng: Generator, x1: float, y1: float, x2: float, y2: float, dot_r: float
) -> Iterator[Circle]:
    yield from jitter_points(
        fill_orthogonal(rng, x1, y1, x2, y2, dot_r),
        rng,
        (dot_r / 3.0, dot_r / 3.0, dot_r / 5.0),
    )


class Pattern(Enum):
    ORTHO = "fill_orthogonal"
    ORTHO_JITTER = "fill_orthogonal_jitter"

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
        with rotation(ctx, gradient_angle), source(ctx, self.stops[0].to_pattern()):
            # Get a bounding box of what needs to be filled:
            start_x, start_y, end_x, end_y = ctx.fill_extents()
            ctx.new_path()

            for cx, cy, cr in self.pattern.func(
                rng, start_x, start_y, end_x, end_y, self.dot_radius
            ):
                ctx.arc(cx, cy, cr, 0, tau)
                ctx.fill()
