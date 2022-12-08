use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("Answer: {}", solve_2(&input));
}

fn solve(input: &str) -> i32 {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().collect::<Vec<&str>>().len();

    let height = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let mut visible = (0..rows)
        .map(|_| (0..cols).map(|_| 0).collect_vec())
        .collect_vec();

    for (row, vis_row) in height.iter().zip(visible.iter_mut()) {
        let mut max = -1;
        for (height, vis) in row.iter().zip(vis_row.iter_mut()) {
            if *height > max {
                max = *height;
                *vis = 1;
            }
        }
        let mut max = -1;
        for (height, vis) in row.iter().zip(vis_row.iter_mut()).rev() {
            if *height > max {
                max = *height;
                *vis = 1;
            }
        }
    }

    for col in 0..cols {
        let mut max = -1;
        for (height, vis) in height
            .iter()
            .map(|row| row.get(col).unwrap())
            .zip(visible.iter_mut().map(|row| row.get_mut(col).unwrap()))
        {
            if *height > max {
                max = *height;
                *vis = 1;
            }
        }
        let mut max = -1;
        for (height, vis) in height
            .iter()
            .map(|row| row.get(col).unwrap())
            .zip(visible.iter_mut().map(|row| row.get_mut(col).unwrap()))
            .rev()
        {
            if *height > max {
                max = *height;
                *vis = 1;
            }
        }
    }
    visible.iter().map(|row| row.iter().sum::<i32>()).sum()
}

fn solve_2(input: &str) -> usize {
    let num_cols = input.lines().next().unwrap().len();
    let num_rows = input.lines().count();

    let height = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let mut max_scenic_score = 0;
    for i_row in 1..num_rows - 1 {
        for i_col in 1..num_cols - 1 {
            let row = &height[i_row];
            let col = height.iter().map(|row| row[i_col]).collect_vec();

            let vis_to_right = distance_to_tree_of_same_height(row.iter().skip(i_col));

            let vis_to_left =
                distance_to_tree_of_same_height(row.iter().rev().skip(num_cols - i_col - 1));

            let vis_down = distance_to_tree_of_same_height(col.iter().skip(i_row));

            let vis_up = distance_to_tree_of_same_height(col.iter().rev().skip(num_rows - i_row - 1));

            let scenic_score = vis_to_right * vis_to_left * vis_up * vis_down;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }

    max_scenic_score
}

fn distance_to_tree_of_same_height<'a>(mut view: impl Iterator<Item = &'a i32>) -> usize {
    let max_height = view.next().unwrap();
    if let Some((i, _)) = view
        .enumerate()
        .find_or_last(|(_, look_height)| *look_height >= max_height) {
        i + 1
    } else {
        0
    }
}

#[test]
fn test_example() {
    let input = r#"30373
25512
65332
33549
35390"#;
    assert_eq!(solve(input), 21)
}

#[test]
fn test_example_2() {
    let input = r#"30373
25512
65332
33549
35390"#;
    assert_eq!(solve_2(input), 8)
}
