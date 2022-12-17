use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input)); // 22650 low
}

fn solve(input: &str) -> usize {
    let order = input.split("\n\n").map(|two_lines| {
        let (first, second) = two_lines.split_once('\n').unwrap();
        NumOrList::from(first) < NumOrList::from(second)
    });

    order
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i + 1) } else { None })
        .sum()
}

fn solve_2(input: &str) -> usize {
    let mut collected = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(NumOrList::from)
        .collect_vec();
    collected.append(&mut vec![NumOrList::from("[[2]]"), NumOrList::from("[[6]]")]);

    collected.sort();
    let first = collected
        .iter()
        .find_position(|item| {
            **item == NumOrList::List(vec![NumOrList::List(vec![NumOrList::Num(2)])])
        })
        .unwrap();
    let second = collected
        .iter()
        .find_position(|item| {
            **item == NumOrList::List(vec![NumOrList::List(vec![NumOrList::Num(6)])])
        })
        .unwrap();
    (first.0 + 1) * (second.0 + 1)
}

#[derive(PartialEq, Eq, Debug)]
enum NumOrList {
    Num(u8),
    List(Vec<NumOrList>),
}

impl Ord for NumOrList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<u8> for NumOrList {
    fn from(num: u8) -> Self {
        NumOrList::List(vec![NumOrList::Num(num)])
    }
}

impl From<&str> for NumOrList {
    fn from(input: &str) -> Self {
        if let Some(stripped) = input
            .strip_prefix('[')
            .map(|inner| inner.strip_suffix(']').unwrap())
        {
            let mut slices = vec![];

            let mut depth = 0;
            let mut start = 0;

            while let Some(offset) = stripped[start..].find(|c| match c {
                ']' => {
                    depth -= 1;
                    false
                }
                '[' => {
                    depth += 1;
                    false
                }
                ',' => depth == 0,
                _ => false,
            }) {
                slices.push(&stripped[start..(start + offset)]);
                start += offset + 1;
            }
            slices.push(&stripped[start..]);

            NumOrList::List(slices.iter().map(|s| NumOrList::from(*s)).collect_vec())
        } else if input.is_empty() {
            NumOrList::List(vec![])
        } else {
            NumOrList::Num(input.parse::<u8>().unwrap())
        }
    }
}

impl std::cmp::PartialOrd for NumOrList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (NumOrList::Num(f), NumOrList::Num(s)) => Some(f.cmp(s)),
            (NumOrList::Num(f), NumOrList::List(_s)) => NumOrList::from(*f).partial_cmp(other),
            (NumOrList::List(_f), NumOrList::Num(s)) => self.partial_cmp(&NumOrList::from(*s)),
            (NumOrList::List(f), NumOrList::List(s)) => {
                for zipped in f.iter().zip_longest(s.iter()) {
                    match zipped {
                        itertools::EitherOrBoth::Both(f, s) => match f.partial_cmp(s).unwrap() {
                            std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => {
                                return Some(std::cmp::Ordering::Greater)
                            }
                        },
                        itertools::EitherOrBoth::Left(_) => {
                            return Some(std::cmp::Ordering::Greater)
                        }
                        itertools::EitherOrBoth::Right(_) => return Some(std::cmp::Ordering::Less),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

#[test]
fn test_example() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(solve(input), 13)
}

#[test]
fn test_example_2() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(solve_2(input), 140)
}
