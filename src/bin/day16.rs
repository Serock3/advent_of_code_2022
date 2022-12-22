use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day??.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    let graph: HashMap<String, (u32, Vec<&str>)> = HashMap::from_iter(input.lines().map(|line| {
        let (node, rem) = &line
            .strip_prefix("Valve ")
            .unwrap()
            .split_once(" has flow rate=")
            .unwrap();
        let (flow_rate, rem) = rem
            .split_once("; tunnels lead to valves ")
            .unwrap_or_else(|| rem.split_once("; tunnel leads to valve ").unwrap());
        let connected_nodes = rem.split(", ").collect_vec();

        (
            node.to_string(),
            (flow_rate.parse::<u32>().unwrap(), connected_nodes),
        )
    }));

    let mut tree: Vec<(Vec<(&str, bool)>, u32)> = vec![(vec![("AA", false)], 0)];

    loop {
        let best_node = tree.iter().max_by(|a,b| a.1.cmp(&b.1)).unwrap();
        
    }

    todo!()
}

#[test]
fn test_example() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    assert_eq!(solve(input), 1651)
}
