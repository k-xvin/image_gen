use nannou::prelude::*;

struct Model {
    draw_once: bool,
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

    Model {
        draw_once: true,
    }
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Space {
     app.main_window().capture_frame("screenshots/screenshot.png");
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);


    let center_rect: Rect<f32> = Rect::from_w_h(400.0, 400.0);
    draw.rect()
        .xy(center_rect.xy())
        .wh(center_rect.wh())
        .color(BLACK); 
    

    let mut curr_size = center_rect.w();
    let mut flip = true;
    while curr_size > 1.0 {
        let r = Rect::from_w_h(curr_size, curr_size);
        draw_corners(&draw, r,
            if flip {
                WHITE
            }
            else {
                BLACK
            },
        );

        flip = !flip;
        curr_size /= 1.1;
    }

    

    if model.draw_once {
        if app.elapsed_frames() == 1 {
            draw.to_frame(app, &frame).unwrap();
        }
    }
    else {
        draw.to_frame(app, &frame).unwrap();
    }
}

fn draw_corners(draw: &Draw, rect: Rect<f32>, color: Srgb<u8>) {
    let r = Rect::from_w_h(rect.w()/2.0, rect.h()/2.0).top_right_of(rect);
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .color(color);
    
    let r = Rect::from_w_h(rect.w()/2.0, rect.h()/2.0).bottom_left_of(rect);
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .color(color);
}