use std::fmt::Display;
use std::time::Instant;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

pub fn run<I, T: Display>(func: impl FnOnce(I) -> T, input: I) {
    let timer = Instant::now();
    let result = func(input);
    let elapsed = timer.elapsed();
    println!("{result} (elapsed: {elapsed:.2?})");
}
