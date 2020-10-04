import pytest

from genart.cloudscript import generator
from genart.cloudscript.models import BubbleChamber


def test_generate_superchamber():
    res = generator.make_superchamber(1000, 1000, 5, 3)

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
def test_superchamber_get_chamber_at(dims, grid, point, exp_row_col):
    superchamber = generator.make_superchamber(dims[0], dims[1], grid[0], grid[1])

    res = superchamber.chamber_at(*point)

    assert res is superchamber.chambers[exp_row_col[0]][exp_row_col[1]]
