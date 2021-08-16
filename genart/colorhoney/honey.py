from math import pi, radians, sin, sqrt

import cairo

from genart import cairoctx
from genart.color import Color

from .base import BaseRenderer
from .palette import ALPHABET_PATTERN


class ColorHoneyRenderer(BaseRenderer):
    def __init__(self, surface: cairo.Surface, scale: int = 10, *args):
        super().__init__(surface, scale, *args)
        self.diag_offset = sin(radians(60)) * scale
        self.diag_margin = sqrt(self.margin / 2)
        self.blockwidth = self.diag_offset * 2 + self.margin * 2

        # (Translation to baseline of comb, rotation to base)
        self.orientations = [
            ((0, 0), radians(30)),
            ((self.margin / 2, self.diag_margin), radians(90)),
            ((2 * self.diag_offset + self.margin, 0), radians(-30)),
            (
                (
                    self.diag_offset + self.margin + self.diag_margin,
                    (-1.5 * self.scale) - self.diag_margin,
                ),
                radians(90),
            ),
        ]

    def render(self, text: str):
        with cairoctx.translation(self.ctx, self.scale * 3, self.scale * 3):
            for i, char in enumerate(text):
                orient_idx = i % 4
                block_offset = i // 4
                orient = self.orientations[orient_idx]

                x_offset = block_offset * self.blockwidth + orient[0][0]
                y_offset = orient[0][1]
                with cairoctx.translation(self.ctx, x_offset, y_offset):
                    with cairoctx.rotation(self.ctx, orient[1]):
                        self.letter(char)

    def letter(self, letter: str):
        color_top, color_bot = ALPHABET_PATTERN[letter.upper()]
        self.ctx.move_to(0, 0)
        self.triangle(color_bot)

        # Move up 2x diag_offset and flip the canvas upside-down
        with cairoctx.translation(self.ctx, 0, -2 * self.diag_offset):
            self.ctx.move_to(0, 0)
            with cairoctx.rotation(self.ctx, pi):
                self.triangle(color_top)

    def triangle(self, color: Color):
        """
        Draws a single triangle with the point down.
        """
        with cairoctx.source(self.ctx, color.to_pattern()):
            self.ctx.rel_line_to(-self.scale / 2, -self.diag_offset)
            self.ctx.rel_line_to(self.scale, 0)
            self.ctx.rel_line_to(-self.scale / 2, self.diag_offset)
            self.ctx.fill()
