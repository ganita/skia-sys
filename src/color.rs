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
use ::sk_color_set_argb;
use ::sk_color_get_r;
use ::sk_color_get_g;
use ::sk_color_get_b;
use ::sk_color_get_a;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_native(color: ffi::sk_color_t) -> Color {
        Color {
            r: sk_color_get_r(color),
            g: sk_color_get_g(color),
            b: sk_color_get_b(color),
            a: sk_color_get_a(color),
        }
    }

    pub fn get_native(&self) -> ffi::sk_color_t {
        sk_color_set_argb(self.a, self.r, self.g, self.b)
    }
}