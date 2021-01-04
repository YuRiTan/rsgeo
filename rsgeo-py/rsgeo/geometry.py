from collections.abc import Sequence
from typing import Tuple, Any

import numpy as np
from nptyping import NDArray

from .rsgeo import contains, distance  # noqa


class Polygon:
    def __init__(self, exterior: "Sequence[Tuple[float, float]]") -> None:
        self.exterior = exterior
        self.ext_array = self._seq_to_2darray(exterior)

    def __repr__(self):
        return f"Polygon({self.exterior})"

    @staticmethod
    def _seq_to_2darray(seq: "Sequence[Tuple[float, float]]") -> "np.array":
        array = np.array(seq, dtype=np.float64)
        if not isinstance(array, NDArray[(len(seq), 2), float]):
            msg = "Something went wrong while converting coordinates to array"
            raise ValueError(msg)

        return array

    @staticmethod
    def _to_1d(x):
        if x.ndim == 1:
            return x
        elif x.ndim == 2 and x.shape[1] == 1:
            return x.flatten()
        else:
            raise ValueError("Method algorithm expects 1d Numpy array.")

    def contains(
            self, x: "NDArray[(Any,), float]", y: "NDArray[(Any,), float]"
    ) -> "NDArray[(Any,), bool]":

        return contains(self.ext_array, self._to_1d(x), self._to_1d(y))

    def distance(
            self, x: "NDArray[(Any,), float]", y: "NDArray[(Any,), float]"
    ) -> "NDArray[(Any,), float]":
        return distance(self.ext_array, self._to_1d(x), self._to_1d(y))
