use std::collections::HashMap;

use itertools::Itertools;

use petgraph::algo::floyd_warshall;
use petgraph::data::FromElements;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::{prelude::*, Graph, Undirected};

fn main() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> u32 {
    let parsed = input
        .lines()
        .map(|line| {
            let (node, rem) = &line
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(" has flow rate=")
                .unwrap();
            let (flow_rate, rem) = rem
                .split_once("; tunnels lead to valves ")
                .unwrap_or_else(|| rem.split_once("; tunnel leads to valve ").unwrap());

            let connected_nodes = rem.split(", ").collect_vec();

            (*node, flow_rate.parse::<u32>().unwrap(), connected_nodes)
        })
        .collect_vec();

    let mut graph: GraphMap<&str, (), Directed> = Default::default();
    let mut flow_rates = HashMap::new();
    for item in parsed.iter() {
        if item.1 != 0{
            flow_rates.insert(item.0, item.1);
        }
        for node_b in item.2.iter() {
            graph.add_edge(item.0, node_b, ());
        }
    }
    let dist = floyd_warshall(&graph, |_| 1).unwrap();
    let idx = flow_rates.iter().enumerate()

    todo!()
}


fn asd(time: u32, valve: &str, open: &[bool]) -> u32 {

    let next = todo!();
    ads(time- dist[next]- 1, next, todo!())
    
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
