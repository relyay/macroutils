use std::env;
use std::fs;

fn main() {
    for d in env::args().skip(1) {
        if let Err(e) = fs::create_dir(&d) {
            eprintln!("{e}");
        }
    }
}