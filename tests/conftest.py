from pathlib import Path

import pytest


@pytest.fixture
def data_dir() -> Path:
    this_file = Path(__file__)
    return this_file.parent / "data"
