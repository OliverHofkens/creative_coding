from dataclasses import dataclass
from typing import Sequence

import cairo


@dataclass
class Color:
    r: float
    g: float
    b: float
    a: float = 1.0

    def to_pattern(self):
        return cairo.SolidPattern(self.r, self.g, self.b, self.a)


@dataclass
class RadialGradient:
    stops: Sequence[Color]

    def to_pattern(self, x: float, y: float, size: float):
        pat = cairo.RadialGradient(x, y, 1.0, x, y, size)
        for i, stop in enumerate(self.stops):
            pat.add_color_stop_rgb(i, stop.r, stop.g, stop.b)

        return pat
