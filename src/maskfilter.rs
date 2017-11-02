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

pub use ::bindings::sk_blurstyle_t as BlurStyle;

pub struct MaskFilter {
    pub(crate) native_pointer: *mut sk_maskfilter_t,
}

impl MaskFilter {
    pub fn new_blur(&mut self, blurstyle: BlurStyle, sigma: f32) -> MaskFilter {
        let native_pointer = unsafe { sk_maskfilter_new_blur(blurstyle, sigma) };
        if native_pointer.is_null() {
            panic!("Unable to create mask filter");
        }

        MaskFilter {
            native_pointer
        }
    }
}

impl Drop for MaskFilter {
    fn drop(&mut self) {
        unsafe { sk_maskfilter_unref(self.native_pointer) };
    }
}