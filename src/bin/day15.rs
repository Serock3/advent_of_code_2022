use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();
    // println!("Answer 1: {}", solve(&input, 2000000)); // 4879972
    println!("Answer 2: {}", solve_2(&input, 4000000));
}

fn solve(input: &str, row: i32) -> i32 {
    let parsed = input.lines().map(|line| {
        let (sensor, beacon) = line
            .strip_prefix("Sensor at x=")
            .unwrap()
            .split_once(": closest beacon is at x=")
            .unwrap();
        let (x_sensor, y_sensor) = sensor.split_once(", y=").unwrap();
        let (x_beacon, y_beacon) = beacon.split_once(", y=").unwrap();

        (
            (
                x_sensor.parse::<i32>().unwrap(),
                y_sensor.parse::<i32>().unwrap(),
            ),
            (
                x_beacon.parse::<i32>().unwrap(),
                y_beacon.parse::<i32>().unwrap(),
            ),
        )
    });

    let mut ranges = Vec::new();

    for ((x_sensor, y_sensor), (x_beacon, y_beacon)) in parsed {
        let manhattan_dist = (x_sensor - x_beacon).abs() + (y_sensor - y_beacon).abs();
        let overlap = manhattan_dist - (row - y_sensor).abs();
        if overlap >= 0 {
            let range = ((x_sensor - overlap), (x_sensor + overlap));
            ranges.push(range);
        }
    }

    ranges.sort();
    for i in (1..ranges.len()).rev() {
        for j in 0..i {
            if ranges[i].0 <= ranges[j].1 {
                ranges[j].1 = std::cmp::max(ranges[i].1, ranges[j].1);
                ranges.pop();
                break;
            }
        }
    }

    ranges.iter().map(|(l, h)| h - l).sum::<i32>()
}

fn solve_2(input: &str, window: i32) -> u128 {
    let parsed = input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line
                .strip_prefix("Sensor at x=")
                .unwrap()
                .split_once(": closest beacon is at x=")
                .unwrap();
            let (x_sensor, y_sensor) = sensor.split_once(", y=").unwrap();
            let (x_beacon, y_beacon) = beacon.split_once(", y=").unwrap();

            (
                (
                    x_sensor.parse::<i32>().unwrap(),
                    y_sensor.parse::<i32>().unwrap(),
                ),
                (
                    x_beacon.parse::<i32>().unwrap(),
                    y_beacon.parse::<i32>().unwrap(),
                ),
            )
        })
        .collect_vec();

    for y in 0..window {
        let mut ranges = Vec::new();
        for ((x_sensor, y_sensor), (x_beacon, y_beacon)) in &parsed {
            let manhattan_dist = (x_sensor - x_beacon).abs() + (y_sensor - y_beacon).abs();
            let overlap_row = manhattan_dist - (y - y_sensor).abs();
            let overlap_window = manhattan_dist - ((window/2 - x_sensor).abs() - window/2);
            if overlap_row >= 0 && overlap_window >= 0 {
                let range = ((x_sensor - overlap_row), (x_sensor + overlap_row));
                ranges.push(range);
            }
        }

        ranges.sort();
        for i in (1..ranges.len()).rev() {
            for j in 0..i {
                if ranges[i].0 <= ranges[j].1 {
                    ranges[j].1 = std::cmp::max(ranges[i].1, ranges[j].1);
                    ranges.remove(i);
                    break;
                }
            }
        }
        ranges[0].0 = max(ranges[0].0, 0);
        ranges.last_mut().unwrap().1 = min(ranges.last_mut().unwrap().1, window);

        if y % (window /10) == 0 {
            dbg!(y);
        }
        let occupied = ranges.iter().map(|(l, h)| h - l).sum::<i32>();
        if occupied < window {
            let x: i32 = ranges[0].1 + 1;
            return x as u128 * 4000000 + y as u128;
        }
    }
    panic!()
}

#[test]
fn test_example2() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    assert_eq!(solve_2(input, 20), 56000011)
}

#[test]
fn test_example() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    assert_eq!(solve(input, 10), 26)
}
