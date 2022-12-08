use std::fs;

fn main() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent = chars.next().unwrap() as i32 - 'A' as i32;
        let me = chars.nth(1).unwrap() as i32 - 'X' as i32;
        let score = ((me - opponent + 1).rem_euclid(3)) * 3 + me + 1;
        // let score = match line {
        //     "A X" => 4,
        //     "B Y" => 5,
        //     "C Z" => 6,
        //     "B X" => 1,
        //     "C Y" => 2,
        //     "A Z" => 3,
        //     "C X" => 7,
        //     "A Y" => 8,
        //     "B Z" => 9,
        //     _ => panic!(),
        // };
        // let score = match line {
        //     "A X" => 3,
        //     "A Z" => 8,
        //     "A Y" => 4,
        //     "B X" => 1,
        //     "B Y" => 5,
        //     "B Z" => 9,
        //     "C X" => 2,
        //     "C Y" => 6,
        //     "C Z" => 7,
        //     _ => panic!(),
        // };
        sum += score;
    }
    dbg!(sum);
}

#[test]
fn test() {
    let input = r#"A X
B Y
C Z
B X
C Y
A Z
C X
A Y
B Z"#;
    let answers = vec![4, 5, 6, 1, 2, 3, 7, 8, 9];
    for (line, answer) in input.lines().zip(answers.iter()) {
        let mut chars = line.chars();
        let opponent = chars.next().unwrap() as i32 - 'A' as i32;
        let me = chars.nth(1).unwrap() as i32 - 'X' as i32;
        let score: u32 = (((me - opponent + 1).rem_euclid(3)) * 3 + me + 1) as u32;
        // let score = match line {
        //     "A X" => 4,
        //     "B Y" => 5,
        //     "C Z" => 6,
        //     "B X" => 1,
        //     "C Y" => 2,
        //     "A Z" => 3,
        //     "C X" => 7,
        //     "A Y" => 8,
        //     "B Z" => 9,
        //     _ => panic!(),
        // };
        assert!(score == *answer);
    }
}
