import datetime as dt
import logging

import cairo

from genart.fps import FPSCounter
from genart.util import parse_size

from .generator import generate_particles, make_superchamber
from .models import Particle
from .render import BubbleChamberRenderer
from .simulation import Simulation

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser(
        "cloudscript", help="A script made out of tiny bubble chambers."
    )

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-m", "--magnet-stddev", type=float, default=2.0)
    parser.add_argument("-f", "--friction-lambda", type=float, default=0.1)

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)

    chamber = make_superchamber(
        width, height, 10, 10, args.magnet_stddev, args.friction_lambda
    )
    particles = generate_particles(chamber)
    sim = Simulation(chamber, particles)

    out_file = config["output_dir"] / f"cloudscript_{dt.datetime.now().isoformat()}.svg"
    surface = cairo.SVGSurface(str(out_file), width, height)
    renderer = BubbleChamberRenderer(surface)

    fps = FPSCounter()

    sim.start()
    fps.start()
    while any(p.is_dirty for p in sim.particles):
        sim.step()
        renderer.render(sim)
        fps.frame_done()

    renderer.finalize(sim)
    surface.finish()
