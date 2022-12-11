extern crate core;

use std::collections::VecDeque;
use std::io::Error;

use itertools::Itertools;

fn main() -> Result<(), Error> {
    let mut input = include_str!("my_input.txt").lines();

    let number_of_stacks = input
        .clone()
        .find(|s| s.chars().all(|c| c.is_numeric() || c.is_whitespace()))
        .expect("There should be a line with numbers like: 1 2 3")
        .split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .max()
        .expect("There should be at least one stack");

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); number_of_stacks];

    input
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(parse_crates_line)
        .for_each(|cols| {
            cols
                .iter()
                .enumerate()
                .for_each(|col| {
                    if let (idx, Some(c)) = col {
                        stacks[idx].push_front(*c)
                    }
                })
        });


    println!("{:?}", stacks);


    input
        .map(parse_moves)
        .for_each(|(amount, from, to)| {
            //do_move_part1(&mut stacks, amount, from, to);
            do_move_part2(&mut stacks, amount, from, to);
        });


    println!("{:?}", stacks);

    let result: String = stacks
        .iter()
        .filter_map(|stack| stack.back())
        .collect();
    println!("{result}");

    Ok(())
}

fn do_move_part1(stacks: &mut [VecDeque<char>], amount: u32, from: u32, to: u32) {
    for _ in 0..amount {
        let from = (from - 1) as usize;
        let to = (to - 1) as usize;
        let crate_being_moved = stacks[from].pop_back().expect("There was nothing to move");
        stacks[to].push_back(crate_being_moved)
    }
}

fn do_move_part2(stacks: &mut [VecDeque<char>], amount: u32, from: u32, to: u32) {
    let from = (from - 1) as usize;
    let from_deque = &mut stacks[from];
    let mut crates_being_moved = from_deque
        .drain((from_deque.len() - amount as usize)..)
        .collect::<VecDeque<char>>();
    let to = (to - 1) as usize;
    stacks[to].append(&mut crates_being_moved)
}

fn parse_crates_line(s: &str) -> Vec<Option<char>> {
    let mut result: Vec<Option<char>> = vec![];
    let mut chars = s.chars();
    let mut idx = 1;
    while let Some(letter_or_whitespace) = chars.nth(idx) {
        if letter_or_whitespace.is_alphabetic() {
            result.push(Some(letter_or_whitespace))
        } else {
            result.push(None)
        }
        idx = 3;
    }
    result
}

fn parse_moves(s: &str) -> (u32, u32, u32) {
    if let Some((amount, from, to)) = s.split_whitespace()
        .flat_map(|s| s.parse::<u32>())
        .collect_tuple() {
        (amount, from, to)
    } else {
        panic!("Instruction not properly formated")
    }
}
