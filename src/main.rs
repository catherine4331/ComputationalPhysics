fn main() {
    let xs = [
        0.0, 0.05, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 2.0,
    ];
    let sines = xs.map(|x| (x, sine_series(x, 10E-12), x.sin()));

    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
        "x", "imax", "sum", "sum - sin(x)", "sum / sin(x)"
    );
    for (x, series, exact) in sines {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3:.5} | {4:.5}",
            x,
            series.1,
            series.1,
            series.0 - exact,
            series.0 / exact
        )
    }
}

fn sine_series(x: f64, eps: f64) -> (f64, i32) {
    let mut term = x;
    let mut sum = x;
    let mut n = 2;

    while (term / sum).abs() > eps {
        term = -term * x * x / (2.0 * n as f64 + 1.0) * (2.0 * n as f64 - 2.0);
        sum = term + sum;
        n += 1;
    }

    (sum, n)
}
