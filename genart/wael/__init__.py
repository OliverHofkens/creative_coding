import datetime as dt
import logging

import cairo
from genart.util import parse_size

from . import generator
from .circlepacker import pack

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("wael", help="He Who Sees and Is Not Seen")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-g", "--grow-rate", type=float, default=5.0)
    parser.add_argument("-m", "--max-eyeballs", type=int, default=1000)

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)

    out_file = config["output_dir"] / f"wael_{dt.datetime.now().isoformat()}.svg"
    surface = cairo.SVGSurface(str(out_file), width, height)
    context = cairo.Context(surface)

    circles = pack(width, height, args.grow_rate, args.max_eyeballs)
    eyes = [generator.random_eye(c.pos, c.r) for c in circles]
    for eye in eyes:
        eye.draw(context)

    surface.finish()
