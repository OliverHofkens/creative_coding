import datetime as dt
import logging
from math import cos, pi, sin, tau

import cairo
from numpy.random import default_rng

from genart.color import Color
from genart.parse import parse_size

from ._utils import draw_grid
from .circlepacking import pack
from .pointillism import Pattern, PointLinearGradient

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("technique", help="Visualize techniques")
    subparsers = parser.add_subparsers(dest="technique", required=True)
    _register_circlepacking_parser(subparsers)
    _register_pointillism_parser(subparsers)


def _register_circlepacking_parser(subparsers):
    parser = subparsers.add_parser("circlepacking")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-g", "--grow-rate", type=float, default=5.0)
    parser.add_argument("-m", "--max-circles", type=int, default=1000)
    parser.add_argument("-u", "--unbounded", action="store_true")
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=_circlepacking)


def _register_pointillism_parser(subparsers):
    parser = subparsers.add_parser("pointillism")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=_pointillism)


def _circlepacking(args, config):
    width, height = parse_size(args.size)
    rng = default_rng(args.seed)

    out_file = (
        config["output_dir"]
        / f"technique_circlepacking_{dt.datetime.now().isoformat()}.svg"
    )
    surface = cairo.SVGSurface(str(out_file), width, height)

    ctx = cairo.Context(surface)
    circles = pack(rng, width, height, args.grow_rate, args.max_circles, args.unbounded)
    for c in circles:
        ctx.arc(*c.pos, c.r, 0, tau)
        ctx.stroke()

    surface.finish()


def _pointillism(args, config):
    width, height = parse_size(args.size)
    rng = default_rng(args.seed)

    out_file = (
        config["output_dir"]
        / f"technique_pointillism_{dt.datetime.now().isoformat()}.svg"
    )
    surface = cairo.SVGSurface(str(out_file), width, height)

    ctx = cairo.Context(surface)

    # Divide the canvas in squares to show off different styles:
    ROWS = 3
    COLS = len(Pattern.__members__)
    draw_grid(ctx, width, height, ROWS, COLS)
    rowheight = height // ROWS
    colwidth = width // COLS
    radius = min(rowheight, colwidth) / 2
    square_size = radius / 3

    for rown in range(ROWS):
        starty = rown * rowheight
        endy = starty + rowheight
        cy = (starty + endy) / 2
        grad_angle = rown * (pi / 2.0 / (ROWS - 1))
        offset_x = cos(grad_angle) * radius
        offset_y = sin(grad_angle) * radius

        for coln, pat in enumerate(Pattern):
            startx = coln * colwidth
            endx = startx + colwidth
            cx = (startx + endx) / 2

            ctx.arc(cx, cy, radius, 0, tau)
            ctx.clip()

            # Cut out a square to observe clipping behavior:
            # ctx.rectangle(
            #     cx - (square_size / 2), cy - (square_size / 2), square_size, square_size
            # )
            # ctx.clip()

            # Finally, fill the outer circle:
            ctx.arc(cx, cy, radius, 0, tau)
            grad = PointLinearGradient(
                [Color(0.0, 0.0, 0.0), Color(1.0, 1.0, 1.0)], pat
            )
            grad.fill(ctx, rng, cx, cy, cx + offset_x, cy + offset_y)

            ctx.reset_clip()

    surface.finish()
