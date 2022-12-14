use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

fn solve_2(input: &str) -> usize {
    let mut map = HashSet::new();
    for row in input.lines().map(|line| {
        line.split(" -> ")
            .map(|coord| {
                let (x, y) = coord.split_once(',').unwrap();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .tuple_windows()
            .flat_map(|(prev, new)| {
                if prev.0 != new.0 {
                    assert_eq!(prev.1, new.1);
                    if prev.0 > new.0 {
                        (new.0..=prev.0).map(|line_x| (line_x, new.1)).collect_vec()
                    } else {
                        (prev.0..=new.0).map(|line_x| (line_x, new.1)).collect_vec()
                    }
                } else {
                    assert_eq!(prev.0, new.0);
                    if prev.1 < new.1 {
                        (prev.1..new.1).map(|line_y| (new.0, line_y)).collect_vec()
                    } else {
                        (new.1..=prev.1).map(|line_y| (new.0, line_y)).collect_vec()
                    }
                }
            })
    }) {
        map.extend(row)
    }
    let floor = *map.iter().map(|(_x, y)| y).max().unwrap() + 2;

    for count in 0.. {
        let mut sand = (500, 0);
        if map.contains(&sand) {
            return count;
        }
        loop {
            sand.1 += 1;

            if sand.1 >= floor {
                sand.1 -= 1;
                assert!(map.insert(sand));
                break;
            }

            if map.contains(&sand) {
                sand.0 -= 1;
                if map.contains(&sand) {
                    sand.0 += 2;
                    if map.contains(&sand) {
                        sand.0 -= 1;
                        sand.1 -= 1;
                        assert!(map.insert(sand));
                        break;
                    }
                }
            }
        }
    }
    panic!()
}

fn solve(input: &str) -> usize {
    let mut map = HashSet::new();
    for row in input.lines().map(|line| {
        line.split(" -> ")
            .map(|coord| {
                let (x, y) = coord.split_once(',').unwrap();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .tuple_windows()
            .flat_map(|(prev, new)| {
                if prev.0 != new.0 {
                    assert_eq!(prev.1, new.1);
                    if prev.0 > new.0 {
                        (new.0..=prev.0).map(|line_x| (line_x, new.1)).collect_vec()
                    } else {
                        (prev.0..=new.0).map(|line_x| (line_x, new.1)).collect_vec()
                    }
                } else {
                    assert_eq!(prev.0, new.0);
                    if prev.1 < new.1 {
                        (prev.1..new.1).map(|line_y| (new.0, line_y)).collect_vec()
                    } else {
                        (new.1..=prev.1).map(|line_y| (new.0, line_y)).collect_vec()
                    }
                }
            })
    }) {
        map.extend(row)
    }

    for count in 0.. {
        let mut sand = (500, 0);
        loop {
            sand.1 += 1;
            if map.contains(&sand) {
                sand.0 -= 1;
                if map.contains(&sand) {
                    sand.0 += 2;
                    if map.contains(&sand) {
                        sand.0 -= 1;
                        sand.1 -= 1;
                        assert!(map.insert(sand));
                        break;
                    }
                }
            }

            if sand.1 > 200 {
                return count;
            }
        }
    }
    panic!()
}

#[test]
fn test_example() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(solve(input), 24)
}

#[test]
fn test_example_2() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(solve_2(input), 93)
}
