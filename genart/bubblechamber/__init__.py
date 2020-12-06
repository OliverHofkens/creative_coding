import datetime as dt
import logging

import cairo
from numpy.random import default_rng

from genart.fps import FPSCounter
from genart.util import parse_size

from .generator import generate_particles, make_chamber
from .render import BubbleChamberRenderer
from .simulation import Simulation

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("bubblechamber", help="Bubble chamber simulation")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-m", "--magnet", type=float)
    parser.add_argument("-f", "--friction", type=float)
    parser.add_argument("-n", "--n-particles", type=int)
    parser.add_argument("--seed", type=int)

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)
    rng = default_rng(args.seed)

    sim = Simulation(
        make_chamber(rng, args.magnet, args.friction),
        generate_particles(rng, width, height, args.n_particles),
        rng,
    )

    out_file = (
        config["output_dir"]
        / f"bubblechamber_{dt.datetime.now().isoformat().reformat(':', '-')}.svg"
    )
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
