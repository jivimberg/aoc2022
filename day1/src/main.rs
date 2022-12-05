extern crate core;

use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("src/my_input.txt")?;

    // Part 1
    let lines = io::BufReader::new(file).lines()
        .map(|result| result.unwrap())
        .collect::<Vec<String>>();

    let calories = lines
        .as_slice()
        .split(|line| line.is_empty()) // Split on empty lines
        .map(|calories_on_elf| {
            // sum calories
            calories_on_elf
                .iter()
                .map(|calories_string| calories_string.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();

    let max_calories = calories
        .iter()
        .max()
        .unwrap();
    println!("{max_calories}");

    // Part 2
    let mut _calories: Vec<i32> = calories.to_owned();
    _calories.sort();
    _calories.reverse();

    let top_3_sum: i32 = _calories.iter()
        .take(3)
        .sum();

    println!("{top_3_sum}");
    Ok(())
}

// See https://fasterthanli.me/series/advent-of-code-2022/part-1#part-2 for step-by-step solution
