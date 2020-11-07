import datetime as dt
import logging
from math import pi

import cairo
from numpy.random import default_rng

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

    MOONS = 8
    for i, (x, y) in enumerate(
        points_along_arc(width / 2, height / 2, width / 3, 0, 2 * pi, MOONS), 1
    ):
        pct_done = i / MOONS
        eclipse_pct = (pct_done * 2.0) - 1.0
        draw_moon(ctx, x, y, width / 10, eclipse_pct)

    surface.finish()


def draw_moon(
    ctx: cairo.Context,
    pos_x: float,
    pos_y: float,
    radius: float,
    eclipse_pct: float = -1.0,
):
    ctx.arc(pos_x, pos_y, radius, 0, 2 * pi)
    ctx.stroke_preserve()
    ctx.clip()

    eclipse_cx = pos_x + eclipse_pct * 2.0 * radius
    ctx.arc(eclipse_cx, pos_y, radius, 0, 2 * pi)
    ctx.fill()
    ctx.reset_clip()
