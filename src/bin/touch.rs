use std::env;
use std::fs::OpenOptions;

fn main() {
    for f in env::args().skip(1) {
        let _ = OpenOptions::new().create(true).write(true).open(f);
    }
}