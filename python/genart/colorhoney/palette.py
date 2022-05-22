from genart.color import Color

RED = Color(1.0, 0.0, 0.0)
GREEN = Color(0.0, 153 / 255, 0.0)
BLUE = Color(0.0, 102 / 255, 1.0)
CYAN = Color(0.0, 204 / 255, 1.0)
MAGENTA = Color(204 / 255, 102 / 255, 1.0)
YELLOW = Color(1.0, 204 / 255, 0.0)

ALPHABET_PATTERN = dict(
    A=(MAGENTA, MAGENTA),
    B=(MAGENTA, CYAN),
    C=(MAGENTA, BLUE),
    D=(MAGENTA, GREEN),
    E=(RED, RED),
    F=(RED, GREEN),
    G=(RED, BLUE),
    H=(RED, CYAN),
    I=(GREEN, GREEN),
    J=(GREEN, BLUE),
    K=(GREEN, CYAN),
    L=(GREEN, MAGENTA),
    M=(GREEN, YELLOW),
    N=(GREEN, RED),
    O=(YELLOW, YELLOW),
    P=(YELLOW, MAGENTA),
    Q=(YELLOW, CYAN),
    R=(YELLOW, BLUE),
    S=(YELLOW, GREEN),
    T=(YELLOW, RED),
    U=(BLUE, BLUE),
    V=(YELLOW, CYAN),
    W=(BLUE, MAGENTA),
    X=(BLUE, YELLOW),
    Y=(CYAN, CYAN),
    Z=(CYAN, BLUE),
)
