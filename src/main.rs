fn main() {
    let c = 10E-14;

    let (alpha, beta) = quadratic_roots(1.0, 1.0, c);
    let (alpha1, beta1) = other_quadratic_roots(1.0, 1.0, c);

    println!("{} {}", alpha, beta);
    println!("{} {}", alpha1, beta1);
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let det = (b.powi(2) - 4.0 * a * c).sqrt();

    ((-b + det) / (2.0 * a), (-b - det) / (2.0 * a))
}

fn other_quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let det = (b.powi(2) - 4.0 * a * c).sqrt();

    ((-2.0 * c) / (b + det), (-2.0 * c) / (b - det))
}
