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


use ::bindings::*;
use ::{Rect};

pub use ::bindings::sk_path_direction_t as PathDirection;

pub struct Path {
    pub(crate) native_pointer: *mut sk_path_t
}

impl Path {
    pub fn new() -> Path {
        let native_pointer = unsafe { sk_path_new() };
        if native_pointer.is_null() {
            panic!("Cannot create path");
        }

        Path { native_pointer }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe { sk_path_move_to(self.native_pointer, x, y) };
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        unsafe { sk_path_line_to(self.native_pointer, x, y) };
    }

    pub fn quad_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        unsafe { sk_path_quad_to(self.native_pointer, x0, y0, x1, y1) };
    }

    pub fn conic_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, w: f32) {
        unsafe { sk_path_conic_to(self.native_pointer, x0, y0, x1, y1, w) };
    }

    pub fn cubic_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { sk_path_cubic_to(self.native_pointer, x0, y0, x1, y1, x2, y2) };
    }

    pub fn close(&mut self) {
        unsafe { sk_path_close(self.native_pointer) };
    }

    pub fn add_rect(&mut self, rect: &Rect, direction: PathDirection) {
        unsafe { sk_path_add_rect(self.native_pointer, rect, direction) };
    }

    pub fn add_oval(&mut self, rect: &Rect, direction: PathDirection) {
        unsafe { sk_path_add_oval(self.native_pointer, rect, direction) };
    }

    pub fn get_bounds(&mut self, direction: PathDirection) -> Rect {
        let rect = Rect {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        };

        unsafe { sk_path_add_rect(self.native_pointer, &rect, direction) };

        rect
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { sk_path_delete(self.native_pointer) };
    }
}