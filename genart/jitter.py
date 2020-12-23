from functools import partial
from itertools import repeat, zip_longest
from typing import Iterable, Iterator, Tuple, Union, cast

from numpy.random import Generator


def jitter_points(
    points: Iterable[Tuple[float, ...]],
    rng: Generator,
    size: Union[float, Iterable[float]],
) -> Iterator[Tuple[float, ...]]:
    if isinstance(size, tuple):
        agg = partial(zip_longest, fillvalue=0.0)
    else:
        size = repeat(cast(float, size))
        agg = zip

    for point in points:
        yield tuple(rng.uniform(p - jit, p + jit) for p, jit in agg(point, size))
