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

pub use ::bindings::sk_matrix_t as Matrix;


impl Matrix {
    pub fn set_identity(&mut self) {
        unsafe { sk_matrix_set_identity(self) };
    }

    pub fn set_translate(&mut self, tx: f32, ty: f32) {
        unsafe { sk_matrix_set_translate(self, tx, ty) };
    }

    pub fn pre_translate(&mut self, tx: f32, ty: f32) {
        unsafe { sk_matrix_pre_translate(self, tx, ty) };
    }

    pub fn post_translate(&mut self, tx: f32, ty: f32) {
        unsafe { sk_matrix_post_translate(self, tx, ty) };
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32) {
        unsafe { sk_matrix_set_scale(self, sx, sy) };
    }

    pub fn pre_scale(&mut self, sx: f32, sy: f32) {
        unsafe { sk_matrix_pre_scale(self, sx, sy) };
    }

    pub fn post_scale(&mut self, sx: f32, sy: f32) {
        unsafe { sk_matrix_post_scale(self, sx, sy) };
    }
}