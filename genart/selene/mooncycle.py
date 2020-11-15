from math import pi

import cairo
from numpy.random import Generator

from genart.cairo_util import source
from genart.color import Color, LinearGradient
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
        draw_moon(ctx, x, y, (radius_outer - radius_inner) / 2.1, eclipse_pct)


def draw_moon(
    ctx: cairo.Context,
    pos_x: float,
    pos_y: float,
    radius: float,
    eclipse_pct: float = -1.0,
):
    moon_color = LinearGradient([Color(0.2, 0.2, 0.2), Color(0.7, 0.7, 0.7)])
    with source(
        ctx, moon_color.to_pattern(pos_x - radius, pos_y + radius, pos_x, pos_y)
    ):
        ctx.arc(pos_x, pos_y, radius, 0, 2 * pi)
        ctx.fill_preserve()
        ctx.clip()

    # crater_color = RadialGradient([Color(0.7, 0.7, 0.7), Color(0.2, 0.2, 0.2)])
    # crater_radius = radius / 10.0
    # with source(
    #     ctx,
    #     crater_color.to_pattern(pos_x, pos_y, crater_radius, crater_radius * 0.7),
    # ), operator(ctx, cairo.Operator.DARKEN):
    #     ctx.arc(pos_x, pos_y, crater_radius, 0, 2 * pi)
    #     ctx.fill()

    with source(ctx, Color(0.0, 0.0, 0.0).to_pattern()):
        eclipse_cx = pos_x + eclipse_pct * 2.0 * radius
        ctx.arc(eclipse_cx, pos_y, radius, 0, 2 * pi)
        ctx.fill()
        ctx.reset_clip()
