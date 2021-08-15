from math import pi, tau

import cairo
from numpy.random import Generator

from genart.cairoctx import source
from genart.color import Color
from genart.geom import points_along_arc
from genart.jitter import jitter_points
from genart.techniques.pointillism import Pattern, PointLinearGradient


def draw_star_band(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    band_center = (radius_outer + radius_inner) / 2
    star_size = 0.2 * (radius_outer - radius_inner)
    n_stars = rng.integers(20, 50)

    ctx.arc(pos_x, pos_y, radius_outer, 0, tau)
    ctx.arc_negative(pos_x, pos_y, radius_inner, 0, -tau)
    ctx.stroke_preserve()
    ctx.clip_preserve()

    grad_offset_x = rng.uniform(radius_inner / 3.0, radius_inner)
    grad_offset_y = rng.uniform(radius_inner / 3.0, radius_inner)
    grad = PointLinearGradient([Color(0, 0, 0), Color(1, 1, 1, 0.0)], Pattern.PACKED)
    grad.fill(
        ctx,
        rng,
        pos_x - grad_offset_x,
        pos_y - grad_offset_y,
        pos_x + grad_offset_x,
        pos_y + grad_offset_y,
    )

    for x, y in jitter_points(
        points_along_arc(pos_x, pos_y, band_center, 0, tau, n_stars), rng, star_size
    ):
        draw_star(ctx, rng, x, y, star_size)

    ctx.reset_clip()


def draw_star(
    ctx: cairo.Context, rng: Generator, pos_x: float, pos_y: float, size: float
):
    spikes = rng.integers(5, 8)
    outer_points = list(
        jitter_points(
            points_along_arc(pos_x, pos_y, size, 0, tau, spikes), rng, 0.1 * size
        ),
    )
    inner_offset = pi / spikes
    inner_points = list(
        jitter_points(
            points_along_arc(
                pos_x, pos_y, size / 2.0, inner_offset, inner_offset + tau, spikes
            ),
            rng,
            0.1 * size,
        )
    )

    ctx.move_to(*inner_points[-1])
    for (ax, ay), (bx, by) in zip(outer_points, inner_points):
        ctx.line_to(ax, ay)
        ctx.line_to(bx, by)
    ctx.stroke_preserve()
    with source(ctx, Color(1.0, 1.0, 1.0).to_pattern()):
        ctx.fill()
