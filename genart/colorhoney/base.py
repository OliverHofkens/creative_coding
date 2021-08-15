import cairo


class BaseRenderer:
    def __init__(self, surface: cairo.Surface, scale: int = 10):
        self.ctx: cairo.Context = cairo.Context(surface)
        self.scale = scale

    def render(self, text: str):
        pass
