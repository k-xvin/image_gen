// a perlin flow field

mod helpers;

use nannou::{prelude::*, noise::{Perlin, Seedable, NoiseFn}, lyon::geom::euclid::approxeq::ApproxEq};
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
    lifetime: f32,
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

    // https://coolors.co/cacaaa-eec584-c8ab83-32cbdf-fc2179
    let palette = vec![
        helpers::hsla_from_hex_rgb(0xcacaaa),
        helpers::hsla_from_hex_rgb(0xeec584),
        helpers::hsla_from_hex_rgb(0xc8ab83),
        helpers::hsla_from_hex_rgb(0x32cbdf),
        helpers::hsla_from_hex_rgb(0xfc2179),
    ];

    let mut points = vec![];
    for _ in 0..5000 {
        // let x = random_range(0.0, app.window_rect().w()) - app.window_rect().w() / 2.0;
        // let y = random_range(0.0, app.window_rect().h()) - app.window_rect().h() / 2.0;
        // random positon along a circle
        let angle = random_f32() * 2.0 * PI;
        let radius = 200.0;
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        points.push(Particle{
            prev_x: x,
            prev_y: y,
            x: x,
            y: y,
            vx: 0.0,
            vy: 0.0,
            radius: 2.0,
            color: palette[random_range(3, 5)],
            lifetime: random_range(0.0, 1.0),
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

    // particle is approximately stuck, so we reset it
    if (p.prev_x * 100.0).round() / 100.0 == (p.x * 100.0).round() / 100.0 && (p.prev_y * 100.0).round() / 100.0 == (p.y * 100.0).round() / 100.0 {
        place_particle_circle(p);
    }

    // put off screen back on screen
    replace_offscreen_particle(p, width, height);
}

// Capture output at 30fps (skip every other frame)
fn update(_app: &App, model: &mut Model, _update: Update) {
    helpers::save_at_30fps(_app, "7_6_22");

    model.t += 0.001;

    // update all the points with the flow vector
    for p in &mut model.points {
        update_point(&model.noise, p, model.t);

        // TODO? if a point is on top of another point, randomize it somewhere else
    }

}

fn place_particle_circle(p: &mut Particle){
    let angle = random_f32() * 2.0 * PI;
    let radius = 200.0;
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    p.x = x;
    p.y = y;
}

fn replace_offscreen_particle(p: &mut Particle, width: f32, height: f32){
    if p.x > width / 2.0 || p.x < -width / 2.0 || p.y > height / 2.0 || p.y < -height / 2.0 {
        place_particle_circle(p);
    }
}



// Capture single screenshot
fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame(format!("screenshots/7_6_22_{}.png", Utc::now().timestamp()));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();

    
    // background can't seem to draw transparent alpha, so just draw a big rectangle
    let bg = rgba(0.0, 0.0, 0.0, 0.05);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(app.window_rect().w(), app.window_rect().h())
        .color(bg);

    // draw all points
    for point in &model.points {
        let mut color = point.color;
        color.alpha = 0.2;
        draw.ellipse()
            .x_y(point.x, point.y)
            .radius(point.radius)
            .color(color);
    }
    

    draw.to_frame(app, &frame).unwrap();
}