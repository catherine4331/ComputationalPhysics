#![allow(dead_code)]

use std::path::Path;

use plotters::{
    chart::{ChartBuilder, LabelAreaPosition},
    prelude::{BitMapBackend, Cross, IntoDrawingArea},
    style::{BLUE, WHITE},
};

pub fn scatter(data: &[(f64, f64)], path: &Path) {
    let root_area = BitMapBackend::new(path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..1.1, 0.0..1.1)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data.iter().map(|point| Cross::new(*point, 5, &BLUE)))
        .unwrap();
}
