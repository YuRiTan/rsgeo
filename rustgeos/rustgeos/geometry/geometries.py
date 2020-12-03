from collections.abc import Sequence
from typing import Tuple, Any

import numpy as np
from nptyping import NDArray

from ..rustgeos import contains, distance  # noqa


class Polygon:
    def __init__(self,
                 exterior: "Sequence[Tuple[float, float]]") -> None:
        self.exterior = exterior
        self.ext_array = self._seq_to_2darray(exterior)

    def __repr__(self):
        return f'Polygon({self.exterior})'

    @staticmethod
    def _seq_to_2darray(seq: "Sequence[Tuple[float, float]]") -> "NDArray[(Any, 2), float, float]":
        array = np.array(seq, dtype=np.float64)
        if not isinstance(array, NDArray[(len(seq), 2), float]):
            msg = 'Something went wrong while converting coordinates to array'
            raise ValueError(msg)

        return array

    def contains(self,
                 x: "NDArray[(Any,), float]",
                 y: "NDArray[(Any,), float]") -> "NDArray[(Any,), bool]":
        return contains(self.ext_array, x, y)

    def distance(self,
                 x: "NDArray[(Any,), float]",
                 y: "NDArray[(Any,), float]") -> "NDArray[(Any,), float]":
        return distance(self.ext_array, x, y)
