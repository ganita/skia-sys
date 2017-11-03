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


use std::os::raw::c_void;

use ::bindings::*;
use ::{ImageInfo, Data};

pub use ::bindings::sk_irect_t as IRect;

pub struct Image {
    pub(crate) native_pointer: *mut sk_image_t,
}

impl Image {
    pub(crate) fn new_from_pointer(native_pointer: *mut sk_image_t) -> Option<Image> {
        if native_pointer.is_null() {
            None
        } else {
            Some(Image { native_pointer })
        }
    }

    pub fn new_raster_copy(image_info: &ImageInfo, pixels: &[c_void]) -> Option<Image> {
        unsafe { Image::new_from_pointer(
            sk_image_new_raster_copy(image_info, pixels.as_ptr(), pixels.len())
        ) }
    }

    pub fn new_from_encoded(data: &Data, irect: &IRect) -> Option<Image> {
        unsafe { Image::new_from_pointer(
            sk_image_new_from_encoded(data.native_pointer, irect)
        ) }
    }

    pub fn encode(&self) -> Data {
        Data {
            native_pointer: unsafe { sk_image_encode(self.native_pointer) },
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { sk_image_get_width(self.native_pointer) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { sk_image_get_height(self.native_pointer) }
    }

    pub fn get_unique_id(&self) -> u32 {
        unsafe { sk_image_get_unique_id(self.native_pointer) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sk_image_unref(self.native_pointer) };
    }
}