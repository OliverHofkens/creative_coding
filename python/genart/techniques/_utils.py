import cairo

from genart import cairoctx
from genart.color import Color


def draw_grid(ctx: cairo.Context, width: float, height: float, rows: int, cols: int):
    rowheight = height // rows
    colwidth = width // cols

    with cairoctx.source(ctx, Color(0.5, 0.5, 0.5).to_pattern()):
        for col in range(1, cols + 1):
            x = col * colwidth
            ctx.move_to(x, 0)
            ctx.line_to(x, height)
            ctx.stroke()

        for row in range(1, rows + 1):
            y = row * rowheight
            ctx.move_to(0, y)
            ctx.line_to(width, y)
            ctx.stroke()
