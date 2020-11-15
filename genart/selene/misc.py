from math import acos, atan2, cos, pi, sin
from operator import add, sub

import cairo
from numpy.random import Generator

from genart.geom import points_along_arc


def draw_tangents(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    origin_points = rng.choice([12, 24, 36, 48, 60])

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
