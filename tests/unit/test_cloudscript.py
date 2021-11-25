from uuid import uuid4

import pytest

from genart.cloudscript import generator, layout
from genart.cloudscript.models import BubbleChamber


def test_generate_superchamber(rng):
    layout = [list("ABC"), list("DEF"), [None] * 3, list("GHI"), list("123")]
    res = generator.make_superchamber(rng, 1000, 1000, layout)

    assert len(res.chambers) == 5

    for row in res.chambers:
        assert len(row) == 3

        for item in row:
            assert isinstance(item, BubbleChamber)


@pytest.mark.parametrize(
    "dims, grid, point, exp_row_col",
    [
        ((100, 100), (1, 1), (50, 50), (0, 0)),
        ((100, 100), (2, 2), (0, 0), (0, 0)),
        ((100, 100), (2, 2), (99, 99), (1, 1)),
    ],
)
def test_superchamber_get_chamber_at(rng, dims, grid, point, exp_row_col):
    layout = [[str(uuid4()) for _ in range(grid[1])] for _ in range(grid[0])]
    superchamber = generator.make_superchamber(rng, dims[0], dims[1], layout)

    res = superchamber.chamber_at(*point)

    assert res is superchamber.chambers[exp_row_col[0]][exp_row_col[1]]


@pytest.mark.parametrize(
    "text, padding, expected_result",
    [
        ("Hello World", 0, [list("Hello") + [None] + list("World")]),
        (
            "Hello World",
            1,
            [
                [None] * (len("Hello World") + 2),
                [None] + list("Hello") + [None] + list("World") + [None],
                [None] * (len("Hello World") + 2),
            ],
        ),
        (
            "Hello World",
            (1, 1, 0, 0),
            [
                [None] * (len("Hello World") + 1),
                list("Hello") + [None] + list("World") + [None],
            ],
        ),
        (
            "Hello\nWorld",
            0,
            [
                list("Hello"),
                list("World"),
            ],
        ),
    ],
)
def test_layout_text(text, padding, expected_result):
    res = layout.layout_text(text, padding)
    assert res == expected_result
