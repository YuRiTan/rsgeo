use geo::algorithm::contains::Contains;
// use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::{Coordinate, CoordinateType};

// pub fn my_distance<T, N>(geometry: &T, xs: &[N], ys: &[N]) -> Vec<bool>
//     where
//         T: EuclideanDistance<Point<Coordinate<N>>>,
//         N: CoordinateType
// {
//     xs.iter()
//         .zip(ys)
//         .map(|(x, y)| geometry.euclidean_distance(&Point::new(*x, *y)))
//         .collect()
//      // let p2: Point<f64> = c.into(); // cast coord into point
// }

pub fn my_contains<T, N>(geometry: &T, xs: &[N], ys: &[N]) -> Vec<bool>
where
    T: Contains<Coordinate<N>>,
    N: CoordinateType,
{
    xs.iter()
        .zip(ys)
        .map(|(x, y)| geometry.contains(&Coordinate { x: *x, y: *y }))
        .collect()
}
