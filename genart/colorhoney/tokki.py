from math import pi

import cairo

from genart import cairoctx
from genart.color import Color

from .base import BaseRenderer
from .palette import ALPHABET_PATTERN


class ColorTokkiRenderer(BaseRenderer):
    def __init__(self, surface: cairo.Surface, scale: int = 10, *args):
        super().__init__(surface, scale, *args)
        self.rect_height = self.scale / 4
        self.blockwidth = self.scale * 2
        self.blockheight = self.scale * 2
        self.block_margin = (self.scale - (2 * self.rect_height + self.margin)) / 2

        # (Translation to baseline of comb, rotation to base)
        self.orientations = [
            ((0, 0), 0),
            ((self.scale, self.scale), -pi / 2),
            ((2 * self.scale, 0), -pi / 2),
            ((self.scale, self.scale), 0),
        ]

    def letter(self, letter: str):
        color_top, color_bot = ALPHABET_PATTERN[letter.upper()]
        self.ctx.move_to(0, 0)
        self.rectangle(color_bot)

        with cairoctx.translation(self.ctx, 0, -(self.rect_height + self.margin)):
            self.ctx.move_to(0, 0)
            self.rectangle(color_top)

    def rectangle(self, color: Color):
        """
        Draws a single rectangle starting at the bottom left.
        """
        with cairoctx.source(self.ctx, color.to_pattern()):
            self.ctx.rel_line_to(0, -self.rect_height)
            self.ctx.rel_line_to(self.scale, 0)
            self.ctx.rel_line_to(0, -self.rect_height)
            self.ctx.rel_line_to(-self.scale, 0)
            self.ctx.fill()
