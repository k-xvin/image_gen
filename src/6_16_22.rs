use std::{vec, f32::consts::SQRT_2};

use nannou::prelude::*;
use chrono::Utc;

struct Model {
    palette: Vec<Srgba<u8>>,
}

fn main() {
    nannou::app(model)
        .view(view)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
    .size(800,600)
    .key_pressed(key_pressed)
    .build()
    .unwrap();

    // https://www.color-hex.com/color-palette/1014108
    let palette = vec![
        Srgba::new(255,158,47,255),
        Srgba::new(255,183,101,255),
        Srgba::new(255,204,145,255),
        Srgba::new(255,227,195,255),
        Srgba::new(255,249,242,255),
    ];

    Model {
        palette,
    }
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/6_16_22/6_16_22_{}.png", Utc::now().timestamp()));
    }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 2 != 0 {
        return;
    }

    app.main_window().capture_frame(format!("screenshots/6_16_22/6_16_22_{}.png", app.elapsed_frames()/2));
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() % 2 != 0 {
        return;
    }

    let draw = app.draw();

    draw.background().color(model.palette[4]);

    let time = app.time;
    for j in 0..10 {
        for i in 0..30 {
            draw_pillar(&draw, 
                50.0*i as f32 - 400.0 + 15.0*j as f32,
                (2.0 * PI * 0.08 * i as f32 + time).sin() * 100.0 - 15.0*j as f32, 
                30.0,
                model.palette[j%4 + 1]
            )
        }
    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn draw_pillar(draw: &Draw, x: f32, y: f32, width: f32, color: Srgba<u8>){
    let top_rect = Rect::from_corners(vec2(x-width*SQRT_2/2.0, y), vec2(x+width*SQRT_2/2.0, y-600.0));

    let darken = Rgb::from_components((
            (color.red as f32 * 0.8)/ 255.0, 
            (color.green as f32 * 0.8) / 255.0, 
            (color.blue as f32 * 0.8) / 255.0, 
        ));

    draw.rect()
        .xy(top_rect.xy())
        .wh(top_rect.wh())
        .color(darken);

    draw.rect()
        .x_y(x, y)
        .w_h(width, width)
        .z_degrees(45.0)
        .color(Srgba::from_components((color.red, color.green, color.blue, 255)));
}