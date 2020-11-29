#![feature(test)]
use crate::rustgeos::core::my_contains;
extern crate test;
use geo::{LineString, Polygon};
use rand;
use rand::Rng;
use test::Bencher;

#[bench]
fn bench_performance_my_contains(b: &mut Bencher) {
    let mut result = Default::default();

    b.iter(|| {
        let polygon = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        let mut rng = rand::thread_rng();
        let xs: Vec<f64> = (0..1000000).map(|_| rng.gen_range(0., 2.)).collect();
        let ys: Vec<f64> = (0..1000000).map(|_| rng.gen_range(0., 2.)).collect();

        result = my_contains(&polygon, &xs, &ys);
    });

    println!("SUM: {}", result.iter().map(|x| *x as u32).sum::<u32>());
}
