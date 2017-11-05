/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


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
    fill.set_color(&Color { r: 255, g: 255, b: 255, a: 255, });
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

    let mut text_paint = Paint::new();
    text_paint.set_antialias(true);
    text_paint.set_color(&Color { a: 255, r: 0, g: 0, b: 0 });
    text_paint.set_dither(true);
    text_paint.set_text_size(64.);

    let root_dir = env!("CARGO_MANIFEST_DIR");
    let typeface = Typeface::new_from_file(&format!(
        "{}/examples/fonts/STIX2Math.otf",
        root_dir
    ), 0).unwrap();
    text_paint.set_typeface(&typeface);

    let metrics = text_paint.get_font_metrics(0.);
    println!("{:?}", metrics);
    println!("{:?}", text_paint.measure_text("Hello world!"));
    println!("{:?}", text_paint.get_text_encoding());

    text_paint.set_text_encoding(TextEncoding::kGlyphID_TextEncoding);
    println!("{:?}", text_paint.measure_blob(&[19, 19]));

    text_paint.set_text_encoding(TextEncoding::kUTF8_TextEncoding);
    println!("{:?}", text_paint.measure_text("QQ"));

    canvas.draw_text("Hello", 100., 100., &text_paint);

    text_paint.set_text_encoding(TextEncoding::kGlyphID_TextEncoding);
    canvas.draw_blob(&[4000, 2000], 400., 100., &text_paint);

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