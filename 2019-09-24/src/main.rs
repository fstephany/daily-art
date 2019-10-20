use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

////////////////////////////////
fn view(app: &App, _model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(WHITE);

    let radius = 300.0;
    let number_of_lines = 100;

    // Draw 4 lines from (0,0)
    let step = 2.0 * PI / (number_of_lines as f32);

    for x in 0..number_of_lines {
        let pos = step * (x as f32);

        draw.line()
            .start(pt2(0.0, 0.0))
            .stroke_weight(2.0)
            .end(pt2(pos.cos() * radius, pos.sin() * radius))
            .color(PLUM);
    }

    draw.to_frame(app, &frame).unwrap();
}
