from collections import defaultdict
from enum import Enum
from math import log, pi
from typing import DefaultDict, Dict

import cairo
from numpy.random import Generator

from genart import cairoctx
from genart.color import Color, RadialGradient

from .models import Particle
from .simulation import Simulation


class ColorScheme(Enum):
    BW = "bw"
    COMIC = "comic"

    def gen_color(self, rng: Generator, particle: Particle):
        if self is ColorScheme.COMIC:
            hue = rng.random()
            sat = rng.uniform(0.5, 1.0)
            brightness = rng.uniform(0.5, 1.0)
            return Color.from_hsv(hue, sat, brightness)


class LineWidth(Enum):
    CONSTANT = "constant"
    MASS = "mass"
    CHARGE = "charge"


class BubbleChamberRenderer:
    def __init__(
        self,
        surface: cairo.Surface,
        rng: Generator,
        width: float,
        height: float,
        color_scheme: ColorScheme = ColorScheme.BW,
        line_width: LineWidth = LineWidth.CONSTANT,
        fps: int = 120,
    ):
        self.ctx: cairo.Context = cairo.Context(surface)
        self.ctx.set_source_rgba(0, 0, 0, 1)
        self.ctx.set_line_join(cairo.LineJoin.ROUND)
        self.ctx.set_line_cap(cairo.LineCap.ROUND)

        self.rng = rng
        self.width = width
        self.height = height

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

            if self.color_scheme is ColorScheme.BW:
                color = Color(0.0, 0.0, 0.0)
            else:
                color = self.color_scheme.gen_color(self.rng, props)

            self.ctx.move_to(*p[0])
            for control_point, destination in zip(p[1::2], p[2::2]):
                self.ctx.curve_to(*control_point, *control_point, *destination)

            with cairoctx.source(self.ctx, color.to_pattern()):
                self.ctx.stroke()

        if self.color_scheme is ColorScheme.COMIC:
            # Overlay a lightening radial gradient to create a "bang".
            grad = RadialGradient(
                (Color(1.0, 1.0, 1.0, 0.9), Color(1.0, 1.0, 1.0, 0.0))
            )
            mid_x = self.width / 2.0
            mid_y = self.height / 2.0
            radius = min(self.width, self.height) / 4.0
            with cairoctx.operator(self.ctx, cairo.Operator.ATOP), cairoctx.source(
                self.ctx, grad.to_pattern(mid_x, mid_y, radius)
            ):
                self.ctx.arc(mid_x, mid_y, radius, 0.0, pi * 2)
                self.ctx.fill()

    def trail_particle(self, p: Particle):
        if p.total_charge != 0:
            self.trails[id(p)].append(tuple(p.position))
            self.particle_props[id(p)] = p
