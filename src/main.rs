use std::path::Path;

use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::{
        full_palette::{AMBER, DEEPORANGE, LIGHTBLUE, ORANGE, PINK},
        BLACK, BLUE, GREEN, RED, WHITE, YELLOW,
    },
};

pub mod rand;
mod random_walk;

fn main() {
    let colours = [
        BLUE, BLACK, RED, GREEN, LIGHTBLUE, YELLOW, PINK, ORANGE, AMBER, DEEPORANGE,
    ];

    let root_area =
        BitMapBackend::new(Path::new("./output/random_walk.png"), (1080, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-50.0..50.0, -50.0..50.0)
        .unwrap();

    ctx.configure_mesh().disable_mesh().draw().unwrap();

    for colour in colours {
        let data = random_walk::random_walk_2d(1000);

        ctx.draw_series(LineSeries::new(data.iter().map(|point| *point), colour))
            .unwrap();
    }
}
