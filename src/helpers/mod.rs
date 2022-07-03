pub mod shapes;

use nannou::{prelude::*, color::rgb_u32};

/// Capture output at 30fps (skip every other frame)
/// Place into the update() function
#[allow(dead_code)]
pub fn save_at_30fps(app: &App, filename: &str) {
    let frames_to_skip = 2;
    if app.elapsed_frames() % frames_to_skip != 0 {
        return;
    }

    app.main_window().capture_frame(format!("screenshots/{}/{}_{}.jpg", filename, filename, app.elapsed_frames()/frames_to_skip));
}

/// Convert a color from RGB to HSLA
/// Usage: hsla_from_hex_rgb(0x264653),
/// Based on: https://github.com/nannou-org/nannou/blob/70fa9c1cccab8a9b998c9c65b2c03c9d8983f8e7/examples/ui/egui/circle_packing.rs#L170
#[allow(dead_code)]
pub fn hsla_from_hex_rgb(color: u32) -> Hsla {
    let color = rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}

#[allow(dead_code)]
pub fn hsla_from_rgb_u8(color: Rgb<u8>) -> Hsla {
    // let color = rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}
