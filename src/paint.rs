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


use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;

use ::bindings as ffi;
use ::color::Color;
use ::{XfermodeMode, MaskFilter, Shader, Typeface, Rect};

pub use self::ffi::sk_stroke_cap_t as StrokeCap;
pub use self::ffi::sk_stroke_join_t as StrokeJoin;
pub use self::ffi::sk_font_metrics_t as FontMetrics;
pub use self::ffi::sk_text_encoding_t as TextEncoding;

pub struct Paint {
    pub(crate) native_pointer: *mut ffi::sk_paint_t
}

impl Paint {
    pub fn new() -> Paint {
        let pointer = unsafe { ffi::sk_paint_new() };

        if pointer.is_null() {
            panic!("Cannot create paint object");
        }

        Paint {
            native_pointer: pointer,
        }
    }

    pub fn is_antialias(&self) -> bool {
        unsafe { ffi::sk_paint_is_antialias(self.native_pointer) }
    }

    pub fn set_antialias(&mut self, antialias: bool) {
        unsafe { ffi::sk_paint_set_antialias(self.native_pointer, antialias) }
    }

    pub fn set_dither(&mut self, dither: bool) {
        unsafe { ffi::sk_paint_set_dither(self.native_pointer, dither) };
    }

    pub fn is_dither(&self) -> bool {
        unsafe { ffi::sk_paint_is_dither(self.native_pointer) }
    }

    pub fn get_color(&self) -> Color {
        unsafe { Color::from_native(ffi::sk_paint_get_color(self.native_pointer)) }
    }

    pub fn set_color(&mut self, color: &Color) {
        unsafe { ffi::sk_paint_set_color(self.native_pointer, color.get_native()) }
    }

    pub fn is_stroke(&self) -> bool {
        unsafe { ffi::sk_paint_is_stroke(self.native_pointer) }
    }

    pub fn set_stroke(&mut self, stroke: bool) {
        unsafe { ffi::sk_paint_set_stroke(self.native_pointer, stroke) };
    }

    pub fn get_stroke_width(&self) -> f32 {
        unsafe { ffi::sk_paint_get_stroke_width(self.native_pointer) }
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { ffi::sk_paint_set_stroke_width(self.native_pointer, width) }
    }

    pub fn get_stroke_miter(&self) -> f32 {
        unsafe { ffi::sk_paint_get_stroke_miter(self.native_pointer) }
    }

    pub fn set_stroke_miter(&mut self, miter: f32) {
        unsafe { ffi::sk_paint_set_stroke_miter(self.native_pointer, miter) };
    }

    pub fn get_stroke_cap(&self) -> StrokeCap {
        unsafe { ffi::sk_paint_get_stroke_cap(self.native_pointer) }
    }

    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        unsafe { ffi::sk_paint_set_stroke_cap(self.native_pointer, cap) };
    }

    pub fn get_stroke_join(&self) -> StrokeJoin {
        unsafe { ffi::sk_paint_get_stroke_join(self.native_pointer) }
    }

    pub fn set_stroke_join(&mut self, join: StrokeJoin) {
        unsafe { ffi::sk_paint_set_stroke_join(self.native_pointer, join) }
    }

    pub fn set_text_size(&mut self, size: f32) {
        unsafe { ffi::sk_paint_set_text_size(self.native_pointer, size) };
    }

    pub fn get_text_size(&self) -> f32 {
        unsafe { ffi::sk_paint_get_text_size(self.native_pointer) }
    }

    pub fn set_typeface(&mut self, typeface: &Typeface) {
        unsafe { ffi::sk_paint_set_typeface(self.native_pointer, typeface.native_pointer) };
    }

    pub fn get_font_metrics(&self, scale: f32) -> FontMetrics {
        let mut font_metrics = FontMetrics {
            fFlags: 0,
            fTop: 0.0,
            fAscent: 0.0,
            fDescent: 0.0,
            fBottom: 0.0,
            fLeading: 0.0,
            fAvgCharWidth: 0.0,
            fMaxCharWidth: 0.0,
            fXMin: 0.0,
            fXMax: 0.0,
            fXHeight: 0.0,
            fCapHeight: 0.0,
            fUnderlineThickness: 0.0,
            fUnderlinePosition: 0.0,
        };

        unsafe { ffi::sk_paint_get_font_metrics(self.native_pointer, &mut font_metrics, scale) };

        font_metrics
    }

    pub fn measure_text(&self, text: &str) -> (f32, Rect) {
        let ctext = CString::new(text).unwrap();
        let mut rect = Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };

        let width = unsafe { ffi::sk_paint_measure_text(self.native_pointer,
                                                        ctext.as_ptr() as *const c_void,
                                                        text.len(), &mut rect) };

        (width, rect)
    }

    pub fn measure_blob(&self, blob: &[u16]) -> (f32, Rect) {
        let mut rect = Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };

        let width = unsafe { ffi::sk_paint_measure_text(self.native_pointer,
                                                        blob.as_ptr() as *const c_void,
                                                        2*blob.len(), &mut rect) };

        (width, rect)
    }

    pub fn set_text_encoding(&mut self, encoding: TextEncoding) {
        unsafe { ffi::sk_paint_set_text_encoding(self.native_pointer, encoding) };
    }

    pub fn get_text_encoding(&self) -> TextEncoding {
        unsafe { ffi::sk_paint_get_text_encoding(self.native_pointer) }
    }

    pub fn set_shader(&mut self, shader: &Shader) {
        unsafe { ffi::sk_paint_set_shader(self.native_pointer, shader.native_pointer) };
    }

    pub fn set_maskfilter(&mut self, filter: &MaskFilter) {
        unsafe { ffi::sk_paint_set_maskfilter(self.native_pointer, filter.native_pointer) };
    }

    pub fn set_xfermode_mode(&mut self, mode: XfermodeMode) {
        unsafe { ffi::sk_paint_set_xfermode_mode(self.native_pointer, mode) };
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe { ffi::sk_paint_delete(self.native_pointer) };
    }
}