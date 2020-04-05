from math import pi

import numpy as np
from genart.asparagus.models import Branch


def test_walk_along_branch():
    branch = Branch(np.array([0.0, 0.0]), np.array([10.0, 0.0]))

    for i, res in enumerate(branch.walk_along(1.0)):
        assert np.array_equal(res, np.array([float(i), 0]))


def test_branch_from_branch_simple_right_angles():
    root = Branch(np.array([0.0, 0.0]), np.array([0.0, 10.0]))

    child = root.branch_at(np.array([0.0, 0.0]), 5.0, pi / 2)

    assert np.array_equal(child.start, np.array([0.0, 0.0]))
    assert np.allclose(child.end, np.array([5.0, 0.0]))


def test_branch_from_branch_simple_right_angles():
    root = Branch(np.array([0.0, 0.0]), np.array([0.0, 10.0]))

    # Branch to 'left' with a positive angle
    child = root.branch_at(np.array([0.0, 0.0]), 5.0, pi / 2)

    assert np.array_equal(child.start, np.array([0.0, 0.0]))
    assert np.allclose(child.end, np.array([-5.0, 0.0]))

    # Branch to 'right' with a negative angle
    child = root.branch_at(np.array([0.0, 0.0]), 5.0, -1 * (pi / 2))

    assert np.array_equal(child.start, np.array([0.0, 0.0]))
    assert np.allclose(child.end, np.array([5.0, 0.0]))
