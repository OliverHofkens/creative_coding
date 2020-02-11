import datetime as dt
import logging

import cairo
from genart.util import parse_size

from . import generator

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("wael", help="He Who Sees and Is Not Seen")

    parser.add_argument("-s", "--size", default="500x500")

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)

    out_file = config["output_dir"] / f"wael_{dt.datetime.now().isoformat()}.svg"
    surface = cairo.SVGSurface(str(out_file), width, height)
    context = cairo.Context(surface)

    eyes = generator.fill(width, height)
    for eye in eyes:
        eye.draw(context)

    surface.finish()
