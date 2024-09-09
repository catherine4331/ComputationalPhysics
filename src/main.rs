use std::path::Path;

use rand::Rand;

pub mod plot;
pub mod rand;

fn main() {
    let mut ran = Rand::new(10);
    let mut r = 10.0;

    let data: Vec<(f64, f64)> = (0..256)
        .map(|_| {
            let next = ran.gen();

            let out = (r, next);
            r = next;

            out
        })
        .collect();

    plot::scatter(&data, Path::new("./prng.png"))
}
