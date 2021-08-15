from math import pi, radians, sin

import cairo

from genart import cairoctx
from genart.color import Color

from .base import BaseRenderer
from .palette import ALPHABET_PATTERN


class ColorHoneyRenderer(BaseRenderer):
    def __init__(self, surface: cairo.Surface, scale: int = 10):
        super().__init__(surface, scale)
        self.diag_offset = sin(radians(60)) * scale

    def render(self, text: str):
        for i, char in enumerate(text, 1):
            with cairoctx.translation(self.ctx, i * self.scale, 2 * self.scale):
                self.letter(char.upper())

    def letter(self, letter: str):
        color_top, color_bot = ALPHABET_PATTERN[letter]
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
