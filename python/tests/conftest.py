from pathlib import Path

import numpy as np
import pytest


@pytest.fixture
def data_dir() -> Path:
    this_file = Path(__file__)
    return this_file.parent / "data"


@pytest.fixture()
def rng():
    return np.random.default_rng()
