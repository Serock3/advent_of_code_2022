use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut tail_visited: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let steps = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        // .chars().nth(2).unwrap().to_digit(10).unwrap();
        for _rep in 0..steps {
            match dir {
                'R' => head_pos.0 += 1,
                'L' => head_pos.0 -= 1,
                'U' => head_pos.1 += 1,
                'D' => head_pos.1 -= 1,
                _ => panic!(),
            }

            let del_x = head_pos.0 - tail_pos.0;
            let del_y = head_pos.1 - tail_pos.1;
            match (del_x, del_y) {
                (2, 0) => tail_pos.0 += 1,
                (-2, 0) => tail_pos.0 -= 1,
                (0, 2) => tail_pos.1 += 1,
                (0, -2) => tail_pos.1 -= 1,
                (2, 1) => {
                    tail_pos.0 += 1;
                    tail_pos.1 += 1
                }
                (1, 2) => {
                    tail_pos.0 += 1;
                    tail_pos.1 += 1
                }
                (2, 2) => {
                    tail_pos.0 += 1;
                    tail_pos.1 += 1
                }
                (2, -1) => {
                    tail_pos.0 += 1;
                    tail_pos.1 -= 1
                }
                (1, -2) => {
                    tail_pos.0 += 1;
                    tail_pos.1 -= 1
                }
                (2, -2) => {
                    tail_pos.0 += 1;
                    tail_pos.1 -= 1
                }
                (-2, 1) => {
                    tail_pos.0 -= 1;
                    tail_pos.1 += 1
                }
                (-1, 2) => {
                    tail_pos.0 -= 1;
                    tail_pos.1 += 1
                }
                (-2, 2) => {
                    tail_pos.0 -= 1;
                    tail_pos.1 += 1
                }
                (-2, -1) => {
                    tail_pos.0 -= 1;
                    tail_pos.1 -= 1
                }
                (-1, -2) => {
                    tail_pos.0 -= 1;
                    tail_pos.1 -= 1
                }
                (-2, -2) => {
                    tail_pos.0 -= 1;
                    tail_pos.1 -= 1
                }
                (_x, _y) => {} // {
                               //     assert!(x.abs() <= 1);
                               //     assert!(y.abs() <= 1)
                               // }
            }
            tail_visited.push(tail_pos);
            assert!(
                ((tail_pos.1 - head_pos.1) * (tail_pos.0 - head_pos.0)) <= 1,
                "at step {_rep} of {line:?}"
            );
        }
    }
    tail_visited.iter().unique().count()
}
fn solve_2(input: &str) -> usize {
    // let mut head_pos = (0, 0);
    let mut tail_list = vec![(0i32, 0i32); 10];

    let mut tail_visited: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let steps = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        // .chars().nth(2).unwrap().to_digit(10).unwrap();
        for _rep in 0..steps {
            match dir {
                'R' => tail_list[0].0 += 1,
                'L' => tail_list[0].0 -= 1,
                'U' => tail_list[0].1 += 1,
                'D' => tail_list[0].1 -= 1,
                _ => panic!(),
            }

            for i in 0..9 {
                let head_pos = tail_list[i];
                let mut tail_pos = tail_list.get_mut(i + 1).unwrap();
                let del_x = head_pos.0 - tail_pos.0;
                let del_y = head_pos.1 - tail_pos.1;
                match (del_x, del_y) {
                    (2, 0) => tail_pos.0 += 1,
                    (-2, 0) => tail_pos.0 -= 1,
                    (0, 2) => tail_pos.1 += 1,
                    (0, -2) => tail_pos.1 -= 1,
                    (2, 1) => {
                        tail_pos.0 += 1;
                        tail_pos.1 += 1
                    }
                    (1, 2) => {
                        tail_pos.0 += 1;
                        tail_pos.1 += 1
                    }
                    (2, 2) => {
                        tail_pos.0 += 1;
                        tail_pos.1 += 1
                    }
                    (2, -1) => {
                        tail_pos.0 += 1;
                        tail_pos.1 -= 1
                    }
                    (1, -2) => {
                        tail_pos.0 += 1;
                        tail_pos.1 -= 1
                    }
                    (2, -2) => {
                        tail_pos.0 += 1;
                        tail_pos.1 -= 1
                    }
                    (-2, 1) => {
                        tail_pos.0 -= 1;
                        tail_pos.1 += 1
                    }
                    (-1, 2) => {
                        tail_pos.0 -= 1;
                        tail_pos.1 += 1
                    }
                    (-2, 2) => {
                        tail_pos.0 -= 1;
                        tail_pos.1 += 1
                    }
                    (-2, -1) => {
                        tail_pos.0 -= 1;
                        tail_pos.1 -= 1
                    }
                    (-1, -2) => {
                        tail_pos.0 -= 1;
                        tail_pos.1 -= 1
                    }
                    (-2, -2) => {
                        tail_pos.0 -= 1;
                        tail_pos.1 -= 1
                    }
                    (_x, _y) => {} // {
                                   //     assert!(x.abs() <= 1);
                                   //     assert!(y.abs() <= 1)
                                   // }
                }

                assert!(
                    ((tail_pos.1 - head_pos.1) * (tail_pos.0 - head_pos.0)) <= 1,
                    "at step {_rep} of {line:?}"
                );
            }
            tail_visited.push(tail_list.last().unwrap().to_owned());
        }
    }
    tail_visited.iter().unique().count()
}

#[test]
fn test_example() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    assert_eq!(solve(input), 13)
}

#[test]
fn test_example_2() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    assert_eq!(solve_2(input), 1)
}

#[test]
fn test_example_2_large() {
    let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    assert_eq!(solve_2(input), 36)
}