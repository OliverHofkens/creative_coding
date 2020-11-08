import datetime as dt
import logging
from math import acos, atan2, cos, pi, sin
from operator import add, sub

import cairo
from numpy.random import default_rng

from genart.cairo_util import operator, source
from genart.color import Color, LinearGradient, RadialGradient
from genart.geom import points_along_arc
from genart.util import parse_size

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("selene", help="O Chaire Selene")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)
    rng = default_rng(args.seed)

    out_file = config["output_dir"] / f"selene_{dt.datetime.now().isoformat()}.svg"
    surface = cairo.SVGSurface(str(out_file), width, height)
    ctx = cairo.Context(surface)

    bg = RadialGradient([Color(0.7, 0.7, 0.7), Color(0.2, 0.2, 0.2)])
    with source(ctx, bg.to_pattern(width / 2, height / 2, width / 2)):
        ctx.paint()

    draw_crown(ctx, width / 2, height / 3, width / 10)

    n_moons = rng.integers(6, 12)
    for i, (x, y) in enumerate(
        points_along_arc(width / 2, height / 2, width / 2.8, 0, 2 * pi, n_moons)
    ):
        pct_done = i / n_moons
        eclipse_pct = (pct_done * 2.0) - 1.0
        draw_moon(ctx, x, y, width / (1.2 * n_moons), eclipse_pct)

    CAL_OUTER_R = width / 4
    CAL_INNER_R = 0.9 * CAL_OUTER_R
    TANGENT_INNER_R = 0.5 * CAL_INNER_R

    draw_circular_calendar(ctx, width / 2, height / 2, CAL_OUTER_R, CAL_INNER_R)
    draw_tangents(ctx, width / 2, height / 2, CAL_INNER_R, TANGENT_INNER_R)
    draw_dodecahedron(ctx, width / 2, height / 2, 0.9 * TANGENT_INNER_R)

    surface.finish()


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


def draw_crown(ctx: cairo.Context, pos_x: float, pos_y: float, radius: float):
    ctx.arc(pos_x, pos_y, radius, 0, 2 * pi)
    ctx.stroke_preserve()
    ctx.clip()

    crown_eclipse = -0.2

    eclipse_cy = pos_y + crown_eclipse * 2.0 * radius
    ctx.arc(pos_x, eclipse_cy, radius, 0, 2 * pi)
    ctx.fill()
    ctx.reset_clip()


def draw_circular_calendar(
    ctx: cairo.Context,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
    chunks: int = 12,
):
    ctx.arc(pos_x, pos_y, radius_outer, 0, 2 * pi)
    ctx.stroke_preserve()

    ctx.arc(pos_x, pos_y, 0.9 * radius_outer, 0, 2 * pi)
    ctx.stroke()

    for (start_x, start_y), (end_x, end_y) in zip(
        points_along_arc(pos_x, pos_y, radius_inner, 0, 2 * pi, chunks),
        points_along_arc(pos_x, pos_y, radius_outer, 0, 2 * pi, chunks),
    ):
        ctx.move_to(start_x, start_y)
        ctx.line_to(end_x, end_y)
        ctx.stroke()


def draw_tangents(
    ctx: cairo.Context,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
    origin_points: int = 24,
):
    for start_x, start_y in points_along_arc(
        pos_x, pos_y, radius_outer, 0, 2 * pi, origin_points
    ):
        angle_c_to_tangent = acos(radius_inner / radius_outer)
        angle_c_to_point = atan2(start_y - pos_y, start_x - pos_x)

        # 2 tangents:
        for op in (add, sub):
            angle_tangent = op(angle_c_to_point, angle_c_to_tangent)
            tangent_x = pos_x + radius_inner * cos(angle_tangent)
            tangent_y = pos_y + radius_inner * sin(angle_tangent)
            ctx.move_to(start_x, start_y)
            ctx.line_to(tangent_x, tangent_y)
            ctx.stroke()


def draw_dodecahedron(ctx: cairo.Context, pos_x: float, pos_y: float, radius: float):
    fs_color = RadialGradient([Color(0.7, 0.7, 0.7), Color(0.5, 0.5, 0.5)])
    with source(ctx, fs_color.to_pattern(pos_x, pos_y, radius)):
        # Outer boundary
        outer_points = list(
            p for p in points_along_arc(pos_x, pos_y, radius, 0, 2 * pi, 10)
        )
        for prev_index, (bx, by) in enumerate(outer_points, -1):
            ax, ay = outer_points[prev_index]
            ctx.move_to(ax, ay)
            ctx.line_to(bx, by)
            ctx.stroke()

        # Frontside inner pentagon
        inner_fs_points = list(
            p for p in points_along_arc(pos_x, pos_y, 0.6 * radius, 0, 2 * pi, 5)
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
            p
            for p in points_along_arc(
                pos_x, pos_y, 0.6 * radius, offset, offset + 2 * pi, 5
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
