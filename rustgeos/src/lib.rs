#![deny(rust_2018_idioms)]
use coregeos::my_contains;
use geo::{LineString, Polygon};
use itertools::Itertools;
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::prelude::{pymodule, pyfunction, Py, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;

#[pyfunction]
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
    Ok(my_contains(&polygon, xs, ys).into_pyarray(py).to_owned())
}

#[pymodule]
fn rustgeos(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(contains)).unwrap();
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
