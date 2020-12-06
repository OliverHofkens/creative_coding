import datetime as dt
import logging

import cairo
from numpy.random import default_rng

from genart.util import parse_size

from . import generator, models
from .circlepacker import pack
from .palette import FLESH_COLOR

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("wael", help="He Who Sees and Is Not Seen")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-g", "--grow-rate", type=float, default=5.0)
    parser.add_argument("-m", "--max-eyeballs", type=int, default=1000)
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)
    rng = default_rng(args.seed)

    out_file = (
        config["output_dir"]
        / f"wael_{dt.datetime.now().isoformat().replace(':', '-')}.png"
    )
    surface = cairo.ImageSurface(cairo.Format.ARGB32, width, height)
    context = cairo.Context(surface)

    circles = pack(rng, width, height, args.grow_rate, args.max_eyeballs)
    eyes = [generator.random_eye(rng, c.pos, c.r) for c in circles]
    flesh = models.Flesh(FLESH_COLOR)

    flesh.draw(context)
    for eye in eyes:
        eye.draw(context)

    surface.write_to_png(out_file)
