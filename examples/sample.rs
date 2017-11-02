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

use std::ptr;
use std::fs::File;
use std::slice;
use std::io::Write;

use skia_sys::*;

fn main() {
    unsafe {
        let image_info = sk_imageinfo_t {
            width: 640,
            height: 480,
            colorType: sk_colortype_get_default_8888(),
            alphaType: sk_alphatype_t::PREMUL_SK_ALPHATYPE,
        };
        let surface = sk_surface_new_raster(&image_info, ptr::null());
        let canvas = sk_surface_get_canvas(surface);

        let fill = sk_paint_new();
        sk_paint_set_color(fill, sk_color_set_argb(255, 0, 0, 255));
        sk_canvas_draw_paint(canvas, fill);

        sk_paint_set_color(fill, sk_color_set_argb(255, 0, 255, 255));
        let rect = sk_rect_t {
            left: 100.0,
            top: 100.0,
            right: 540.0,
            bottom: 380.0,
        };
        sk_canvas_draw_rect(canvas, &rect, fill);

        let stroke = sk_paint_new();
        sk_paint_set_color(stroke, sk_color_set_argb(255, 255, 0, 0));
        sk_paint_set_antialias(stroke, true);
        sk_paint_set_stroke(stroke, true);
        sk_paint_set_stroke_width(stroke, 5.);

        let path = sk_path_new();
        sk_path_move_to(path, 50., 50.);
        sk_path_line_to(path, 590., 50.);
        sk_path_cubic_to(path, -490., 50., 1130., 430., 50., 430.);
        sk_path_line_to(path, 590., 430.);
        sk_canvas_draw_path(canvas, path, stroke);

        sk_paint_set_color(fill, sk_color_set_argb(128, 0, 255, 0));
        let rect = sk_rect_t {
            left: 120.0,
            top: 120.0,
            right: 520.0,
            bottom: 360.0,
        };
        sk_canvas_draw_oval(canvas, &rect, fill);

        sk_path_delete(path);
        sk_paint_delete(fill);
        sk_paint_delete(stroke);

        let image = sk_surface_new_image_snapshot(surface);
        let data = sk_image_encode(image);

        sk_image_unref(image);

        let bytes = sk_data_get_data(data) as *const u8;
        let len = sk_data_get_size(data);

        let bytes = slice::from_raw_parts(bytes, len);

        let mut file = File::create("target/test.png").expect("Cannot create file");
        file.write(bytes).expect("Cannot write to file");

        sk_data_unref(data);

        sk_surface_unref(surface);
    }
}