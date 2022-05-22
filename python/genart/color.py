import colorsys
from dataclasses import dataclass
from typing import Sequence

import cairo


def hex_to_rgb(value):
    value = value.lstrip("#")
    lv = len(value)
    return tuple(int(value[i : i + lv // 3], 16) for i in range(0, lv, lv // 3))


@dataclass
class Color:
    r: float
    g: float
    b: float
    a: float = 1.0

    @classmethod
    def from_hex(cls, hex_color: str) -> "Color":
        return cls(*hex_to_rgb(hex_color))

    @classmethod
    def from_hsv(cls, hue: float, sat: float, val: float) -> "Color":
        return cls(*colorsys.hsv_to_rgb(hue, sat, val))

    def to_pattern(self):
        return cairo.SolidPattern(self.r, self.g, self.b, self.a)


@dataclass
class LinearGradient:
    stops: Sequence[Color]

    def to_pattern(self, x1: float, y1: float, x2: float, y2: float):
        pat = cairo.LinearGradient(x1, y1, x2, y2)
        for i, stop in enumerate(self.stops):
            pat.add_color_stop_rgba(i, stop.r, stop.g, stop.b, stop.a)

        return pat


@dataclass
class RadialGradient:
    stops: Sequence[Color]

    def to_pattern(self, x: float, y: float, size: float, start_radius: float = 1.0):
        pat = cairo.RadialGradient(x, y, start_radius, x, y, size)
        for i, stop in enumerate(self.stops):
            pat.add_color_stop_rgba(i, stop.r, stop.g, stop.b, stop.a)

        return pat
