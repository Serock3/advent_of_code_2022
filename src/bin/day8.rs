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
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().collect::<Vec<&str>>().len();

    let hight = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let mut max_scenic_score = 0;
    for i_row in 1..rows - 1 {
        for i_col in 1..cols - 1 {
            let current_height = hight[i_row][i_col];
            let row = &hight[i_row];
            let col = hight.iter().map(|row| row[i_col]).collect_vec();

            let mut delta = 0;
            let vis_to_right = loop {
                if i_col + delta == cols - 1 {
                    break delta;
                }
                delta += 1;
                if row[i_col + delta] >= current_height {
                    break delta;
                }
            };

            let mut delta = 0;
            let vis_to_left = loop {
                if i_col - delta == 0 {
                    break delta;
                }
                delta += 1;
                if row[i_col - delta] >= current_height {
                    break delta;
                }
            };
            let mut delta = 0;
            let vis_down = loop {
                if i_row + delta == rows - 1 {
                    break delta;
                }
                delta += 1;
                if col[i_row + delta] >= current_height {
                    break delta;
                }
            };

            let mut delta = 0;
            let vis_up = loop {
                delta += 1;
                if i_row - delta == 0 {
                    break delta;
                }
                if col[i_row - delta] >= current_height {
                    break delta;
                }
            };

            let scenic_score = vis_to_right * vis_to_left * vis_up * vis_down;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }

    max_scenic_score
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
