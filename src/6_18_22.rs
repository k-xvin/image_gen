mod helpers;
use nannou::{prelude::*, color::Shade, noise::{Perlin, Seedable, NoiseFn}};
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
    helpers::save_at_30fps(app, "6_18_22");
}

// Capture single screenshot
fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/6_18_22_{}.png", Utc::now().timestamp()));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    // Redraw every 60 frames (every second)
    // if app.elapsed_frames() % 60 != 0 {
    //     return;
    // }

    let draw = app.draw();

    draw.background().color(model.palette[4]);

    let mut noise = Perlin::new();
    noise = noise.set_seed(random_f32() as u32);

    let t = (app.elapsed_frames() as f32) * 0.01;

    // draw a grid of cubes
    for i in 0..50 {
        for j in 0..50 {
            let x = i as f32 * 25.0 - 400.0;
            let y = j as f32 * 25.0 - 300.0 + (i%2)as f32*12.5;
            let noisevalue = noise.get([x as f64, y as f64, t as f64]) as f32;
            draw_cube(&draw, x, y, 30.0 * noisevalue + 30.0, model.palette[(i*j)%4]); 
        }
    }

    
    draw.to_frame(app, &frame).unwrap();
}

// Draw an isometric cube centered at x y
fn draw_cube(draw: &Draw, x: f32, y: f32, width: f32, color: Hsla) {
    // height from center to top of cube
    let height = 1.0/f32::sqrt(3.0) * width;
    
    // Bounds of the cube
    let center = pt2(x, y);
    let top = pt2(x, y + height);
    let top_left = pt2(x-(width/2.0), y + height/2.0);
    let top_right = pt2(x+(width/2.0), y + height/2.0);
    let bottom_left = pt2(x-(width/2.0), y - height/2.0);
    let bottom_right = pt2(x+(width/2.0), y - height/2.0);
    let bottom = pt2(x, y - height);

    // top face
    draw.quad()
        .points(center, top_left, top, top_right)
        .color(color);

    // left face
    draw.quad()
        .points(top_left, center, bottom, bottom_left)
        .color(color.darken(0.05));

    // right face
    draw.quad()
        .points(center, top_right, bottom_right, bottom)
        .color(color.darken(0.1));
}