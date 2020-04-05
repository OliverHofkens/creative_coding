import datetime as dt
import logging

import cairo
import numpy as np
from genart.asparagus.generator import generate_asparagus
from genart.util import parse_size

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("asparagus", help="Asparagus fern growth patterns")

    parser.add_argument("-s", "--size", default="500x500")

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)

    out_file = config["output_dir"] / f"asparagus_{dt.datetime.now().isoformat()}.svg"
    surface = cairo.SVGSurface(str(out_file), width, height)
    ctx = cairo.Context(surface)

    plant = generate_asparagus(np.array([0.0, 0.0]), np.array([width, height]))
    plant.draw(ctx)

    surface.finish()
