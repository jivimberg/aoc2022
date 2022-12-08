fn main() {
    let input = include_str!("my_input.txt");

    // Part 1
    let result: usize = input.lines()
        .filter(|line| {
            let boundaries: Vec<u32> = line.split(',')
                .flat_map(|group| group.split('-'))
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let ((s1, e1), (s2, e2)) = ((boundaries[0], boundaries[1]), (boundaries[2], boundaries[3]));
            (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1)
        })
        .count();

    // Part 2
    let result2: usize = input.lines()
        .filter(|line| {
            let boundaries: Vec<u32> = line.split(',')
                .flat_map(|group| group.split('-'))
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let ((s1, e1), (s2, e2)) = ((boundaries[0], boundaries[1]), (boundaries[2], boundaries[3]));
            (s2 <= e1 && e2 >= s1) || (s1 <= s2 && e1 >= s2)
        })
        .count();

    dbg!(result2);

    ()
}
