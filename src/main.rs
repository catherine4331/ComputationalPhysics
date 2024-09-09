use std::path::Path;

use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::{BLUE, WHITE},
};
use rand::Rand;

pub mod rand;

fn main() {
    let mut ran = Rand::new(10);

    let data: Vec<(i32, f64)> = (0..256).map(|i| (i, ran.gen())).collect();

    let root_area = BitMapBackend::new(Path::new("./prng.png"), (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..257, 0.0..1.1)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(data.iter().map(|point| *point), BLUE))
        .unwrap();
}
