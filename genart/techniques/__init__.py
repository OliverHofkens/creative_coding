import datetime as dt
import logging
from math import tau

import cairo
from numpy.random import default_rng

from genart.parse import parse_size

from .circlepacking import pack

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("technique", help="Visualize techniques")
    subparsers = parser.add_subparsers(dest="technique", required=True)
    _register_circlepacking_parser(subparsers)


def _register_circlepacking_parser(subparsers):
    parser = subparsers.add_parser("circlepacking")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-g", "--grow-rate", type=float, default=5.0)
    parser.add_argument("-m", "--max-circles", type=int, default=1000)
    parser.add_argument("-u", "--unbounded", action="store_true")
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=_circlepacking)


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
