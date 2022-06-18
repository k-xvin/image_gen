mod helpers;

use nannou::prelude::*;
use chrono::Utc;

fn main() {
    nannou::app(model)
        .view(view)
        .update(update)
        .run();
}

struct Model {
    palette: Vec<Hsla>,
}

fn model(app: &App) -> Model {
    app.new_window()
    .size(800,600)
    .key_pressed(key_pressed)
    .build()
    .unwrap();

    // https://coolors.co/cc444b-da5552-df7373-e39695-e4b1ab
    let palette = vec![
        helpers::hsla_from_hex_rgb(0xcc444b),
        helpers::hsla_from_hex_rgb(0xda5552),
        helpers::hsla_from_hex_rgb(0xdf7373),
        helpers::hsla_from_hex_rgb(0xe39695),
        helpers::hsla_from_hex_rgb(0xe4b1ab),
    ];

    Model {
        palette,
    }
}

// Capture output at 30fps (skip every other frame)
fn update(app: &App, _model: &mut Model, _update: Update) {
    // helpers::save_at_30fps(app, "template");
}

// Capture single screenshot
fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/template_{}.png", Utc::now().timestamp()));
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