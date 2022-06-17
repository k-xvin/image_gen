use nannou::prelude::*;
use chrono::Utc;

struct Model {
    palette: Vec<Srgba<u8>>,
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

    // https://www.color-hex.com/color-palette/27739
    let palette = vec![
        Srgba::new(154,172,184,255),
        Srgba::new(221,226,227,255),
        Srgba::new(179,124,87,255),
        Srgba::new(60,69,92,255),
        Srgba::new(96,65,43,255),
    ];

    Model {
        palette,
    }
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