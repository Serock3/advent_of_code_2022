use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    println!("Answer: {}", solve_2(&input));
}

fn solve(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let len = line.len();
        assert!(len % 2 == 0);
        let (first, last) = line.split_at(len / 2);
        let common_items = first.chars().unique().filter(|item| last.contains(*item));
        let value_for_line: u8 = common_items.map(priority).sum();
        dbg!(value_for_line);
        sum += value_for_line as i32;
        // dbg!(common);
        // for item in first.chars() {
        //     last.contains(item)
        // }
    }
    sum
}

fn solve_2(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for three_lines in &input.lines().chunks(3) {
        let (first, second, third) = three_lines.collect_tuple().unwrap();
        let common_items = first.chars().unique().filter(|item| second.contains(*item) && third.contains(*item));
        let value_for_line: u8 = common_items.map(priority).sum();
        dbg!(value_for_line);
        sum += value_for_line as i32;
        // dbg!(common);
        // for item in first.chars() {
        //     last.contains(item)
        // }
    }
    sum
}

fn priority(item: char) -> u8 {
    // dbg!(item);
    if item.is_lowercase() {
        item as u8 - b'a' + 1
    } else {
        item as u8 - b'A' + 27
    }
}

#[test]
fn test_priority() {
    let items = "pLPvts".chars();
    let values = vec![16, 38, 42, 22, 20, 19];
    for (item, value) in items.zip(values) {
        assert_eq!(priority(item), value, "Letter was: {item}");
    }
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        ),
        157
    )
}

#[test]
fn test_example2() {
    assert_eq!(
        solve_2(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        ),
        70
    )
}
