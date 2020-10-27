from itertools import zip_longest
from typing import List, Optional, Sequence, Union


def layout_text(
    text: str, padding: Union[int, Sequence[int]]
) -> Sequence[Sequence[Optional[str]]]:
    if isinstance(padding, int):
        padding_top = padding_right = padding_bot = padding_left = padding
    else:
        padding_top, padding_right, padding_bot, padding_left = padding

    lines = text.splitlines()
    max_line_length = max(len(line) for line in lines)
    fullwidth = max_line_length + padding_left + padding_right

    results: List[List[Optional[str]]] = []

    if padding_top:
        results.extend([[None] * fullwidth for _ in range(padding_top)])

    for i, line in enumerate(lines, padding_top):
        results.append([None] * padding_left if padding_left else [])

        for j, symbol in zip_longest(range(max_line_length), line):
            if symbol == " ":
                symbol = None
            results[i].append(symbol)

        if padding_right:
            results[i].extend([None] * padding_right)

    if padding_bot:
        results.extend([[None] * fullwidth for _ in range(padding_bot)])

    return results
