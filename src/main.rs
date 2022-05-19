// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use learn_rust_lib::run;

fn main() {
    pollster::block_on(run());
}
