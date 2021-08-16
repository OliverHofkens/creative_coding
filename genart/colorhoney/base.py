import cairo


class BaseRenderer:
    def __init__(self, surface: cairo.Surface, scale: int = 10, margin: int = 1):
        self.ctx: cairo.Context = cairo.Context(surface)
        self.scale = scale
        self.margin = margin

    def render(self, text: str):
        pass
