#![allow(unused)]

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day18.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

/// For simplicity, we hardcode the size of the grid with one voxel of margin to make
/// bounds checking easier. This takes up more memory but is perhaps faster.
/// 
/// It also guarantees that the steam can reach around the lava for task 2.
const MAX_COORD: usize = 24;

type VoxelList = Vec<(usize, usize, usize)>;

type VoxelMap = [[[usize; MAX_COORD]; MAX_COORD]; MAX_COORD];

/// To support both fast iteration over voxels and fast checking of existence
/// of voxels, we keep both a list of coords and a 3d matrix of booleans
fn parse(input: &str) -> (VoxelList, VoxelMap) {
    let voxel_list = input
        .lines()
        .map(|line| {
            let (x, y, z) = line.split(',').collect_tuple().unwrap();
            (
                x.parse::<usize>().unwrap() + 1,
                y.parse::<usize>().unwrap() + 1,
                z.parse::<usize>().unwrap() + 1,
            )
        })
        .collect_vec();
    let mut voxel_map = [[[0; MAX_COORD]; MAX_COORD]; MAX_COORD];
    for &(x, y, z) in &voxel_list {
        voxel_map[x][y][z] = 1usize;
    }
    (voxel_list, voxel_map)
}

/// Count every touching side among the laval voxels.
/// Remove parsed voxels to avoid double counting.
fn solve(input: &str) -> usize {
    let (voxel_list, mut voxel_map) = parse(input);
    let touching_sides: usize = voxel_list
        .iter()
        .map(|&(x, y, z)| {
            voxel_map[x][y][z] = 0;

            voxel_map[x + 1][y][z]
                + voxel_map[x - 1][y][z]
                + voxel_map[x][y + 1][z]
                + voxel_map[x][y - 1][z]
                + voxel_map[x][y][z + 1]
                + voxel_map[x][y][z - 1]
        })
        .sum();
    voxel_list.len() * 6 - touching_sides * 2
}

fn solve_2(input: &str) -> usize {
    let (parsed, mut voxels) = parse(input);

    // Simulate the spread of steam, starting from 0,0,0
    let mut steam_frontier: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];
    let mut outside_voxel_map = [[[0; MAX_COORD]; MAX_COORD]; MAX_COORD];
    outside_voxel_map[0][0][0] = 1;

    loop {
        let Some((x,y,z)) = steam_frontier.pop() else{
            break;
        };
        outside_voxel_map[x][y][z] = 1;

        // Expand in all cardinal directions
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        {
            // Wrap around to avoid going outside edges
            let x_new = (x as i32 + dx).rem_euclid(MAX_COORD as i32) as usize;
            let y_new = (y as i32 + dy).rem_euclid(MAX_COORD as i32) as usize;
            let z_new = (z as i32 + dz).rem_euclid(MAX_COORD as i32) as usize;
            if outside_voxel_map[x_new][y_new][z_new] == 0 && voxels[x_new][y_new][z_new] == 0 {
                steam_frontier.push((x_new, y_new, z_new))
            };
        }
    }

    calc_all_surface_area_2(parsed, outside_voxel_map)
}


/// Calc surface area by comparing list of inside voxels to map of outside voxels (steam). 
/// This is done by directly counting every neighbor on the outside (steam voxels) for the inner
/// voxels (lava).
fn calc_all_surface_area_2(coords: VoxelList, mut outside_map: VoxelMap) -> usize {
    let touching_sides: usize = coords
        .iter()
        .map(|&(x, y, z)| {

            outside_map[x + 1][y][z]
                + outside_map[x - 1][y][z]
                + outside_map[x][y + 1][z]
                + outside_map[x][y - 1][z]
                + outside_map[x][y][z + 1]
                + outside_map[x][y][z - 1]
        })
        .sum();
    touching_sides
}

fn expand_frontier_wrapping(
    x: i32,
    y: i32,
    z: i32,
    outside_voxels: VoxelMap,
    voxels: VoxelMap,
    steam_frontier: &mut VoxelList,
) {
    let x_new = x.rem_euclid(MAX_COORD as i32) as usize;
    let y_new = y.rem_euclid(MAX_COORD as i32) as usize;
    let z_new = z.rem_euclid(MAX_COORD as i32) as usize;
    if outside_voxels[x_new][y_new][z_new] == 0 && voxels[x_new][y_new][z_new] == 0 {
        steam_frontier.push((x_new, y_new, z_new))
    }
}

#[test]
fn test_example() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_eq!(solve(input), 64)
}

#[test]
fn test_example_2() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_eq!(solve_2(input), 58)
}
