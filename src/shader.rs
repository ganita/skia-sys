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
use ::{Point, Color, Matrix};

pub use ::bindings::sk_shader_tilemode_t as TileMode;

pub struct Shader {
    pub(crate) native_pointer: *mut sk_shader_t,
}

impl Shader {
    pub fn new_linear_gradient(points: [Point; 2], colors: Vec<Color>, color_pos: Vec<f32>,
                               tile_mode: TileMode, matrix: &Matrix) -> Shader {
        let colors: Vec<u32> = colors.iter().map(|c| c.get_native()).collect();

        let native_pointer = unsafe { sk_shader_new_linear_gradient(
            points.as_ptr(),
            colors[..].as_ptr(),
            color_pos[..].as_ptr(),
            colors.len() as i32,
            tile_mode,
            matrix
        ) };

        if native_pointer.is_null() {
            panic!("Cannot create gradient");
        }

        Shader { native_pointer }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { sk_shader_unref(self.native_pointer) };
    }
}