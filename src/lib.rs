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


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;

mod paint;              pub use self::paint::Paint;
mod color;              pub use self::color::Color;
mod canvas;             pub use self::canvas::Canvas;
mod path;               pub use self::path::Path;
mod image;              pub use self::image::Image;
mod picture;            pub use self::picture::Picture;
mod data;               pub use self::data::Data;
mod maskfilter;         pub use self::maskfilter::MaskFilter;
mod matrix;             pub use self::matrix::*;
mod shader;             pub use self::shader::*;
mod surface;            pub use self::surface::*;

pub use bindings::sk_xfermode_mode_t as XfermodeMode;
pub use bindings::sk_colortype_t as ColorType;
pub use bindings::sk_alphatype_t as AlphaType;
pub use bindings::sk_cliptype_t as ClipType;
pub use bindings::sk_pixelgeometry_t as PixelGeometry;
pub use bindings::sk_imageinfo_t as ImageInfo;
pub use bindings::sk_point_t as Point;
pub use bindings::sk_rect_t as Rect;
pub use bindings::sk_matrix_t as Matrix;
pub use bindings::sk_path_direction_t as PathDirection;
pub use bindings::sk_shader_tilemode_t as ShaderTileMode;

pub fn sk_color_set_argb(a: u8, r: u8, g: u8, b: u8) -> bindings::sk_color_t {
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

pub fn sk_color_get_a(color: bindings::sk_color_t) -> u8 {
    (color >> 24 & 0xFF) as u8
}

pub fn sk_color_get_r(color: bindings::sk_color_t) -> u8 {
    (color >> 16 & 0xFF) as u8
}

pub fn sk_color_get_g(color: bindings::sk_color_t) -> u8 {
    (color >> 8 & 0xFF) as u8
}

pub fn sk_color_get_b(color: bindings::sk_color_t) -> u8 {
    (color >> 0 & 0xFF) as u8
}
