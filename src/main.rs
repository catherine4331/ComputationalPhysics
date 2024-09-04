fn main() {
    let mut under = 1.0;
    let mut over = 1.0;
    let mut i = 0;

    loop {
        under = under / 2.0;
        over = over * 2.0;
        i += 1;

        println!("{} {} {}", i, under, over);

        if under == 0.0 || over == std::f64::INFINITY {
            break;
        }
    }
}
