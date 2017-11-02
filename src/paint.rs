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


use ::bindings as ffi;
use ::color::Color;
use ::{XfermodeMode, StrokeCap, StrokeJoin};

pub struct Paint {
    native_pointer: *mut ffi::sk_paint_t
}

pub type Shader = ffi::sk_shader_t;
pub type MaskFilter = ffi::sk_maskfilter_t;

impl Paint {
    pub fn new() -> Paint {
        Paint {
            native_pointer: unsafe { ffi::sk_paint_new() },
        }
    }

    pub fn is_antialias(&self) -> bool {
        unsafe { ffi::sk_paint_is_antialias(self.native_pointer) }
    }

    pub fn set_antialias(&mut self, antialias: bool) {
        unsafe { ffi::sk_paint_set_antialias(self.native_pointer, antialias) }
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

    pub fn set_shader(&mut self, shader: &mut Shader) {
        unsafe { ffi::sk_paint_set_shader(self.native_pointer, shader) };
    }

    pub fn set_maskfilter(&mut self, filter: &mut MaskFilter) {
        unsafe { ffi::sk_paint_set_maskfilter(self.native_pointer, filter) };
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