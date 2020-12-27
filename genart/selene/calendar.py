from math import pi, tau
from typing import Iterator, Sequence

import cairo
from numpy.random import Generator

from genart.cairoctx import rotation, translation
from genart.geom import points_along_arc
from genart.numbering import int_to_roman


def _unicode_range(start_hex: str, end_hex: str) -> Iterator[str]:
    yield from (chr(i) for i in range(int(start_hex, 16), int(end_hex, 16) + 1))


def _calendar_base(
    ctx: cairo.Context,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
    chunks: int,
):
    ctx.arc(pos_x, pos_y, radius_outer, 0, tau)
    ctx.stroke_preserve()

    ctx.arc(pos_x, pos_y, radius_inner, 0, tau)
    ctx.stroke()

    for (start_x, start_y), (end_x, end_y) in zip(
        points_along_arc(pos_x, pos_y, radius_inner, 0, tau, chunks),
        points_along_arc(pos_x, pos_y, radius_outer, 0, tau, chunks),
    ):
        ctx.move_to(start_x, start_y)
        ctx.line_to(end_x, end_y)
        ctx.stroke()


def draw_circular_roman(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    chunks = rng.integers(6, 16)
    _calendar_base(ctx, pos_x, pos_y, radius_outer, radius_inner, chunks)

    ctx.select_font_face("Noto Sans Symbols")
    font_size = 0.8 * (radius_outer - radius_inner)
    ctx.set_font_size(font_size)

    angle_offset = pi / chunks
    for i, (x, y) in enumerate(
        points_along_arc(
            pos_x,
            pos_y,
            (radius_inner + radius_outer) / 2.0,
            angle_offset,
            angle_offset + tau,
            chunks,
        ),
        1,
    ):
        with translation(ctx, x, y), rotation(
            ctx, (i * tau / chunks) + (pi / 2) - angle_offset
        ):
            roman = int_to_roman(i)
            extents = ctx.text_extents(roman)

            ctx.move_to(-1 * extents.width / 2.0, extents.height / 2.0)
            ctx.show_text(roman)
            ctx.new_path()


def _calendar_mapped(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
    mapping: Sequence[str],
    font_family: str = "Noto Sans Symbols",
):
    chunks = rng.integers(6, min(len(mapping), 24))
    _calendar_base(ctx, pos_x, pos_y, radius_outer, radius_inner, chunks)

    ctx.select_font_face(font_family)
    font_size = 0.8 * (radius_outer - radius_inner)
    ctx.set_font_size(font_size)

    symbols = rng.choice(mapping, size=chunks, replace=False)

    angle_offset = pi / chunks
    for i, (x, y) in enumerate(
        points_along_arc(
            pos_x,
            pos_y,
            (radius_inner + radius_outer) / 2.0,
            angle_offset,
            angle_offset + tau,
            chunks,
        ),
        1,
    ):
        with translation(ctx, x, y), rotation(
            ctx, (i * tau / chunks) + (pi / 2) - angle_offset
        ):
            symbol = symbols[i - 1]
            extents = ctx.text_extents(symbol)

            ctx.move_to(-1 * extents.width / 2.0, extents.height / 2.0)
            ctx.show_text(symbol)
            ctx.new_path()


def draw_circular_astrological_planets(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    UNICODES = list(_unicode_range("263F", "2647"))
    _calendar_mapped(ctx, rng, pos_x, pos_y, radius_outer, radius_inner, UNICODES)


def draw_circular_zodiac(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    UNICODES = list(_unicode_range("2648", "2654"))
    _calendar_mapped(ctx, rng, pos_x, pos_y, radius_outer, radius_inner, UNICODES)


def draw_circular_alchemical(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    UNICODES = list(_unicode_range("1F700", "1F773"))
    _calendar_mapped(ctx, rng, pos_x, pos_y, radius_outer, radius_inner, UNICODES)


def draw_circular_hexagrams(
    ctx: cairo.Context,
    rng: Generator,
    pos_x: float,
    pos_y: float,
    radius_outer: float,
    radius_inner: float,
):
    UNICODES = list(_unicode_range("4DC0", "4DFF"))
    _calendar_mapped(
        ctx,
        rng,
        pos_x,
        pos_y,
        radius_outer,
        radius_inner,
        UNICODES,
        "Noto Sans Symbols2",
    )
