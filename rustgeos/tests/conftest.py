import pytest
import numpy as np

from rustgeos.geometry import Polygon  # noqa


@pytest.fixture(scope='function')
def xs():
    return np.array([1, 0, 2, .5], dtype=np.float64)


@pytest.fixture(scope='function')
def ys():
    return np.array([1, 0, 2, .25], dtype=np.float64)


@pytest.fixture(scope='function')
def polygon_coords():
    return np.array([[0., 0.], [1., 1.], [1., 0.], [0., 0.]], dtype=np.float64)


@pytest.fixture(scope='function')
def polygon():
    return Polygon([(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)])
