use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    // println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

type T = u64;

enum OpType {
    Add(T),
    Mul(T),
    Sqr,
}
struct Monkey {
    items: Vec<T>,
    op: OpType,
    divisible: T,
    throw_monkeys_true: usize,
    throw_monkeys_false: usize,
    inspected_items: usize,
}

impl Monkey {
    fn action(&mut self) -> Vec<(usize, T)> {
        std::mem::take(&mut self.items)
            .into_iter()
            .map(|mut item| {
                self.inspected_items += 1;
                item = match self.op {
                    OpType::Add(x) => item + x,
                    OpType::Mul(x) => item * x,
                    OpType::Sqr => item * item,
                };
                item /= 3;
                if item % self.divisible == 0 {
                    (self.throw_monkeys_true, item)
                } else {
                    (self.throw_monkeys_false, item)
                }
            })
            .collect_vec()
    }

    fn action_2(&mut self, prime_prod: u64) -> Vec<(usize, T)> {
        std::mem::take(&mut self.items)
            .into_iter()
            .map(|mut item| {
                self.inspected_items += 1;
                item = match self.op {
                    OpType::Add(x) => item + x,
                    OpType::Mul(x) => item * x,
                    OpType::Sqr => item * item,
                };
                // item = (item/3.0).floor();
                item %= prime_prod;
                if item % self.divisible == 0 {
                    (self.throw_monkeys_true, item)
                } else {
                    (self.throw_monkeys_false, item)
                }
            })
            .collect_vec()
    }
}

impl Monkey {
    fn new(input: &str) -> Self {
        let mut lines = input.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|val| val.parse().unwrap())
            .collect_vec();
        let op_str = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap();
        let op = match op_str.split_ascii_whitespace().collect_tuple().unwrap() {
            ("*", "old") => OpType::Sqr,
            ("*", val) => OpType::Mul(val.parse().unwrap()),
            ("+", val) => OpType::Add(val.parse().unwrap()),
            _ => panic!(),
        };
        let divisible = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let throw_monkeys_true = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let throw_monkeys_false = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        Self {
            items,
            op,
            divisible,
            throw_monkeys_true,
            throw_monkeys_false,
            inspected_items: 0,
        }
    }
}

fn solve(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();
    // dbg!(monkeys);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items_to_throw = monkeys[i].action();
            for (j, item) in items_to_throw.into_iter() {
                monkeys[j].items.push(item);
            }
        }
    }
    let mut num_inspected = monkeys.iter().map(|m| m.inspected_items).collect_vec();
    dbg!(&num_inspected);
    num_inspected.sort();
    num_inspected.iter().rev().take(2).product()
}

fn solve_2(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::new).collect();
    // dbg!(monkeys);
    let prime_prod: u64 = monkeys.iter().map(|m| m.divisible).product();

    for i in 0..10000 {
        for i in 0..monkeys.len() {
            let items_to_throw = monkeys[i].action_2(prime_prod);
            for (j, item) in items_to_throw.into_iter() {
                monkeys[j].items.push(item);
            }
        }
        if i == 20 || i % 1000 == 0 {
            let _ = 1;
        }
    }
    let mut num_inspected = monkeys.iter().map(|m| m.inspected_items).collect_vec();
    dbg!(&num_inspected);
    num_inspected.sort();
    num_inspected.iter().rev().take(2).product()
}

#[test]
fn test_example() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    assert_eq!(solve(input), 10605)
}

#[test]
fn test_example_2() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    assert_eq!(solve_2(input), 2713310158)
}
