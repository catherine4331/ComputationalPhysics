use random_walk::random_walk_2d;

pub mod rand;
mod random_walk;

const N: i32 = 31;

fn main() {
    // let root_area =
    //     BitMapBackend::new(Path::new("./output/random_walk.png"), (1080, 1080)).into_drawing_area();
    // root_area.fill(&WHITE).unwrap();

    // let mut ctx = ChartBuilder::on(&root_area)
    //     .margin(15)
    //     .set_label_area_size(LabelAreaPosition::Left, 40)
    //     .set_label_area_size(LabelAreaPosition::Bottom, 40)
    //     .build_cartesian_2d(-50.0..50.0, -50.0..50.0)
    //     .unwrap();

    // ctx.configure_mesh().disable_mesh().draw().unwrap();

    // ctx.draw_series(LineSeries::new(data.iter().map(|point| *point), BLUE))
    //     .unwrap();

    let mut r_squared_sum = 0.0;
    for _ in 0..N {
        let walk = random_walk_2d(10000);
        r_squared_sum += walk.r_mean_square;
    }

    println!("{}", r_squared_sum / N as f64);
}
