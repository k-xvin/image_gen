use nannou::{prelude::*, color::{IntoColor, Shade, Hue}};

struct Model {
    palette: Vec<Srgba<u8>>,
    background: wgpu::Texture,
}

fn main() {
    nannou::app(model)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
    .size(1920,1080)
    .fullscreen()
    .key_pressed(key_pressed)
    .build()
    .unwrap();

    // https://www.color-hex.com/color-palette/27739
    let palette = vec![
        Srgba::new(221,226,227,255),
        Srgba::new(154,172,184,255),
        Srgba::new(179,124,87,255),
        Srgba::new(60,69,92,255),
        Srgba::new(96,65,43,255),
    ];

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("backgrounds").join("darkest_hour.jpg");
    let background = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        palette,
        background
    }
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame("screenshots/square3.png");
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() % 60 != 0 {
        return;
    }

    let draw = app.draw();

    draw.background().color(model.palette[0]);
    draw.texture(&model.background);
    
    let center_rect: Rect<f32> = Rect::from_w_h(1920.0, 1080.0);
    // draw.rect()
    //     .xy(center_rect.xy())
    //     .wh(center_rect.wh())
    //     .color(model.palette[3]); 


    split_random(&draw, center_rect, &rand_color, &model.palette);

    draw.to_frame(app, &frame).unwrap();

    // app.main_window().capture_frame(format!("screenshots/square2/{}.png", app.elapsed_frames()));
}

// recursively split until rects are too small
fn split_random(draw: &Draw, rect: Rect<f32>, color_fn: &dyn Fn(&Vec<Srgba<u8>>) -> Srgba<u8>, palette: &Vec<Srgba<u8>>) {
    if rect.w() < 50.0 || rect.h() < 50.0 {
        return;
    }

    if random_f32() < 0.5 {
        let rects = split_rect_horizontal(draw, rect, color_fn(palette), color_fn(palette));
        split_random(draw, rects.0, color_fn, palette);
        split_random(draw, rects.1, color_fn, palette);
    }
    else {
        let rects = split_rect_vertical(draw, rect, color_fn(palette), color_fn(palette));
        split_random(draw, rects.0, color_fn, palette);
        split_random(draw, rects.1, color_fn, palette);
    } 
}

fn rand_color(palette: &Vec<Srgba<u8>>) -> Srgba<u8> {
    palette[random_range(0, palette.len())]
}

fn split_rect_vertical(draw: &Draw, rect: Rect<f32>, left_color: Srgba<u8>, right_color: Srgba<u8>) -> (Rect<f32>, Rect<f32>) {
    let margin = 10.0;
    let split_height = rect.h() - 2.0*margin;
    let split_width = rect.w()/2.0 - 1.5*margin;
    let shift_amt = split_width/2.0 + margin/2.0;

    let mut left_rect = Rect::from_x_y_w_h(rect.x(), rect.y(), split_width, split_height);
    left_rect = left_rect.shift_x(-shift_amt);
    // draw.rect()
    //     .xy(left_rect.xy())
    //     .wh(left_rect.wh())
    //     .color(left_color);
    draw_rectangular_prism(draw, left_rect, left_color);

    let mut right_rect = Rect::from_x_y_w_h(rect.x(), rect.y(), split_width, split_height);
    right_rect = right_rect.shift_x(shift_amt);
    // draw.rect()
    //     .xy(right_rect.xy())
    //     .wh(right_rect.wh())
    //     .color(right_color);
    draw_rectangular_prism(draw, right_rect, right_color);
    
    return (left_rect, right_rect);
}

fn split_rect_horizontal(draw: &Draw, rect: Rect<f32>, top_color: Srgba<u8>, bot_color: Srgba<u8>) -> (Rect<f32>, Rect<f32>) {
    let margin = 10.0;
    let split_height = rect.h()/2.0 - 1.5*margin;
    let split_width = rect.w() - 2.0*margin;
    let shift_amt = split_height/2.0 + margin/2.0;

    let mut top_rect = Rect::from_x_y_w_h(rect.x(), rect.y(), split_width, split_height);
    top_rect = top_rect.shift_y(shift_amt);
    // draw.rect()
    //     .xy(top_rect.xy())
    //     .wh(top_rect.wh())
    //     .color(top_color);
    draw_rectangular_prism(draw, top_rect, top_color);

    let mut bot_rect = Rect::from_x_y_w_h(rect.x(), rect.y(), split_width, split_height);
    bot_rect = bot_rect.shift_y(-shift_amt);
    // draw.rect()
    //     .xy(bot_rect.xy())
    //     .wh(bot_rect.wh())
    //     .color(bot_color);
    draw_rectangular_prism(draw, bot_rect, bot_color);

    
    return (top_rect, bot_rect);
    
}

fn draw_rectangular_prism(draw: &Draw, rect: Rect<f32>, color: Srgba<u8>){
    // let mut shift_rect = rect.shift_x(50.0);
    // shift_rect = shift_rect.shift_y(50.0);
    // draw.rect()
    //     .xy(shift_rect.xy())
    //     .wh(shift_rect.wh())
    //     .color(color.into_format::<f32, u8>().into_hsl().shift_hue(100.0));

    // draw.rect()
    //     .xy(rect.xy())
    //     .wh(rect.wh())
    //     .color(color);

    draw.ellipse()
        .xy(rect.xy())
        // .wh(rect.wh())
        .radius(10.0)
        .color(color);

    

}