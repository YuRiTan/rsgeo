#![deny(rust_2018_idioms)]

pub mod core;

use crate::core::my_contains;
use geo::{LineString, Polygon};
use itertools::Itertools;
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};

#[pymodule]
fn rustgeos(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "contains")]
    unsafe fn contains(
        py: Python<'_>,
        poly_coords: &PyArray2<f64>,
        xs: &PyArray1<f64>,
        ys: &PyArray1<f64>,
    ) -> PyResult<Py<PyArray1<bool>>> {
        let ext_coords = poly_coords
            .as_slice()
            .unwrap()
            .iter()
            .copied()
            .tuples::<(_, _)>()
            .collect_vec();
        let polygon = Polygon::new(LineString::from(ext_coords), vec![]);
        let xs = xs.as_slice().unwrap();
        let ys = ys.as_slice().unwrap();
        println!("JOE");
        Ok(my_contains(&polygon, xs, ys).into_pyarray(py).to_owned())
    }
    Ok(())
}

// #[pyclass]
// #[repr(transparent)]
// pub struct PyPolygon {
//     pub polygon: Polygon<f64>,
// }
//
// impl PyPolygon {
//     fn new(polygon: Polygon<f64>) -> Self {
//         PyPolygon { polygon }
//     }
// }
//
// impl From<Polygon<f64>> for PyPolygon {
//     fn from(polygon: Polygon<f64>) -> Self {
//         PyPolygon { polygon }
//     }
// }
//
// #[pymethods]
// impl PyPolygon {
//     #[new]
//     unsafe fn __init__(coords: &PyArray2<f64>) -> PyResult<Self> {
//         let ext_coords = coords
//             .as_slice()
//             .unwrap()
//             .iter()
//             .copied()
//             .tuples::<(_, _)>()
//             .collect_vec();
//         let polygon = Polygon::new(LineString::from(ext_coords), vec![]);
//         Ok(PyPolygon::new(polygon))
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::my_contains;
    use geo::Polygon;

    #[test]
    fn test_my_contains() {
        let polygon = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        let xs = vec![1.0, 0.0, 2.0, 0.5];
        let ys = vec![1.0, 0.0, 2.0, 0.25];
        let result = my_contains(&polygon, &xs, &ys);
        assert_eq!(result, vec!(false, false, false, true))
    }
}
