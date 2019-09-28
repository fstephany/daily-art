use nannou::prelude::*;
use std::vec::Vec;

fn main() {
    nannou::app(model).update(update).run();
}

// Todo:
// - Random length at creation 
// - Expand/Retract mechanism
// - Expand/Retract based on mouse position ?


fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .build()
        .unwrap();

    let max_radius = 300.0;
    let number_of_lines = 100;
    let angle_step = 2.0 * PI / (number_of_lines as f32);

    let mut lines = Vec::with_capacity(number_of_lines);

    for i in 0..number_of_lines {
        lines.push(Line { 
            angle: angle_step * (i as f32),
            length: max_radius,
            direction: Direction::Expanding,
        });
    }

    Model { _window, lines }
}

enum Direction {
    Expanding, 
    Retracting
}

struct Line {
    angle: f32, // in radian
    length: f32,
    direction: Direction, 
}

struct Model {
    _window: window::Id,
    lines: Vec<Line>
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(WHITE);

    for line in &model.lines {
        let endpoint = pt2(
            line.angle.cos() * line.length, 
            line.angle.sin() * line.length
        );

        draw.line()
            .start(pt2(0.0, 0.0))
            .stroke_weight(2.0)
            .end(endpoint)
            .color(PLUM);
    }

    draw.to_frame(app, &frame).unwrap();
}
