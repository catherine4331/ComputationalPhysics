#![allow(dead_code)]

use rand::Rng;

pub fn random_walk_2d(n: i64) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut points = vec![];
    let mut x = 0.0;
    let mut y = 0.0;

    for _ in 0..n {
        points.push((x, y));

        let dx = (rng.gen::<f64>() - 0.5) * 2.0;
        let dy = (rng.gen::<f64>() - 0.5) * 2.0;
        let l = (dx.powi(2) + dy.powi(2)).sqrt();

        x += dx * (1.0 / l);
        y += dy * (1.0 / l);
    }

    points
}
