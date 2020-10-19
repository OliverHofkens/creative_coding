from collections import defaultdict
from math import pi, sin
from typing import DefaultDict

import cairo

from genart import cairo_util
from genart.color import Color

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

    def add_grid(self, width: float, height: float, rows: int, cols: int):
        rowheight = height // rows
        colwidth = width // cols

        with cairo_util.source(self.ctx, Color(0.5, 0.5, 0.5).to_pattern()):
            for col in range(cols):
                x = col * colwidth
                self.ctx.move_to(x, 0)
                self.ctx.line_to(x, height)
                self.ctx.stroke()

            for row in range(rows):
                y = row * rowheight
                self.ctx.move_to(0, y)
                self.ctx.line_to(width, y)
                self.ctx.stroke()

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
        max_linewidth = 5.0

        for p in self.trails.values():
            trail_len = len(p)

            self.ctx.move_to(*p[0])
            for i, (control_point, destination) in enumerate(zip(p[1::2], p[2::2])):
                # Ease the line width like a half-sine, being thickest in the middle
                progress_pct = i / (trail_len / 2)
                linewidth_pct = sin(progress_pct * pi)
                self.ctx.set_line_width(linewidth_pct * max_linewidth)
                self.ctx.curve_to(*control_point, *control_point, *destination)
                self.ctx.stroke()
                self.ctx.move_to(*destination)

    def trail_particle(self, p: Particle):
        if p.charge != 0.0:
            self.trails[id(p)].append(tuple(p.position[:2]))
