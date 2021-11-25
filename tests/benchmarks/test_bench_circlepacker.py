import random

from genart.techniques.circlepacking import pack


def test_bench_pack_circles(benchmark):
    random.seed(0)

    benchmark(pack, width=1000.0, height=1000.0, grow_rate=1.0, max_eyeballs=100)
