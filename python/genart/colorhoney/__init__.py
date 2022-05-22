import datetime as dt
import logging
import sys

import cairo

from genart.parse import parse_size

from .honey import ColorHoneyRenderer
from .tokki import ColorTokkiRenderer

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser(
        "colorhoney", help="Colorhoney writing system by Kim Godgul"
    )

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("--seed", type=int)
    parser.add_argument(
        "--tokki",
        action="store_true",
        help="Use the ColorTokki writing system instead of ColorHoney.",
    )

    parser.set_defaults(func=main)


def main(args, config):
    print("Reading input text from stdin:")
    text = sys.stdin.read().strip()

    width, height = parse_size(args.size)

    out_file = (
        config["output_dir"]
        / f"colorhoney_{dt.datetime.now().isoformat().replace(':', '-')}.svg"
    )
    surface = cairo.SVGSurface(str(out_file), width, height)

    if args.tokki:
        renderer_cls = ColorTokkiRenderer
    else:
        renderer_cls = ColorHoneyRenderer

    renderer = renderer_cls(surface)
    renderer.render(text)
    surface.finish()
