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


use std::mem::forget;
use std::slice;
use std::os::raw::c_void;

use ::bindings::*;

pub struct Data {
    pub(crate) native_pointer: *mut sk_data_t,
}

impl Data {
    fn new_from_pointer(native_pointer: *mut sk_data_t) -> Data {
        if native_pointer.is_null() {
            panic!("Cannot create empty data");
        }

        Data {
            native_pointer
        }
    }

    pub fn new_empty() -> Data {
        let native_pointer = unsafe { sk_data_new_empty() };
        Data::new_from_pointer(native_pointer)
    }

    pub fn new_with_copy(bytes: &[c_void]) -> Data {
        let native_pointer = unsafe { sk_data_new_with_copy(bytes.as_ptr(), bytes.len()) };
        Data::new_from_pointer(native_pointer)
    }

    pub fn new_from_malloc(bytes: Vec<c_void>) -> Data {
        forget(&bytes);
        let native_pointer = unsafe { sk_data_new_from_malloc(bytes.as_ptr(), bytes.len()) };
        Data::new_from_pointer(native_pointer)
    }

    pub fn new_from_subset(data: &Data, offset: usize, length: usize) -> Data {
        let native_pointer = unsafe { sk_data_new_subset(data.native_pointer, offset, length) };
        Data::new_from_pointer(native_pointer)
    }

    pub fn size(&self) -> usize {
        unsafe { sk_data_get_size(self.native_pointer) }
    }

    pub fn get_data(&self) -> &[c_void] {
        let slice = unsafe { sk_data_get_data(self.native_pointer) };
        unsafe { slice::from_raw_parts(slice, self.size()) }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { sk_data_unref(self.native_pointer) };
    }
}