use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;

fn main() {
    let mut forest: Vec<Vec<u32>> = vec![];

    for line in include_str!("my_input.txt").lines() {
        let tree_line = line.chars()
            .map(|char| char.to_digit(10).unwrap())
            .collect();
        forest.push(tree_line)
    }

    // max visibility on position. top, right, bottom, left
    // Here I'm using Visibility to kep max height on each direction for each tree
    let mut visibility_matrix: Vec<Vec<Tree>> = vec![vec![Tree::new(); forest.len()]; forest.len()];
    for (from_top, tree_line) in forest.iter().enumerate() {
        let from_bottom = tree_line.len() - 1 - from_top;
        for (from_left, tree_height) in tree_line.iter().enumerate() {
            let from_right = tree_line.len() - 1 - from_left;

            visibility_matrix[from_top][from_left].height = *tree_height;

            // forward
            let previous_max_top = from_top.checked_sub(1)
                .map_or(-1, |idx| visibility_matrix[idx][from_left].top_vis);
            let previous_top = from_top.checked_sub(1)
                .map_or(-1, |idx| forest[idx][from_left] as i32);

            visibility_matrix[from_top][from_left].top_vis = max(previous_max_top, previous_top);

            let previous_max_left = from_left.checked_sub(1)
                .map_or(-1, |idx| visibility_matrix[from_top][idx].left_vis);
            let previous_left = from_left.checked_sub(1)
                .map_or(-1, |idx| forest[from_top][idx] as i32);

            visibility_matrix[from_top][from_left].left_vis = max(previous_max_left, previous_left);

            // backwards
            let previous_max_bottom = visibility_matrix.get(from_bottom + 1)
                .map_or(-1, |tree_line| tree_line[from_right].bottom_vis);
            let previous_bottom = forest.get(from_bottom + 1)
                .map_or(-1, |tree_line| tree_line[from_right] as i32);

            visibility_matrix[from_bottom][from_right].bottom_vis = max(previous_max_bottom, previous_bottom);

            let previous_max_right = visibility_matrix[from_bottom].get(from_right + 1)
                .map_or(-1, |visibility| visibility.right_vis);
            let previous_right = forest[from_bottom].get(from_right + 1)
                .map_or(-1, |it| *it as i32);

            visibility_matrix[from_bottom][from_right].right_vis = max(previous_max_right, previous_right);
        }
    }

    for line in &visibility_matrix {
        println!("{:?}", line);
    }

    for line in &visibility_matrix {
        for tree in line {
            let print = if tree.is_visible() {
                "✅"
            } else {
                "❌"
            };
            print!("{:?} ", print);
        }
        println!()
    }

    // Part 1
    let number_of_visible_trees = visibility_matrix.iter()
        .flat_map(|tree_line| tree_line.iter())
        .filter(|visibility| visibility.is_visible())
        .count();

    dbg!(number_of_visible_trees);

    // Part 2
    let mut result: usize = 0;
    for (i, tree_line) in forest.iter().enumerate() {
        if i == 0 || i == forest.len() - 1 {
            // Ignore borders because we'd be multiplying by 0
            continue;
        }

        for (j, tree_height) in tree_line.iter().enumerate() {
            if j == 0 || j == forest.len() - 1 {
                // Ignore borders because we'd be multiplying by 0
                continue;
            }

            let mut top = forest[0..i].iter()
                .rev()
                .map(|tree_line| tree_line[j])
                .take_while(|other_tree_height| tree_height > other_tree_height)
                .count();
            // didn't reach edge, count 1 more
            top = if top < i {
                top + 1
            } else {
                top
            };

            let mut right = forest[i][j + 1..forest.len()].iter()
                .take_while(|other_tree_height| tree_height > other_tree_height)
                .count();
            // didn't reach edge, count 1 more
            right = if right < forest.len() - (j + 1) {
                right + 1
            } else {
                right
            };

            let mut bottom = forest[i + 1..forest.len()].iter()
                .map(|tree_line| tree_line[j])
                .take_while(|other_tree_height| tree_height > other_tree_height)
                .count();
            // didn't reach edge, count 1 more
            bottom = if bottom < forest.len() - (i + 1) {
                bottom + 1
            } else {
                bottom
            };

            let mut left = forest[i][0..j].iter()
                .rev()
                .take_while(|other_tree_height| tree_height > other_tree_height)
                .count();
            // didn't reach edge, count 1 more
            left = if left < j {
                left + 1
            } else {
                left
            };

            result = max(result, top * right * bottom * left)
        }
    }

    dbg!(result);
}

#[derive(Clone)]
struct Tree {
    top_vis: i32,
    right_vis: i32,
    bottom_vis: i32,
    left_vis: i32,
    height: u32
}

impl Tree {
    fn new() -> Tree {
        Tree {
            top_vis: 0,
            right_vis: 0,
            bottom_vis: 0,
            left_vis: 0,
            height: 0
        }
    }

    fn is_visible(&self) -> bool {
        self.height as i32 > self.top_vis
            || self.height as i32 > self.right_vis
            || self.height as i32 > self.bottom_vis
            || self.height as i32 > self.left_vis
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?}, {:?})", self.top_vis, self.right_vis, self.bottom_vis, self.left_vis)
    }
}
