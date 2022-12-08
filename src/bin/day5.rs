use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day5.txt").unwrap();

    // let mut stacks: Vec<Vec<char>> = input
    //     .lines()
    //     .take_while(|line| !line.starts_with(" 1 "))
    //     .collect::<Vec<&str>>()
    //     .iter()
    //     .rev()
    //     .map(|line| line.chars().skip(1).step_by(4).collect())
    //     .collect();

    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let start_conf = input.lines().take_while(|line| !line.starts_with(" 1 "));
    for line in start_conf.collect::<Vec<&str>>().iter().rev() {
        let letters = line.chars().skip(1).step_by(4);
        // dbg!(letters.collect::<Vec<char>>());
        for (i, chr) in letters.enumerate() {
            if chr != ' ' {
                stacks[i].push(chr);
            }
        }
    }

    for (amount, from, to) in input
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
    {
        let range = (stacks[from - 1].len() - amount)..;
        // let mut moving_crates = stacks[from - 1].drain(range).rev().collect();
        let mut moving_crates = stacks[from - 1].drain(range).collect();
        stacks[to - 1].append(&mut moving_crates);
    }
    for stack in stacks.iter(){
        print!("{}",stack.last().unwrap());
    }
    // println!("{stacks:?}");
    // dbg!(stacks);
}

// const test_input: &str = r#"
// [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
// "#;

// const test_answer: &str = r#"CMZ"#;
