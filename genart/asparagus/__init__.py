import datetime as dt
import logging
from math import pi

import cairo
import numpy as np
from genart.asparagus.models import Branch
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

    main_branch = Branch((0, 0), (width, height))
    main_branch.draw(ctx)

    for bud in main_branch.walk_along(10.0):
        child = main_branch.branch_at(bud, 10.0, pi / 2)
        child.draw(ctx)

    surface.finish()
