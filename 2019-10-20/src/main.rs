use nannou::prelude::*;
use rand::prelude::*;
use std::vec::Vec;
// use nannou::geom::point::Point2;

const W_WIDTH: u32 = 720;
const GRID_SIZE: usize = 20;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(W_WIDTH, W_WIDTH)
        .view(view)
        .build()
        .unwrap();

    // Because (0, 0) is at the center.
    let origin = -(W_WIDTH as f32) / 2.0;

    let spacing = (W_WIDTH as f32) / (GRID_SIZE as f32);
    let mut grid = [[pt2(0.0, 0.0); GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = origin + i as f32 * spacing;
            let y = origin + j as f32 * spacing;
            grid[i][j] = pt2(x, y);
        }
    }

    Model { _window, grid }
}

struct Model {
    _window: window::Id,
    grid: [[Point2<f32>; GRID_SIZE]; GRID_SIZE],
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(WHITE);

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            draw.ellipse()
                .xy(model.grid[i][j])
                .radius(2.0)
                .color(CRIMSON);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
