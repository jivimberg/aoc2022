use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Error;
use std::str::FromStr;

use itertools::Itertools;

fn main() -> Result<(), io::Error> {
    let input = include_str!("my_input.txt");

    let index = ('a'..='z').chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c as char, idx + 1))
        .collect::<HashMap<char, usize>>();

    println!("{:?}", index);

    // Part 1
    let sum: usize = input
        .lines()
        .map(|line| line.parse::<Rucksack>())
        .filter_map_ok(|rucksack| rucksack.find_misplaced())
        .map(|missplaced| index[&missplaced.unwrap()])
        .sum();

    dbg!(sum);

    // Part 2
    let sum2: usize = input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(l1, l2, l3)| l3.chars().find(|c| l1.contains(*c) && l2.contains(*c)).unwrap())
        .map(|badge| index[&badge])
        .sum();

    dbg!(sum2);
    Ok(())
}

#[derive(Debug, Clone)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mid = s.len() / 2;

        Ok(Self {
            compartment1: s[..mid].to_string(),
            compartment2: s[mid..].to_string(),
        })
    }
}

impl Rucksack {
    fn find_misplaced(&self) -> Option<char> {
        self.compartment1.chars().find(|c| self.compartment2.contains(*c))
    }
}
