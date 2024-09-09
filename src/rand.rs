#![allow(dead_code)]

pub struct Rand {
    r: i64,

    a: i64,
    c: i64,
    m: i64,
}

impl Rand {
    pub fn new(seed: i64) -> Self {
        Self {
            r: seed,
            a: 4,
            c: 1,
            m: 9,
        }
    }

    // Generate a number in the range [0, 1)
    pub fn gen(&mut self) -> f64 {
        self.r = (self.a * self.r + self.c) % self.m;

        self.r as f64 / self.m as f64
    }
}
