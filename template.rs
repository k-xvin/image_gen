use nannou::prelude::*;
use chrono::Utc;

struct Model {
}

fn main() {
    nannou::app(model)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
    .size(800,600)
    .key_pressed(key_pressed)
    .build()
    .unwrap();

    Model {}
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/template/template_{}.png", Utc::now().timestamp()));
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {

    // Redraw every 60 frames (every second)
    // if app.elapsed_frames() % 60 != 0 {
    //     return;
    // }

    let draw = app.draw();

    draw.background().color(BLACK);

    let center_rect: Rect<f32> = Rect::from_w_h(400.0, 400.0);
    draw.rect()
        .xy(center_rect.xy())
        .wh(center_rect.wh())
        .color(WHITE); 


    
    draw.to_frame(app, &frame).unwrap();
}