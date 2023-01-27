/*

Copyright © 2022–2023 Piotr Bajdek

The Universal Permissive License (UPL), Version 1.0

Subject to the condition set forth below, permission is hereby granted to any
person obtaining a copy of this software, associated documentation and/or data
(collectively the "Software"), free of charge and under any and all copyright
rights in the Software, and any and all patent rights owned or freely
licensable by each licensor hereunder covering either (i) the unmodified
Software as contributed to or provided by such licensor, or (ii) the Larger
Works (as defined below), to deal in both

(a) the Software, and

(b) any piece of software and/or hardware listed in the lrgrwrks.txt file if
one is included with the Software (each a “Larger Work” to which the Software
is contributed by such licensors),

without restriction, including without limitation the rights to copy, create
derivative works of, display, perform, and distribute the Software and make,
use, sell, offer for sale, import, export, have made, and have sold the
Software and the Larger Work(s), and to sublicense the foregoing rights on
either these or other terms.

This license is subject to the following condition:

The above copyright notice and either this complete permission notice or at
a minimum a reference to the UPL must be included in all copies or
substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[cfg(windows)]
compile_error!("This doesn't work on Windows.");

use std::env;
use std::fs;
use std::process::exit;

pub mod info;

fn main() {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let blue_underlined = "\x1b[34;4m";
    let grey = "\x1b[38;5;240m";
    let violet = "\x1b[38;5;133m";
    let yellow = "\x1b[38;5;220m";
    let green = "\x1b[38;5;106m";

    let get_home = dirs::home_dir();
    let binding = get_home.unwrap();
    let home_path = binding.to_str();
    let home_str: &str = home_path.unwrap().trim();

    let cargo_bin_str = "/.cargo/bin/";

    let args: Vec<String> = env::args().collect();

    let arg_cnt = args.len();

    if arg_cnt == 2 {
        for file in fs::read_dir(home_str.to_owned() + cargo_bin_str).unwrap() {
            let get_file = file.as_ref().unwrap().path();
            let file_clone = get_file.clone();
            let fl_name = file_clone.file_name().unwrap().to_str();
            let fl_nm: &str = fl_name.unwrap().trim();

            if fl_nm == "cargo" || fl_nm == "cargo-clippy" || fl_nm == "cargo-fmt" || fl_nm == "cargo-miri" || fl_nm == "clippy-driver" || fl_nm == "rls" || fl_nm == "rustc" || fl_nm == "rustdoc" || fl_nm == "rustfmt" || fl_nm == "rust-gdb" || fl_nm == "rust-gdbgui" || fl_nm == "rust-lldb" || fl_nm == "rustup" {
                print!("{grey}");
                println!("{}", file.unwrap().path().display());
                print!("{reset}");
            } else if fl_nm.starts_with("cargo-") {
                print!("{violet}");
                println!("{}", file.unwrap().path().display());
                print!("{reset}");
            } else {
                print!("{green}");
                println!("{}", file.unwrap().path().display());
                print!("{reset}");
            }
        }
        exit(0);
    }

    let argument = args.get(2).unwrap();

    if argument == "--help" {
        info::help(reset, blue_underlined, grey, violet, yellow);
        exit(0);
    } else if argument == "--license" {
        info::license(reset, yellow);
        exit(0);
    } else if argument == "--version" {
        info::version(reset, grey, yellow);
        exit(0);
    } else if argument == "--default" {
        for file in fs::read_dir(home_str.to_owned() + cargo_bin_str).unwrap() {
            let get_file = file.as_ref().unwrap().path();
            let file_clone = get_file.clone();
            let fl_name = file_clone.file_name().unwrap().to_str();
            let fl_nm: &str = fl_name.unwrap().trim();

            if fl_nm == "cargo" || fl_nm == "cargo-clippy" || fl_nm == "cargo-fmt" || fl_nm == "cargo-miri" || fl_nm == "clippy-driver" || fl_nm == "rls" || fl_nm == "rustc" || fl_nm == "rustdoc" || fl_nm == "rustfmt" || fl_nm == "rust-gdb" || fl_nm == "rust-gdbgui" || fl_nm == "rust-lldb" || fl_nm == "rustup" {
                print!("{grey}");
                println!("{}", file.unwrap().path().display());
                print!("{reset}");
            }
        }
        exit(0);
    } else if argument == "--plugins" {
        for file in fs::read_dir(home_str.to_owned() + cargo_bin_str).unwrap() {
            let get_file = file.as_ref().unwrap().path();
            let file_clone = get_file.clone();
            let fl_name = file_clone.file_name().unwrap().to_str();
            let fl_nm: &str = fl_name.unwrap().trim();

            if fl_nm.starts_with("cargo-") {
                print!("{violet}");
                println!("{}", file.unwrap().path().display());
                print!("{reset}");
            }
        }
        exit(0);
    } else if arg_cnt > 2 {
        panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
    }
}
