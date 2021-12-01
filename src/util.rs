use std::io::{stdin, BufRead};

pub fn lines() -> Vec<String> {
    let stdin = stdin();

    stdin.lock().lines()
        .filter_map(|r| r.ok())
        .collect()
}

pub fn nums() -> Vec<isize> {
    lines().iter()
        .filter_map(|line| line.parse::<isize>().ok())
        .collect()
}

#[macro_export]
macro_rules! solution {
    ($n:literal: $x:expr) => {
        println!("Solution {}: {}", $n, $x)
    }
}