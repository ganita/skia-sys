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

use ::bindings::*;

pub struct Typeface {
    pub(crate) native_pointer: *mut sk_typeface_t,
}

impl Typeface {
    pub fn new_from_file(path: &str, index: u32) -> Option<Typeface> {
        let c_str = CString::new(path).unwrap();
        let native_pointer = unsafe { sk_typeface_new_from_file(c_str.as_ptr(), index) };

        if native_pointer.is_null() {
            return None;
        }

        Some(Typeface { native_pointer })
    }
}

impl Drop for Typeface {
    fn drop(&mut self) {
        unsafe { sk_typeface_unref(self.native_pointer) };
    }
}