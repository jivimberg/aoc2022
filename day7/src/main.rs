use std::collections::{HashMap, HashSet};
use std::fmt;

use itertools::Itertools;

fn main() {
    let mut dirs: HashSet<Vec<String>> = HashSet::new();
    dirs.insert(vec!["/".to_string()]); // insert root
    let mut filesystem: HashSet<File> = HashSet::new();
    let mut current_path: Vec<String> = vec![];
    for line in include_str!("my_input.txt").lines() {
        let tokens: Vec<&str> = line
            .split_whitespace()
            .collect::<Vec<&str>>();
        match tokens.as_slice() {
            ["$", "ls"] => (), // do nothing on ls
            ["$", "cd", ".."] => { current_path.pop(); }
            ["$", "cd", dir] => current_path.push(dir.to_string()),
            ["dir", dir] => {
                let mut listed_dir = current_path.clone();
                listed_dir.push(dir.to_string());
                dirs.insert(listed_dir);
            },
            [file_size, file_name] => {
                filesystem.insert(
                    File {
                        dir: current_path.clone(),
                        name: file_name.to_string(),
                        size: file_size.parse::<usize>().expect("Has to be a valid number"),
                    }
                );
            }
            _ => panic!("Unknown output 💥")
        }
    }

    // a map of dir to size without counting recursive dirs
    let dir_partial_size_map: HashMap<Vec<String>, usize> = filesystem
        .iter()
        .into_grouping_map_by(|file| file.dir.clone())
        .fold(0, |acc, _key, file| acc + file.size);

    let mut dir_full_size_map: HashMap<Vec<String>, usize> = HashMap::new();
    for dir in dirs {
        let full_size = dir_partial_size_map
            .iter()
            .filter(|(path, _)| path.starts_with(&dir))
            .fold(0, |acc, (_path, size)| acc + size);
        dir_full_size_map.insert(dir.clone(), full_size);
    }


    // Part 1
    let size_threshold = 100000;
    let result: usize = dir_full_size_map
        .values()
        .filter(|&&size| size <= size_threshold)
        .sum();

    dbg!(result);

    // Part 2
    dbg!(&dir_full_size_map);
    let needed_space = 30000000;
    let total_space = 70000000;
    let used_space = dir_full_size_map
        .get(&vec!["/".to_string()])
        .expect("Root has to be there");
    let space_to_free_up = needed_space - (total_space - used_space);

    let (_dir_to_delete, size) = dir_full_size_map
        .iter()
        .sorted_by_key(|(_path, &size)| size)
        .find(|(_path, &size)| size > space_to_free_up)
        .expect("There has to be something we can delete");

    dbg!(size);
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct File {
    dir: Vec<String>,
    name: String,
    size: usize,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}/{} {}", self.dir[1..].join("/"), self.name, self.size)
    }
}
