import datetime as dt
import logging

import cairo
from numpy.random import Generator, default_rng

from genart.cairo_util import source
from genart.color import Color, RadialGradient
from genart.selene import calendar, cores, misc, mooncycle
from genart.util import parse_size
from genart.wael import circlepacker

log = logging.getLogger(__name__)

CONCENTRICS = [
    mooncycle.draw_moon_cycles,
    misc.draw_tangents,
    calendar.draw_circular_roman,
    calendar.draw_circular_astrological_planets,
    calendar.draw_circular_zodiac,
]

CORES = [cores.no_core, cores.draw_dodecahedron]


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

    n_circles = rng.integers(4, 12)
    circles = circlepacker.pack(rng, width, height, 10.0, n_circles)

    for circle in circles:
        randomly_fill_circle(ctx, rng, circle.pos[0], circle.pos[1], circle.r)

    surface.finish()


def randomly_fill_circle(
    ctx: cairo.Context, rng: Generator, center_x: float, center_y: float, radius: float
):
    n_concentrics = rng.integers(2, 4)
    concentrics = rng.choice(CONCENTRICS, size=n_concentrics)

    radius_left = radius
    for conc in concentrics:
        width = rng.uniform(0.1, 0.4) * radius_left
        rad_outer = radius_left
        rad_inner = radius_left - width
        radius_left -= width

        conc(ctx, rng, center_x, center_y, rad_outer, rad_inner)

    core_size = rng.uniform(0.5, 0.9) * radius_left
    core = rng.choice(CORES)
    core(ctx, rng, center_x, center_y, core_size)
