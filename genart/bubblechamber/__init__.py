import datetime as dt
import logging

import cairo

from genart.fps import FPSCounter
from genart.util import parse_size

from .models import BubbleChamber, Particle
from .render import BubbleChamberRenderer
from .simulation import Simulation

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("bubblechamber", help="Bubble chamber simulation")

    parser.add_argument("-s", "--size", default="500x500")
    parser.add_argument("-m", "--magnet", type=float, default=2.0)
    parser.add_argument("-f", "--friction", type=float, default=0.3)

    parser.set_defaults(func=main)


def main(args, config):
    width, height = parse_size(args.size)

    sim = Simulation(
        BubbleChamber(magnetic_field=[0.0, 0.0, args.magnet], friction=args.friction),
        [
            Particle(
                position=[0.0, height / 2.0, 0.0],
                velocity=[350.0, 0.0, 0.0],
                charges=[25, 25, 25],
            )
        ],
    )

    out_file = (
        config["output_dir"] / f"bubblechamber_{dt.datetime.now().isoformat()}.svg"
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
