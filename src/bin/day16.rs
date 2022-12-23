use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::{RwLock};

use itertools::Itertools;

use petgraph::algo::floyd_warshall;

use petgraph::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

/// Struct to gather all variables that need to be 'globally' readable
struct Cave<'a> {
    flow_rate: HashMap<&'a str, i32>,
    distances: HashMap<(&'a str, &'a str), i32>,
    valid_room_enum: HashMap<&'a str, usize>,
    // This didn't seem to help as much as I would have thought
    cache: RwLock<HashMap<(&'a str, u16, i32), i32>>, 
}

impl<'a> Cave<'a> {
    fn new(input: &str) -> Cave {
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

                (*node, flow_rate.parse::<i32>().unwrap(), connected_nodes)
            })
            .collect_vec();
        let mut graph: GraphMap<&str, (), Directed> = Default::default();
        let mut flow_rates = HashMap::new();
        for item in parsed.iter() {
            if item.1 != 0 {
                flow_rates.insert(item.0, item.1);
            }
            for node_b in item.2.iter() {
                graph.add_edge(item.0, node_b, ());
            }
        }
        let dist = floyd_warshall(&graph, |_| 1).unwrap();
        let non_zero: HashMap<&str, usize, RandomState> =
            HashMap::from_iter(flow_rates.iter().enumerate().map(|(i, (k, _v))| (*k, i)));
        Cave {
            flow_rate: flow_rates,
            distances: dist,
            valid_room_enum: non_zero,
            cache: Default::default(),
        }
    }

    /// opened_valves uses a stack allocated bit array, which has some margin, but is probably still better than heap allocating
    fn traverse_tunnel(&self, current_valve: &'a str, opened_valves: u16, time: i32) -> i32 {
        if let Some(cached_res) =
            self.cache
                .read()
                .unwrap()
                .get(&(current_valve, opened_valves, time))
        {
            return *cached_res;
        }
        let res = self.valid_room_enum
            .iter()
            .filter(|(next_valve, idx)| {
                **next_valve != current_valve && (opened_valves & 1 << **idx) == 0
            })
            .map(|(next_valve, idx)| -> i32 {
                let time_rem = time - self.distances[&(current_valve, *next_valve)] - 1;
                if time_rem <= 0 {
                    return 0;
                }
                let opened_next = opened_valves | 1 << *idx;
                // println!("opened_valves: {opened_valves:b}, opened_next: {opened_next:b}, idx: {idx}");
                time_rem * self.flow_rate[next_valve]
                    + self.traverse_tunnel(next_valve, opened_next, time_rem)
            })
            .max()
            .unwrap_or(0);
        self.cache.write().unwrap().insert((current_valve, opened_valves, time), res);
        res
    }
}

fn solve(input: &str) -> i32 {
    let cave = Cave::new(input);
    let time = 30;
    let current_valve = "AA";
    let opened_valves = 0u16;
    cave.traverse_tunnel(current_valve, opened_valves, time)
}

fn solve_2(input: &str) -> i32 {
    let cave = Cave::new(input);
    let time = 26;
    let current_valve = "AA";

    (0..1 << cave.valid_room_enum.len())
        .map(|your_valves_bitfield| {
            let elephant_valves_bitfield = !your_valves_bitfield;
            // println!("{elephant_valves_bitfield:b}");
            cave.traverse_tunnel(current_valve, your_valves_bitfield, time)
                + cave.traverse_tunnel(current_valve, elephant_valves_bitfield, time)
        })
        .max()
        .unwrap()
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

#[test]
fn test_example_2() {
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
    assert_eq!(solve_2(input), 1707)
}
