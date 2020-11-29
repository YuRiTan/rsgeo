import numpy as np
import rustgeos


def test_contains():
    # from rustgeos import Polygon
    coords = np.array([[0., 0.], [1., 1.], [1., 0.], [0., 0.]], dtype=np.float64)
    # p = Polygon(coords)

    xs = np.array([1, 0, 2, .5], dtype=np.float64)
    ys = np.array([1, 0, 2, .25], dtype=np.float65)
    result = rustgeos.contains(coords, xs, ys)

    np.testing.assert_array_equal(result, np.array([False, False, False, True]))
