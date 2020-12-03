#![deny(rust_2018_idioms)]
use coregeos::{vectorized_contains, vectorized_euclidean_distance};
use geo::{LineString, Polygon};
use itertools::Itertools;
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::prelude::{pymodule, pyfunction, Py, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;

unsafe fn pyarray2_to_tuple_vec(arr: &PyArray2<f64>) -> Vec<(f64, f64)> {
    arr.as_slice()
    .unwrap()
    .iter()
    .copied()
    .tuples::<(_, _)>()
    .collect_vec()
}

#[pyfunction]
unsafe fn contains(
    py: Python<'_>,
    poly_coords: &PyArray2<f64>,
    xs: &PyArray1<f64>,
    ys: &PyArray1<f64>,
) -> PyResult<Py<PyArray1<bool>>> {
    let ext_coords = pyarray2_to_tuple_vec(poly_coords);
    let polygon = Polygon::new(LineString::from(ext_coords), vec![]);
    let xs = xs.as_slice().unwrap();
    let ys = ys.as_slice().unwrap();
    Ok(vectorized_contains(&polygon, xs, ys).into_pyarray(py).to_owned())
}

#[pyfunction]
unsafe fn distance(
    py: Python<'_>,
    poly_coords: &PyArray2<f64>,
    xs: &PyArray1<f64>,
    ys: &PyArray1<f64>,
) -> PyResult<Py<PyArray1<f64>>> {
    let ext_coords = pyarray2_to_tuple_vec(poly_coords);
    let polygon = Polygon::new(LineString::from(ext_coords), vec![]);
    let xs = xs.as_slice().unwrap();
    let ys = ys.as_slice().unwrap();
    Ok(vectorized_euclidean_distance(&polygon, xs, ys).into_pyarray(py).to_owned())
}

#[pymodule]
fn rustgeos(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(contains)).unwrap();
    m.add_wrapped(wrap_pyfunction!(distance)).unwrap();
    Ok(())
}
