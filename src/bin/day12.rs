use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' => 0,
                    'E' => 25,
                    other => other as u8 - b'a',
                })
                .collect_vec()
        })
        .collect_vec();

    let start = input.find('S').unwrap();
    let line_len = input.lines().next().unwrap().len() + 1;
    let start = (start / line_len, start % line_len);
    let goal = input.find('E').unwrap();
    let goal = (goal / line_len, goal % line_len);

    dumb_pathfinder(&heights, start, goal).unwrap()
}

fn solve_2(input: &str) -> usize {
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' => 0,
                    'E' => 25,
                    other => other as u8 - b'a',
                })
                .collect_vec()
        })
        .collect_vec();

    input
        .chars()
        .enumerate()
        .filter(|(_, r)| *r == 'S' || *r == 'a')
        .map(|(index, _)| index)
        .filter_map(|start| {
            let line_len = input.lines().next().unwrap().len() + 1;
            let start = (start / line_len, start % line_len);
            let goal = input.find('E').unwrap();
            let goal = (goal / line_len, goal % line_len);
            dumb_pathfinder(&heights, start, goal)
        })
        .min()
        .unwrap()
    // let start = input.find('E').unwrap();
    // let line_len = input.lines().next().unwrap().len() + 1;
    // let start = (start / line_len, start % line_len);

    // dumb_pathfinder_2(&heights, start).unwrap()
}

fn dumb_pathfinder_2(heights: &Vec<Vec<u8>>, start: (usize, usize)) -> Option<usize> {
    let mut frontier: Vec<((usize, usize), usize)> = vec![(start, 0)];
    let mut visited: Vec<((usize, usize), usize)> = vec![(start, 0)];
    loop {
        if let Some(path) = frontier
            .iter()
            .find(|node| heights[node.0 .0][node.0 .1] == 0)
        {
            dbg!(&path);
            return Some(path.1);
        }
        frontier.sort_by(|a, b| b.1.cmp(&a.1)); // Sort backward so last elem is smallest
        let Some(expand_from) = frontier.pop() else{
            return None
        };
        frontier.append(&mut get_reachable_neighbors_reverse(
            heights,
            expand_from,
            &mut visited,
        ));
    }
}

fn dumb_pathfinder(
    heights: &Vec<Vec<u8>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    let mut frontier: Vec<((usize, usize), usize)> = vec![(start, 0)];
    let mut visited: Vec<((usize, usize), usize)> = vec![(start, 0)];
    loop {
        if let Some(path) = frontier.iter().find(|node| node.0 == goal) {
            dbg!(&path);
            return Some(path.1);
        }
        frontier.sort_by(|a, b| b.1.cmp(&a.1)); // Sort backward so last elem is smallest
        let Some(expand_from) = frontier.pop() else{
            return None
        };
        frontier.append(&mut get_reachable_neighbors(
            heights,
            expand_from,
            &mut visited,
        ));
    }
}

fn get_reachable_neighbors_reverse(
    heights: &Vec<Vec<u8>>,
    current: ((usize, usize), usize),
    visited: &mut Vec<((usize, usize), usize)>,
) -> Vec<((usize, usize), usize)> {
    let mut reachable = vec![];

    let pos = current.0;
    let dist = current.1;

    // left
    if pos.0 > 0 {
        let new = (pos.0 - 1, pos.1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] + 1 >= heights[pos.0][pos.1]
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }
    // right
    if pos.0 < heights.len() - 1 {
        let new = (pos.0 + 1, pos.1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] + 1 >= heights[pos.0][pos.1]
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }
    // up
    if pos.1 > 0 {
        let new = (pos.0, pos.1 - 1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] + 1 >= heights[pos.0][pos.1]
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }
    // down
    if pos.1 < heights[0].len() - 1 {
        let new = (pos.0, pos.1 + 1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] + 1 >= heights[pos.0][pos.1]
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }

    reachable
}
fn get_reachable_neighbors(
    heights: &Vec<Vec<u8>>,
    current: ((usize, usize), usize),
    visited: &mut Vec<((usize, usize), usize)>,
) -> Vec<((usize, usize), usize)> {
    let mut reachable = vec![];

    let pos = current.0;
    let dist = current.1;

    // left
    if pos.0 > 0 {
        let new = (pos.0 - 1, pos.1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] <= heights[pos.0][pos.1] + 1
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }
    // right
    if pos.0 < heights.len() - 1 {
        let new = (pos.0 + 1, pos.1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] <= heights[pos.0][pos.1] + 1
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }
    // up
    if pos.1 > 0 {
        let new = (pos.0, pos.1 - 1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] <= heights[pos.0][pos.1] + 1
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }
    // down
    if pos.1 < heights[0].len() - 1 {
        let new = (pos.0, pos.1 + 1);
        // If we haven't visited the new location and it is no more than one step higher than we are, add it
        if !visited.iter().any(|node| node.0 == new)
            && heights[new.0][new.1] <= heights[pos.0][pos.1] + 1
        {
            visited.push((new, dist + 1));
            reachable.push((new, dist + 1));
        }
    }

    reachable
}

#[test]
fn test_example() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(solve(input), 31)
}
#[test]
fn test_example_2() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(solve_2(input), 29)
}
