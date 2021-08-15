from typing import Tuple


def parse_size(size: str) -> Tuple[int, int]:
    """
    Parses a size string such as '1000' or '1920x1080'
    into a (width, height) pair
    """
    try:
        if "x" in size:
            parts = size.split("x", maxsplit=1)
            return int(parts[0]), int(parts[1])
        else:
            width = int(size)
            return width, width
    except ValueError:
        raise ValueError("Invalid size string")
