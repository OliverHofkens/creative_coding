from contextlib import contextmanager

import cairo


@contextmanager
def source(ctx: cairo.Context, source: cairo.Pattern):
    src = ctx.get_source()
    ctx.set_source(source)

    try:
        yield
    finally:
        ctx.set_source(src)


@contextmanager
def translation(ctx: cairo.Context, x: float, y: float):
    ctx.translate(x, y)

    try:
        yield
    finally:
        ctx.translate(-x, -y)


@contextmanager
def rotation(ctx: cairo.Context, radians: float):
    ctx.rotate(radians)

    try:
        yield
    finally:
        ctx.rotate(-radians)


@contextmanager
def operator(ctx: cairo.Context, op: cairo.Operator):
    prev_op = ctx.get_operator()
    ctx.set_operator(op)

    try:
        yield
    finally:
        ctx.set_operator(prev_op)
