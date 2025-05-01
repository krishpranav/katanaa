use std::env::consts::OS;
use colored::Colorize;

pub fn install_linker(linker: &str) {
    match OS {
        "windows" => {

        }
        "linux" => {
        }
        "macos" => {

        }

        &_ => {}
    }
}