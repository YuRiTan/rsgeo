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

#[cfg(test)]
mod test {
    use super::*;
    use geo::{LineString, Polygon};

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
