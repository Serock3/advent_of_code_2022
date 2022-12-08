use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day1.txt")?;
    // let lines: Vec<Option<i32>> = input.lines().map(|line| line.parse().ok()).collect();
    // let mut sums: Vec<i32> = lines
    //     .split(|line| line.is_none()) // Split items to groups for each elf
    //     .map(|group| group.iter().map(|item| item.unwrap()).sum()) //  unwrap items in the groups and sum
    //     .collect();

    // Shorter solution using 0 instead of None
    let lines: Vec<i32> = input.lines().map(|line| line.parse().unwrap_or(0)).collect();
    let mut sums: Vec<i32> = lines
        .split(|line| *line == 0) // Split items to groups for each elf
        .map(|group| group.iter().sum()) //  unwrap items in the groups and sum
        .collect();
    sums.sort();
    
    let sum_max: i32 = sums.iter().rev().take(3).sum();
    dbg!(sum_max);
    Ok(())
}
