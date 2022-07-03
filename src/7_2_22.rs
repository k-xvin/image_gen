mod helpers;

use helpers::hsla_from_rgb_u8;
use nannou::{prelude::*, noise::{Perlin, Seedable, NoiseFn}, color::Shade};
use chrono::Utc;

fn main() {
    nannou::app(model)
        .view(view)
        .update(update)
        .run();
}

struct Model {
    noise: Perlin,
}

fn model(app: &App) -> Model {
    app.new_window()
    .size(800,600)
    .key_pressed(key_pressed)
    .build()
    .unwrap();
        
    let noise = Perlin::new().set_seed(random_range(0, 100));

    Model {
        noise,
    }
}

// Capture output at 30fps (skip every other frame)
fn update(_app: &App, _model: &mut Model, _update: Update) {
    helpers::save_at_30fps(_app, "7_2_22_jpg");
}

// Capture single screenshot
fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/7_2_22_{}.png", Utc::now().timestamp()));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    // Redraw every 60 frames (every second)
    // if app.elapsed_frames() % 60 != 0 {
    //     return;
    // }

    let draw = app.draw();

    draw.background().color(BLACK);

    // Draw a grid of points
    // Each point has a noise value between 0 and 100
    // The point is colored based on the noise value
    let zoom_factor: f64 = 250.0;
    let scale_factor = 5;
    let t = (app.elapsed_frames() as f32) * 0.005;
    for i in 0..(800/scale_factor)+1 {
        for j in 0..(600/scale_factor)+1 {
            let x = i as f32 * scale_factor as f32 - 400.0;
            let y = j as f32 * scale_factor as f32 - 300.0;

            // needs to be a super zoomed in part of the perlin map

            let noise_val = model.noise.get([x as f64 /zoom_factor, y as f64 / zoom_factor, t as f64]) as f32 * 100.0;

            let color = get_color(noise_val);
            draw.rect()
                .x_y(x, y)
                .w_h(scale_factor as f32, scale_factor as f32)
                .color(color);

            // helpers::shapes::draw_iso_cube(&draw, x, y, scale_factor as f32, color);

        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn get_color(noise_val: f32) -> Hsla {
    let noise_val = noise_val as i32;
    match noise_val{
        // the math is a happy accident
        0..=20 => hsla_from_rgb_u8(TAN).darken((noise_val%20) as f32 / 100.0),
        21..=100 => hsla_from_rgb_u8(GREEN).lighten((noise_val%20) as f32 / 100.0),
        // 81..=100 => hsla_from_rgb_u8(GREY).darken((noise_val%20) as f32 / 100.0),
        _ => hsla_from_rgb_u8(BLUE).darken((noise_val%20) as f32 / 100.0),
    }
}