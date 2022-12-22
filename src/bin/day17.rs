use std::{cmp::max, collections::HashSet};

fn main() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    println!("Answer 1: {}", solve(&input)); 
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> i32 {
    let mut pattern = input
        .chars()
        .map(|c| if c == '<' { -1i32 } else { 1i32 })
        .cycle();
    let shapes = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut solid: HashSet<(i32, i32)> = Default::default();
    
    let mut highest_stone = -1;

    for i in 0..2022 {
        let new_shape = &shapes[i % 5];

        let mut x = 2;
        let mut y = highest_stone + 4;

        loop {
            let move_x = pattern.next().unwrap();
            x += move_x;

            let move_impossible = x < 0
                || new_shape
                    .iter()
                    .map(|(shape_x, shape_y)| {
                        shape_x + x >= 7 || solid.contains(&(shape_x + x, shape_y + y))
                    })
                    .any(|b| b);

            if move_impossible {
                x -= move_x;
            }

            y -= 1;

            let move_impossible = y < 0
                || new_shape
                    .iter()
                    .map(|(shape_x, shape_y)| solid.contains(&(shape_x + x, shape_y + y)))
                    .any(|b| b);

            if move_impossible {
                y += 1;

                let prev_pos = new_shape
                    .iter()
                    .map(|(shape_x, shape_y)| (shape_x + x, shape_y + y));
                solid.extend(prev_pos);
                break;
            }
        }

        let top_of_current_stone = y + new_shape
            .iter()
            .max_by(|(_x1, y1), (_x2, y2)| y1.cmp(y2))
            .unwrap()
            .1;
        highest_stone = max(highest_stone, top_of_current_stone);

        plot(&solid, highest_stone);
    }
    highest_stone + 1
}

fn solve_2(input: &str) -> u64 {
    let mut pattern = input
        .chars()
        .map(|c| if c == '<' { -1i32 } else { 1i32 })
        .cycle();
    let shapes = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut solid: HashSet<(i32, i32)> = Default::default();
    
    let mut highest_stone = -1;

    for i in 0..2022 {
        let new_shape = &shapes[i % 5];

        let mut x = 2;
        let mut y = highest_stone + 4;

        loop {
            let move_x = pattern.next().unwrap();
            x += move_x;

            let move_impossible = x < 0
                || new_shape
                    .iter()
                    .map(|(shape_x, shape_y)| {
                        shape_x + x >= 7 || solid.contains(&(shape_x + x, shape_y + y))
                    })
                    .any(|b| b);

            if move_impossible {
                x -= move_x;
            }

            y -= 1;

            let move_impossible = y < 0
                || new_shape
                    .iter()
                    .map(|(shape_x, shape_y)| solid.contains(&(shape_x + x, shape_y + y)))
                    .any(|b| b);

            if move_impossible {
                y += 1;

                let prev_pos = new_shape
                    .iter()
                    .map(|(shape_x, shape_y)| (shape_x + x, shape_y + y));
                solid.extend(prev_pos);
                break;
            }
        }

        let top_of_current_stone = y + new_shape
            .iter()
            .max_by(|(_x1, y1), (_x2, y2)| y1.cmp(y2))
            .unwrap()
            .1;
        highest_stone = max(highest_stone, top_of_current_stone);

        // plot(&solid, highest_stone);
    }
    highest_stone + 1;
    todo!()
}

fn plot(solid: &HashSet<(i32, i32)>, max: i32) {
    for y in (0..=max).rev() {
        for x in 0..7 {
            let char = if solid.contains(&(x, y)) { "#" } else { "." };
            print!("{char}");
        }
        println!();
    }
    println!();
}

#[test]
fn test_example() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(solve(input), 3068)
}

#[test]
fn test_example_2() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(solve_2(input), 1514285714288u64)
}
