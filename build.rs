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


extern crate cmake;

use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let skia_out_dir = format!("{}/skia", &out_dir);

    assert!(Command::new("python")
                .current_dir("skia")
                .args(&["tools/git-sync-deps"])
                .status().unwrap().success(), "Cannot download skia depenedencies");

    assert!(Command::new("bin/gn")
                .current_dir("skia")
                .args(&["gen", &skia_out_dir, "--args=is_official_build=true"])
                .status().unwrap().success(), "Cannot generate build files");

    assert!(Command::new("ninja")
                .current_dir("skia")
                .args(&["-C", &skia_out_dir])
                .status().unwrap().success(), "Cannot build skia");

    println!("cargo:rustc-link-search=native={}", skia_out_dir);
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(target_os="macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=CoreServices");
    }

    let mut cmd = cmake::Config::new("bridge");
    let dest = cmd.build();

    println!("cargo:rustc-link-search=native={}/build/libs", dest.display());
    println!("cargo:rustc-link-lib=static=skia_rust");

}