fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: \n{}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut signal_strength_sum = 0;
    let cycles_to_check = vec![20, 60, 100, 140, 180, 220];
    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            if cycles_to_check.contains(&cycle) {
                signal_strength_sum += dbg!(cycle * x as usize);
            }
        } else {
            cycle += 1;
            if cycles_to_check.contains(&cycle) {
                signal_strength_sum += dbg!(cycle * x as usize);
            }
            cycle += 1;
            if cycles_to_check.contains(&cycle) {
                signal_strength_sum += dbg!(cycle * x as usize);
            }
            x += line
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }
    signal_strength_sum
}

fn solve_2(input: &str) -> String {
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut drawing = String::new();
    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            draw(cycle, x, &mut drawing);
        } else {
            cycle += 1;
            draw(cycle, x, &mut drawing);
            cycle += 1;
            draw(cycle, x, &mut drawing);
            x += line
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }

    drawing
}

fn draw(cycle: i32, x: i32, canvas: &mut String) {
    if ((cycle -1) % 40 - x).abs() <= 1 {
        canvas.push('#');
    } else {
        canvas.push('.');
    }
    if cycle % 40 == 0 {
        canvas.push('\n');
    }
}

#[test]
fn test_example_2() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    let screen = solve_2(input);
    println!("{screen}");
    assert_eq!(
        screen,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    )
}

#[test]
fn test_example() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(solve(input), 13140)
}
