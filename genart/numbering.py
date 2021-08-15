# Arabic, ASCII, Unicode
ROMAN_NUMERALS = [
    (1000, "M", "Ⅿ"),
    (900, "CM", "ⅭⅯ"),
    (500, "D", "Ⅾ"),
    (400, "CD", "ⅭⅮ"),
    (100, "C", "Ⅽ"),
    (90, "XC", "ⅩⅭ"),
    (50, "L", "Ⅼ"),
    (40, "XL", "ⅩⅬ"),
    (10, "X", "Ⅹ"),
    (9, "IX", "Ⅸ"),
    (5, "V", "Ⅴ"),
    (4, "IV", "Ⅳ"),
    (1, "I", "Ⅰ"),
]


def int_to_roman(number: int, unicode: bool = True) -> str:
    # https://stackoverflow.com/questions/28777219/basic-program-to-convert-integer-to-roman-numerals
    result = []
    for (arabic, roman_ascii, roman_unicode) in ROMAN_NUMERALS:
        (factor, number) = divmod(number, arabic)
        result.append((roman_unicode if unicode else roman_ascii) * factor)
        if number == 0:
            break

    return "".join(result)
