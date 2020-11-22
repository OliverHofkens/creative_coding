from math import pi

import cairo
from numpy.random import Generator

from genart.cairo_util import source
from genart.color import Color
from genart.geom import points_along_arc


def draw_moon_cycles(
    ctx: cairo.Context,
    rng: Generator,
    center_x: float,
    center_y: float,
    radius_outer: float,
    radius_inner: float,
):
    n_moons = rng.integers(6, 12)
    for i, (x, y) in enumerate(
        points_along_arc(
            center_x, center_y, (radius_outer + radius_inner) / 2.0, 0, 2 * pi, n_moons
        )
    ):
        pct_done = i / n_moons
        eclipse_pct = (pct_done * 2.0) - 1.0
        draw_moon(ctx, x, y, (radius_outer - radius_inner) / 2.3, eclipse_pct)


def draw_moon(
    ctx: cairo.Context,
    pos_x: float,
    pos_y: float,
    radius: float,
    eclipse_pct: float = -1.0,
):
    moon_color = Color(0.9, 0.9, 0.9)

    with source(
        ctx,
        moon_color.to_pattern(),
    ):
        ctx.arc(pos_x, pos_y, radius, 0, 2 * pi)
        ctx.fill_preserve()
        ctx.clip()

    with source(ctx, Color(0.0, 0.0, 0.0).to_pattern()):
        eclipse_cx = pos_x + eclipse_pct * 2.0 * radius
        ctx.arc(eclipse_cx, pos_y, radius, 0, 2 * pi)
        ctx.fill()
        ctx.reset_clip()
