use std::env;
use std::fs;

fn main() {
    for filename in env::args().skip(1) {
        match fs::read_to_string(&filename) {
            Ok(content) => print!("{}", content),
            Err(e) => eprintln!("{e}"),
        }
    }
}
