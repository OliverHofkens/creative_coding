from typing import List, Tuple

import cairo

from genart import cairoctx


class BaseRenderer:
    def __init__(self, surface: cairo.Surface, scale: int = 10, margin: int = 1):
        self.ctx: cairo.Context = cairo.Context(surface)
        self.scale = scale
        self.margin = margin
        self.blockheight: float = 0
        self.blockwidth: float = 0
        self.orientations: List[Tuple[Tuple[float, float], float]] = []

    def render(self, text: str):
        lines = text.splitlines()
        for i, line in enumerate(lines):
            y_offset = self.blockheight * i
            with cairoctx.translation(
                self.ctx, self.scale * 3, self.scale * 3 + y_offset
            ):
                # self.debug_square()
                self.render_line(line)

    def render_line(self, line: str):
        for i, char in enumerate(line):
            if char == " ":
                continue

            orient_idx = i % 4
            block_offset = i // 4
            orient = self.orientations[orient_idx]

            x_offset = block_offset * self.blockwidth + orient[0][0]
            y_offset = orient[0][1]
            with cairoctx.translation(self.ctx, x_offset, y_offset):
                with cairoctx.rotation(self.ctx, orient[1]):
                    self.letter(char)

    def debug_square(self):
        self.ctx.line_to(0, -self.blockheight)
        self.ctx.line_to(self.blockwidth, -self.blockheight)
        self.ctx.line_to(self.blockwidth, 0)
        self.ctx.line_to(0, 0)
        self.ctx.stroke()

    def letter(self, letter: str):
        raise NotImplementedError()
