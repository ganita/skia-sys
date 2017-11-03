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


use std::marker::PhantomData;
use std::ptr;

use ::bindings::*;
use ::{ImageInfo, Canvas, Image};

pub use ::bindings::sk_surfaceprops_t as SurfaceProps;

pub struct Surface {
    pub(crate) native_pointer: *mut sk_surface_t,
}

impl Surface {
    pub fn new_raster(image_info: &ImageInfo, surface_props: &Option<SurfaceProps>) -> Surface {
        let surface_props = if let Some(ref val) = *surface_props {
            val
        } else {
            ptr::null()
        };

        let native_pointer = unsafe { sk_surface_new_raster(image_info, surface_props) };
        if native_pointer.is_null() {
            panic!("Cannot create surface");
        }

        Surface { native_pointer }
    }

    pub fn get_canvas<'a>(&'a self) -> Canvas<'a> {
        let canvas_ptr = unsafe { sk_surface_get_canvas(self.native_pointer) };
        Canvas { native_pointer: canvas_ptr, phantom: PhantomData }
    }

    pub fn new_image_snapshot(&self) -> Image {
        Image::new_from_pointer(unsafe { sk_surface_new_image_snapshot(self.native_pointer) }).unwrap()
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { sk_surface_unref(self.native_pointer) };
    }
}