// a perlin flow field

mod helpers;

use std::ops::Add;

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
    ax: f32,
    ay: f32,
    radius: f32,
    color: Hsla,
}

struct Model {
    palette: Vec<Hsla>,
    points: Vec<Particle>,
    noise: Perlin,
    t: f32,
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
            ax: 0.0,
            ay: 0.0,
            radius: 1.0,
            color: palette[0],
        });
    }

    let noise = Perlin::new().set_seed(random_range(0, 100));
    let t = 0.0;

    Model {
        palette,
        points,
        noise,
        t
    }
}

fn get_flow_vector(noise: &Perlin, x: f32, y: f32, t: f32) -> Vec2{
    // map normal x y to a smaller section of the perlin field
    let scale_factor: f64 = 1.0 / 500.0;
    let noise_val = noise.get([x as f64 * scale_factor, y as f64 * scale_factor, t as f64]) as f32 * 100.0;
    
    // value between -1 and 1, need to scale to an angle
    let angle_radians = noise_val * 2.0*PI;

    // convert to a vector
    // sin(a) = o/h -> o = h*sin(a)
    // cos(a) = a/h -> a = h*cos(a)
    let magnitude = 1.0;
    Vec2::new(magnitude*(angle_radians.cos()), magnitude*(angle_radians.sin()))
}

// apply flow vector to the particle
// save the particle's previous position
fn update_point(noise: &Perlin, mut p: &mut Particle, t: f32){
    let flow_vector = get_flow_vector(noise, p.x, p.y, t);

    p.prev_x = p.x;
    p.prev_y = p.y;

    let width = 800.0;
    let height = 600.0;

    p.vx = flow_vector.x;
    p.vy = flow_vector.y;

    p.x += p.vx;
    p.y += p.vy;

    // if particle slows down too much plop it somewhere else
    // println!("{:?}", p.vx);
    // println!("{:?}", p.vy);
    // println!("prev to new distance: {:?}", vec2(p.prev_x, p.prev_y).distance(vec2(p.x, p.y)));

    // if vec2(p.prev_x, p.prev_y).distance(vec2(p.x, p.y)) <= 1.1 {
    //     println!("plopping");
    //     p.x = random_range(-width / 2.0, width / 2.0);
    //     p.y = random_range(-height / 2.0, height / 2.0);
    // }

    // put off screen back on screen
    if p.x > width / 2.0 || p.x <= -width / 2.0{
        p.x = random_range(-width / 2.0, width / 2.0);
    }
    if p.y > height / 2.0 || p.y <= -height / 2.0{
        p.y = random_range(-height / 2.0, height / 2.0);
    }
}

// Capture output at 30fps (skip every other frame)
fn update(_app: &App, model: &mut Model, _update: Update) {
    // helpers::save_at_30fps(_app, "7_5_22");

    model.t += 0.001;

    // update all the points with the flow vector
    for p in &mut model.points {
        update_point(&model.noise, p, model.t);
    }

}

// Capture single screenshot
fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/7_5_22_{}.png", Utc::now().timestamp()));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    // Redraw every 60 frames (every second)
    // if app.elapsed_frames() % 60 != 0 {
    //     return;
    // }

    let draw = app.draw();

    let bg = rgba(0.0, 0.0, 0.0, 0.05);

    // background can't seem to draw transparent alpha, so just draw a big rectangle
    // draw.background().color(bg);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(app.window_rect().w(), app.window_rect().h())
        .color(bg);

    // draw all points
    for point in &model.points {
        let mut color = point.color;
        color.alpha = 0.4;
        draw.ellipse()
            .x_y(point.x, point.y)
            .radius(point.radius)
            .color(color);
        // draw.line()
        //     .start(vec2(point.prev_x, point.prev_y))
        //     .end(vec2(point.x, point.y))
        //     .start_cap_round()
        //     .end_cap_round()
        //     .color(color);
    }
    

    draw.to_frame(app, &frame).unwrap();
}