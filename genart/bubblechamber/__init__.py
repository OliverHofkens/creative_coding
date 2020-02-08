import logging

from genart.fps import FPSCounter

from .models import BubbleChamber, Particle
from .simulation import Simulation

log = logging.getLogger(__name__)


def register_parser(subparsers):
    parser = subparsers.add_parser("bubblechamber", help="Bubble chamber simulation")
    parser.set_defaults(func=main)


def main(args):
    sim = Simulation(
        BubbleChamber(
            dimensions=[20, 20, 20], magnetic_field=[0.0, 0.0, 2.0], friction=0.3
        ),
        [
            Particle(
                position=[0.0, 0.0, 0.0], velocity=[1.0, 1.0, 0.0], charges=[5, 5, 5]
            )
        ],
    )

    fps = FPSCounter()

    sim.start()
    fps.start()
    while any(p.is_dirty for p in sim.particles):
        sim.step()
        fps.frame_done()
