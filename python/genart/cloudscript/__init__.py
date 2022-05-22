import datetime as dt
import logging
import sys

import cairo
from numpy.random import default_rng

from genart.fps import FPSCounter
from genart.parse import parse_size

from .generator import generate_particles, make_superchamber
from .layout import layout_text
from .render import BubbleChamberRenderer
from .simulation import Simulation

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser(
        "cloudscript", help="A script made out of tiny bubble chambers"
    )

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("--max-linewidth", type=float, default=2.5)
    parser.add_argument("-g", "--grid", action="store_true")
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=main)


def main(args, config):
    print("Reading input text from stdin:")
    text = sys.stdin.read().strip()

    width, height = parse_size(args.size)

    layout = layout_text(text, 1)

    rng = default_rng(args.seed)
    chamber = make_superchamber(rng, width, height, layout)
    particles = generate_particles(rng, chamber)

    sim = Simulation(chamber, particles)

    out_file = (
        config["output_dir"]
        / f"cloudscript_{dt.datetime.now().isoformat().replace(':', '-')}.svg"
    )
    surface = cairo.SVGSurface(str(out_file), width, height)
    renderer = BubbleChamberRenderer(surface, max_linewidth=args.max_linewidth)

    if args.grid:
        renderer.add_grid(width, height, chamber.rows, chamber.columns)

    fps = FPSCounter()

    sim.start()
    fps.start()
    while any(p.is_dirty for p in sim.particles):
        sim.step()
        renderer.render(sim)
        fps.frame_done()

    renderer.finalize(sim)
    surface.finish()
