use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    println!("Answer: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(4)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + 4
}

fn solve_2(input: &str) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(14)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + 14
}

#[test]
fn test_example() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(solve(input), 5)
}
#[test]
fn test_example2() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(solve(input), 6)
}
#[test]
fn test_example3() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(solve(input), 10)
}
#[test]
fn test_example4() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(solve(input), 11)
}

#[test]
fn test_example5() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(solve_2(input), 19)
}

#[test]
fn test_example6() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(solve_2(input), 23)
}

#[test]
fn test_example7() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(solve_2(input), 23)
}
