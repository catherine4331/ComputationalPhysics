use std::f64::consts::PI;

fn main() {
    let r = 1.0;

    println!(
        "r, C, A = {}, {}, {}",
        r,
        circle_circumference(r),
        circle_area(r)
    )
}

fn circle_circumference(r: f64) -> f64 {
    2.0 * PI * r
}

fn circle_area(r: f64) -> f64 {
    PI * r.powi(2)
}
