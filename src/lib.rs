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

mod bindings;

pub use bindings::*;

pub fn sk_color_set_argb(a: u8, r: u8, g: u8, b: u8) -> sk_color_t {
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

pub fn sk_color_get_a(color: sk_color_t) -> u8 {
    (color >> 24 & 0xFF) as u8
}

pub fn sk_color_get_r(color: sk_color_t) -> u8 {
    (color >> 16 & 0xFF) as u8
}

pub fn sk_color_get_g(color: sk_color_t) -> u8 {
    (color >> 8 & 0xFF) as u8
}

pub fn sk_color_get_b(color: sk_color_t) -> u8 {
    (color >> 0 & 0xFF) as u8
}
