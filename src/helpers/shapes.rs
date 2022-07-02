use nannou::{prelude::*, color::Shade};

// Draw an isometric cube centered at x y
#[allow(dead_code)]
pub fn draw_iso_cube(draw: &Draw, x: f32, y: f32, width: f32, color: Hsla) {
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