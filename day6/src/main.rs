use std::collections::HashSet;

fn main() {
    include_str!("my_input.txt").lines()
        .map(find_start_idx)
        .for_each(|it| println!("{it}"))
}

fn find_start_idx(s: &str) -> usize {
    let window_size = 14;
    let first_idx = s.as_bytes()
        //.windows(window_size)
        .windows(window_size)
        .position(|window| {
            let distinct = window.iter().collect::<HashSet<_>>();
            window.len() == distinct.len()
        })
        .unwrap();
    first_idx + window_size
}
