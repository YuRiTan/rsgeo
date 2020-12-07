use geo::algorithm::contains::Contains;
use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::{Coordinate, CoordinateType, Point};

pub fn vectorized_euclidean_distance<T, N>(geometry: &T, xs: &[N], ys: &[N]) -> Vec<N>
where
    T: EuclideanDistance<N, Point<N>>,
    N: CoordinateType,
{
    xs.iter()
        .zip(ys)
        .map(|(x, y)| geometry.euclidean_distance(&Point::new(*x, *y)))
        .collect()
}

pub fn vectorized_contains<T, N>(geometry: &T, xs: &[N], ys: &[N]) -> Vec<bool>
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
    fn test_vectorized_contains() {
        let polygon = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        let xs = vec![1.0, 0.0, 2.0, 0.5];
        let ys = vec![1.0, 0.0, 2.0, 0.25];
        let result = vectorized_contains(&polygon, &xs, &ys);
        assert_eq!(result, vec!(false, false, false, true))
    }

    #[test]
    fn test_vectorized_euclidean_distance() {
        let polygon = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        let xs = vec![1.0, 0.0, 2.0, 0.5];
        let ys = vec![1.0, 0.0, 2.0, 0.25];
        let result = vectorized_euclidean_distance(&polygon, &xs, &ys);
        assert_eq!(result, vec![0.0, 0.0, 1.4142135623730951, 0.0])
    }
}
