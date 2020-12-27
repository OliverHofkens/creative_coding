import cairo

from genart.cairoctx import operator, source
from genart.color import Color, RadialGradient


def draw_background(ctx: cairo.Context, width: int, height: int):
    # TODO: Add noise/cracks/stains
    bg_color = RadialGradient([Color(0.84, 0.81, 0.74), Color(0.55, 0.50, 0.36)])
    center_x = width / 2
    center_y = height / 2
    with source(ctx, bg_color.to_pattern(center_x, center_y, center_x)), operator(
        ctx, cairo.Operator.DARKEN
    ):
        ctx.paint()
