from math import pi, tau

import cairo
from numpy.random import Generator

from genart.cairoctx import source
from genart.color import Color, RadialGradient
from genart.geom import points_along_arc


def no_core(*args, **kwargs):
    pass


def draw_crown(ctx: cairo.Context, pos_x: float, pos_y: float, radius: float):
    ctx.arc(pos_x, pos_y, radius, 0, tau)
    ctx.stroke_preserve()
    ctx.clip()

    crown_eclipse = -0.2

    eclipse_cy = pos_y + crown_eclipse * 2.0 * radius
    ctx.arc(pos_x, eclipse_cy, radius, 0, tau)
    ctx.fill()
    ctx.reset_clip()


def draw_dodecahedron(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius: float,
    rotation: float = 0,
):
    fs_color = RadialGradient([Color(0.7, 0.7, 0.7), Color(0.5, 0.5, 0.5)])
    with source(ctx, fs_color.to_pattern(pos_x, pos_y, radius)):
        # Outer boundary
        outer_points = list(
            points_along_arc(pos_x, pos_y, radius, rotation, rotation + tau, 10)
        )
        for prev_index, (bx, by) in enumerate(outer_points, -1):
            ax, ay = outer_points[prev_index]
            ctx.move_to(ax, ay)
            ctx.line_to(bx, by)
            ctx.stroke()

        # Frontside inner pentagon
        inner_fs_points = list(
            points_along_arc(pos_x, pos_y, 0.6 * radius, rotation, rotation + tau, 5)
        )
        for prev_index, (bx, by) in enumerate(inner_fs_points, -1):
            ax, ay = inner_fs_points[prev_index]
            ctx.move_to(ax, ay)
            ctx.line_to(bx, by)
            ctx.stroke()

        # Outer to frontside inner
        for (ax, ay), (bx, by) in zip(inner_fs_points, outer_points[::2]):
            ctx.move_to(ax, ay)
            ctx.line_to(bx, by)
            ctx.stroke()

    bs_color = RadialGradient([Color(0.5, 0.5, 0.5), Color(0.3, 0.3, 0.3)])
    with source(ctx, bs_color.to_pattern(pos_x, pos_y, radius)):
        # backside inner pentagon
        offset = pi / 5
        inner_bs_points = list(
            points_along_arc(
                pos_x,
                pos_y,
                0.6 * radius,
                rotation + offset,
                rotation + offset + tau,
                5,
            )
        )
        for prev_index, (bx, by) in enumerate(inner_bs_points, -1):
            ax, ay = inner_bs_points[prev_index]
            ctx.move_to(ax, ay)
            ctx.line_to(bx, by)
            ctx.stroke()
        # Outer to backside inner
        for (ax, ay), (bx, by) in zip(inner_bs_points, outer_points[1::2]):
            ctx.move_to(ax, ay)
            ctx.line_to(bx, by)
            ctx.stroke()
