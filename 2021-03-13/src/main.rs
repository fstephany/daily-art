/// Shaking Circle
/// MouseX controls the circle resolution
///
/// inspired by https://www.youtube.com/watch?v=O5wjXoFrau4&t=1s
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};

fn main() {
    nannou::app(model).update(update).run();
}

const RADIUS: f32 = 300.0;
const WIN_SIZE: u32 = 720;

struct Model {}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIN_SIZE, WIN_SIZE)
        .view(view)
        .build()
        .unwrap();

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

////////////////////////////////
fn view(app: &App, _model: &Model, frame: Frame) {
    let mut rng = thread_rng();
    let transparent: Rgba<f32> = rgba(0.0, 0.0, 0.0, 0.0);

    let step = map_range(
        app.mouse.x,
        app.window_rect().left(),
        app.window_rect().right(),
        1,
        359,
    );

    let points: Vec<Vector2> = (0..360)
        .step_by(step)
        .map(|deg| {
            let angle = deg_to_rad(deg as f32);
            let radius = RADIUS + rng.gen_range(-50.0, 50.0);
            Vector2::new(angle.cos() * radius, angle.sin() * radius)
        })
        .collect();

    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(WIN_SIZE as f32, WIN_SIZE as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    draw.polygon()
        .stroke_weight(5.0)
        .stroke_color(ORANGE)
        .color(transparent)
        .points(points);

    draw.to_frame(app, &frame).unwrap();
}
