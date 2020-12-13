from typing import Iterable, Iterator, Tuple

from numpy.random import Generator


def jitter_points(
    points: Iterable[Tuple[float, float]], rng: Generator, size: float
) -> Iterator[Tuple[float, float]]:
    for x, y in points:
        yield rng.uniform(x - size, x + size), rng.uniform(y - size, y + size)
