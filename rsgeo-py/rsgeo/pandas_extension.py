import pandas as pd
from rsgeo.geometry import Polygon


@pd.api.extensions.register_dataframe_accessor("rsgeo")
class GeoAccessor:
    def __init__(self, pandas_obj):
        self._obj = pandas_obj
        self._x = "x_pos"
        self._y = "y_pos"

    def _get_xy(self):
        return self._obj[self._x].values, self._obj[self._y].values

    def isin(self, polygon: Polygon):
        return polygon.contains(*self._get_xy())

    def distance(self, polygon: Polygon):
        return polygon.distance(*self._get_xy())
