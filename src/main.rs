use std::path::Path;

use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::{BLUE, WHITE},
};

pub mod rand;
mod random_walk;

fn main() {
    let data = random_walk::random_walk_2d(1000);

    let root_area =
        BitMapBackend::new(Path::new("./output/random_walk.png"), (1080, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-40.0..40.0, -40.0..40.0)
        .unwrap();

    ctx.configure_mesh().disable_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(data.iter().map(|point| *point), BLUE))
        .unwrap();
}
