// a perlin flow field, but the particle calcs aren't quite done

mod helpers;

use nannou::{prelude::*, noise::{Perlin, Seedable, NoiseFn}};
use chrono::Utc;

fn main() {
    nannou::app(model)
        .view(view)
        .update(update)
        .run();
}

struct Particle {
    prev_x: f32,
    prev_y: f32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    color: Hsla,
}

struct Model {
    palette: Vec<Hsla>,
    points: Vec<Particle>,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    app.new_window()
    .size(800,600)
    .key_pressed(key_pressed)
    .build()
    .unwrap();

    // white
    let palette = vec![
        helpers::hsla_from_hex_rgb(0xffffff),
    ];

    let mut points = vec![];
    for _ in 0..10000 {
        let x = random_range(0.0, app.window_rect().w()) - app.window_rect().w() / 2.0;
        let y = random_range(0.0, app.window_rect().h()) - app.window_rect().h() / 2.0;
        points.push(Particle{
            prev_x: x,
            prev_y: y,
            x: x,
            y: y,
            vx: 0.0,
            vy: 0.0,
            radius: 1.0,
            color: palette[0],
        });
    }

    let noise = Perlin::new().set_seed(random_range(0, 100));

    Model {
        palette,
        points,
        noise
    }
}

fn get_flow_vector(noise: &Perlin, x: f32, y: f32) -> Vec2{
    // map normal x y to a smaller section of the perlin field
    let zoom_factor: f64 = 200.0;
    let noise_val = noise.get([x as f64 / zoom_factor, y as f64 / zoom_factor, 0.0]) as f32 * 100.0;
    
    // value between -1 and 1, need to scale to an angle
    let angle_radians = noise_val * 2.0*PI;

    // convert to a vector
    // sin(a) = o/h -> o = h*sin(a)
    // cos(a) = a/h -> a = h*cos(a)
    Vec2::new(1.0*angle_radians.cos(), 1.0*angle_radians.sin())
}

// apply flow vector to the particle
// save the particle's previous position
fn update_point(noise: &Perlin, mut p: &mut Particle){
    let flow_vector = get_flow_vector(noise, p.x, p.y);
    p.prev_x = p.x;
    p.prev_y = p.y;
    p.x += flow_vector.x;
    p.y += flow_vector.y;
}

// Capture output at 30fps (skip every other frame)
fn update(_app: &App, model: &mut Model, _update: Update) {
    helpers::save_at_30fps(_app, "7_4_22");

    // update all the points with the flow vector
    for p in &mut model.points {
        update_point(&model.noise, p);
    }

}

// Capture single screenshot
fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/7_4_22_{}.png", Utc::now().timestamp()));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    // Redraw every 60 frames (every second)
    // if app.elapsed_frames() % 60 != 0 {
    //     return;
    // }

    let draw = app.draw();

    // draw.background().color(BLACK);

    // draw all points
    for point in &model.points {
        let mut color = point.color;
        color.alpha = 0.4;
        draw.ellipse()
            .x_y(point.x, point.y)
            .radius(point.radius)
            .color(color);
    }
    

    draw.to_frame(app, &frame).unwrap();
}