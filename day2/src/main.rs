extern crate core;

use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("src/my_input.txt")?;

    let plays = io::BufReader::new(file)
        .lines()
        .map(|result| {
            let line: String = result.unwrap();
            let mut chars = line.chars();
            (chars.next().unwrap(), chars.last().unwrap())
        })
        .collect::<Vec<(char, char)>>();

    // Part 1
    let total_points_part1: u32 = plays
        .iter()
        .map(|(their_pick, our_pick)| {
            let our_pick_num_value = (*our_pick as u32) - 87;
            let their_pick_numb_value = (*their_pick as u32) - 64;
            let points_for_pick = our_pick_num_value;
            let points_for_result = match (their_pick_numb_value, our_pick_num_value) {
                (1, 3) => 0, // rock scissors => we lose
                (3, 1) => 6, // scissors rock => we win
                (p1, p2) if (p1 < p2) => 6,
                (p1, p2) if (p1 == p2) => 3,
                (p1, p2) if (p1 > p2) => 0,
                _ => panic!("Impossible")
            };

            // Debug
            // let theirs = match their_pick {
            //     'A' | 'X' => "Rock",
            //     'B' | 'Y' => "Paper",
            //     'C' | 'Z' => "Scissors",
            //     _ => panic!("option not valid")
            // };
            // let ours = match our_pick {
            //     'A' | 'X' => "Rock",
            //     'B' | 'Y' => "Paper",
            //     'C' | 'Z' => "Scissors",
            //     _ => panic!("option not valid")
            // };
            // println!("{theirs}, {ours}");
            // println!("Points for pick: {:?}", points_for_pick);
            // println!("Points for result: {:?}", points_for_result);
            // println!("Total: {:?}", points_for_pick + points_for_result);
            // println!(" ");

            points_for_pick + points_for_result
        })
        .sum();

    println!("Total points: {:?}", total_points_part1);

    // Part 2

    let total_points_part2: u32 = plays
        .iter()
        .map(|(their_pick, result)| {
            let their_pick_numb_value = (*their_pick as u32) - 64;
            let points_for_pick = match result {
                'X' => if their_pick_numb_value - 1 > 0 { their_pick_numb_value - 1 } else { 3 },
                'Y' => their_pick_numb_value,
                'Z' => if their_pick_numb_value + 1 <= 3 { their_pick_numb_value + 1 } else { 1 },
                _ => panic!("Impossible")
            };
            let points_for_result = match result {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => panic!("Impossible")
            };

            // Debug
            let theirs = match their_pick {
                'A' | 'X' => "Rock",
                'B' | 'Y' => "Paper",
                'C' | 'Z' => "Scissors",
                _ => panic!("option not valid")
            };
            let ours = match points_for_pick {
                1 => "Rock",
                2 => "Paper",
                3 => "Scissors",
                _ => panic!("option not valid")
            };
            let result = match result {
                'X' => "We have to lose",
                'Y' => "We have to tie",
                'Z' => "We have to win",
                _ => panic!("option not valid")
            };
            println!("{theirs}, {ours}");
            println!("Result needed: {result}");
            println!("Points for pick: {:?}", points_for_pick);
            println!("Points for result: {:?}", points_for_result);
            println!("Total: {:?}", points_for_pick + points_for_result);
            println!(" ");

            points_for_pick + points_for_result
        })
        .sum();


    println!("Total points: {:?}", total_points_part2);

    Ok(())
}

// See https://fasterthanli.me/series/advent-of-code-2022/part-2
