import numpy as np


def test_df_geo_accessor_within(df, polygon):
    result = df.rsgeo.within(polygon)
    expected = [False, False, False, True]
    np.testing.assert_array_equal(result, expected)


def test_df_geo_accessor_distance(df, polygon):
    result = df.rsgeo.distance(polygon)
    expected = [0, 0, 1.4142135623730951, 0]
    np.testing.assert_array_equal(result, expected)
