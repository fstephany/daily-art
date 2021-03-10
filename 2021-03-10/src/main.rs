/// Running dot around a circle.
// inspired by https://www.youtube.com/watch?v=O5wjXoFrau4&t=1s
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

static RADIUS: f32 = 300.0;

struct Model {
    cur_angle: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(720, 720).view(view).build().unwrap();

    Model { cur_angle: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.cur_angle = model.cur_angle + 0.01;
}

////////////////////////////////
fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(RADIUS)
        .stroke_weight(1.0)
        .stroke_color(ORANGE)
        .color(BLACK);

    let cur_pos = Vector2::new(
        model.cur_angle.cos() * RADIUS,
        model.cur_angle.sin() * RADIUS,
    );

    draw.ellipse().xy(cur_pos).radius(10.0).color(YELLOW);

    draw.to_frame(app, &frame).unwrap();
}
