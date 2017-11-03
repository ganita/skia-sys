extern crate skia_sys;

use std::fs::File;
use std::io::Write;

use skia_sys::*;

fn main() {
    let image_info = ImageInfo {
        width: 640,
        height: 480,
        colorType: ColorType::default_8888(),
        alphaType: AlphaType::PREMUL_SK_ALPHATYPE,
    };

    let surface = Surface::new_raster(&image_info, &None);
    let mut canvas = surface.get_canvas();

    let mut fill = Paint::new();
    fill.set_color(&Color { r: 0, g: 0, b: 255, a: 255, });
    canvas.draw_paint(&fill);

    fill.set_color(&Color { a: 255, r: 0, g: 255, b: 255 });
    canvas.draw_rect(&Rect {
        left: 100.0,
        top: 100.0,
        right: 540.0,
        bottom: 380.0,
    }, &fill);

    let mut stroke = Paint::new();
    stroke.set_color(&Color { a: 255, r: 255, g: 0, b: 0 });
    stroke.set_antialias(true);
    stroke.set_stroke(true);
    stroke.set_stroke_width(5.);

    let mut path = Path::new();
    path.move_to(50., 50.);
    path.line_to(590., 50.);
    path.cubic_to(-490., 50., 1130., 430., 50., 430.);
    path.line_to(590., 430.);
    canvas.draw_path(&path, &stroke);

    fill.set_color(&Color {a: 128, r: 0, g: 255, b: 0});
    canvas.draw_oval(&Rect {
        left: 120.0,
        top: 120.0,
        right: 520.0,
        bottom: 360.0,
    }, &fill);

    let snapshot = surface.new_image_snapshot();
    let data = snapshot.encode();

    let bytes = data.get_data();

    let mut file = File::create("target/safe.png").expect("Cannot create file");
    file.write(bytes).expect("Cannot write to file");

}