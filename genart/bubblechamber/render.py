from collections import defaultdict
from typing import DefaultDict

import cairo

from .models import Particle
from .simulation import Simulation


class BubbleChamberRenderer:
    def __init__(self, surface: cairo.Surface, fps: int = 120):
        self.ctx: cairo.Context = cairo.Context(surface)
        self.ctx.set_source_rgba(0, 0, 0, 1)
        self.ctx.set_line_join(cairo.LineJoin.ROUND)

        self.fps = fps
        self._frame_time = 1 / self.fps

        self.last_frame_time = 0.0
        self.trails: DefaultDict = defaultdict(list)

    def render(self, sim: Simulation):
        time_passed = sim.time_passed - self.last_frame_time
        if time_passed < self._frame_time:
            return
        self.last_frame_time = sim.time_passed

        for p in sim.particles:
            if p.is_alive:
                self.trail_particle(p)
            elif p.is_dirty:
                p.is_dirty = False
                self.trail_particle(p)

    def finalize(self, sim: Simulation):
        for p in self.trails.values():
            self.ctx.move_to(*p[0])
            for i, (control_point, destination) in enumerate(zip(p[1::2], p[2::2])):
                self.ctx.curve_to(*control_point, *control_point, *destination)
                self.ctx.stroke()
                self.ctx.move_to(*destination)

    def trail_particle(self, p: Particle):
        if p.total_charge != 0:
            self.trails[id(p)].append(tuple(p.position[:2]))
