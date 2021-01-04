import numpy as np

from rsgeo import contains, distance  # noqa


def test_contains(polygon_coords, xs, ys):
    result = contains(polygon_coords, xs, ys)
    np.testing.assert_array_equal(result, [False, False, False, True])


def test_distance(polygon_coords, xs, ys):
    result = distance(polygon_coords, xs, ys)
    np.testing.assert_array_equal(result, [0, 0, 1.4142135623730951, 0])
