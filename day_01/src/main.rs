#![feature(array_windows)]
use std::{io::BufRead, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::io::Cursor::new(include_str!("input"));

    println!("{:#?}",
        input
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?
        .array_windows::<3>()
        .map(|window| window.iter().sum::<u32>())
        .collect::<Vec<_>>()
        .array_windows::<2>()
        .filter(|[a,b]| a < b)
        .count());

    Ok(())
}
