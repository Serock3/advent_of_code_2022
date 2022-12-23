use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();
    println!("Answer: {}", solve(&input));
    println!("Answer: {}", solve_2(&input));
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let ((first_first, first_second), (second_first, second_second)) = line
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            (((first_first..=first_second).contains(&second_first)
                && (first_first..=first_second).contains(&second_second))
                || ((second_first..=second_second).contains(&first_first)
                    && (second_first..=second_second).contains(&first_second))) as u32
        })
        .sum()
}

fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let ((first_first, first_second), (second_first, second_second)) = line
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            (((first_first..=first_second).contains(&second_first)
                || (first_first..=first_second).contains(&second_second))
                || ((second_first..=second_second).contains(&first_first)
                    || (second_first..=second_second).contains(&first_second))) as u32
        })
        .sum()
}

#[test]
fn test_example() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(solve(input), 2)
}

#[test]
fn test_example_2() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(solve_2(input), 4)
}

// 286
// 494
// 698
