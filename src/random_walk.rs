#![allow(dead_code)]

use rand::Rng;

pub fn random_walk_2d(n: i64) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut points = vec![];
    let mut x = 0.0;
    let mut y = 0.0;

    for _ in 0..n {
        points.push((x, y));

        x += (rng.gen::<f64>() - 0.5) * 2.0;
        y += (rng.gen::<f64>() - 0.5) * 2.0;
    }

    points
}
