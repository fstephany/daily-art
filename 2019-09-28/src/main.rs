use nannou::prelude::*;
use rand;
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
    let mut rng = thread_rng();

    for i in 0..number_of_lines {
        lines.push(Line { 
            angle: angle_step * (i as f32),
            length: rand::rng.gen_rang(0.0, max_radius),
            max_length: max_radius,
            direction: Direction::Expanding,
        });
    }

    Model { _window, lines }
}

enum Direction {
    Expanding, 
    Retracting,
}

struct Line {
    angle: f32, // in radian
    length: f32,
    max_length: f32,
    direction: Direction, 
}

impl Line {
    fn update(&mut self, step: f32) {
        match &self.direction {
            Direction::Expanding => {
                if self.length + step >= self.max_length {
                    self.length = self.max_length;
                    self.direction = Direction::Retracting;
                } else {
                    self.length += step
                }
            },
            Direction::Retracting => {
                if self.length - step <= 0.0 {
                    self.length = 0.0;
                    self.direction = Direction::Expanding;
                } else {
                    self.length -= step
                }
            }
        }
    }
}

struct Model {
    _window: window::Id,
    lines: Vec<Line>
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for line in model.lines.iter_mut() {
        line.update(1.0);
    }
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
