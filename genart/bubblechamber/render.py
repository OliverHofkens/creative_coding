from collections import defaultdict
from enum import Enum
from math import log
from typing import DefaultDict, Dict

import cairo

from .models import Particle
from .simulation import Simulation


class ColorScheme(Enum):
    BW = "bw"
    COMIC = "comic"


class LineWidth(Enum):
    CONSTANT = "constant"
    MASS = "mass"
    CHARGE = "charge"


class BubbleChamberRenderer:
    def __init__(
        self,
        surface: cairo.Surface,
        color_scheme: ColorScheme = ColorScheme.BW,
        line_width: LineWidth = LineWidth.CONSTANT,
        fps: int = 120,
    ):
        self.ctx: cairo.Context = cairo.Context(surface)
        self.ctx.set_source_rgba(0, 0, 0, 1)
        self.ctx.set_line_join(cairo.LineJoin.ROUND)
        self.ctx.set_line_cap(cairo.LineCap.ROUND)

        self.color_scheme = color_scheme
        self.line_width = line_width

        self.fps = fps
        self._frame_time = 1 / self.fps

        self.last_frame_time = 0.0
        self.trails: DefaultDict = defaultdict(list)
        self.particle_props: Dict[int, Particle] = {}

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
        for id, p in self.trails.items():
            props = self.particle_props[id]

            if self.line_width is LineWidth.MASS:
                lw = log(props.mass) + 0.1
                self.ctx.set_line_width(lw)
            elif self.line_width is LineWidth.CHARGE:
                lw = log(abs(props.total_charge))
                self.ctx.set_line_width(lw)

            self.ctx.move_to(*p[0])
            for i, (control_point, destination) in enumerate(zip(p[1::2], p[2::2])):
                self.ctx.curve_to(*control_point, *control_point, *destination)
                self.ctx.stroke()
                self.ctx.move_to(*destination)

    def trail_particle(self, p: Particle):
        if p.total_charge != 0:
            self.trails[id(p)].append(tuple(p.position))
            self.particle_props[id(p)] = p
