use std::fmt::Display;
use array2d::Array2D;
use crate::utils::{print_array2d};

fn get_map(input: String) -> Array2D<usize> {
    let lines = input.lines().map(|s| {
        s.chars().map(|c| {
            if c == '.' {
                0
            } else {
                1
            }
        }).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();
    Array2D::from_rows(&lines).unwrap()
}

fn solve1(input: String) -> impl Display {
    let mut map = get_map(input);

    let mut dug_up_this_round = 1;
    while dug_up_this_round > 0 {
        dug_up_this_round = 0;
        for x in 1..map.num_rows()-1 {
            for y in 1..map.num_columns()-1 {
                let val = map[(x, y)];
                if val == 0 { continue; }
                let neighbors = vec![map[(x, y-1)], map[(x+1, y)], map[(x, y+1)], map[(x-1, y)]];
                if neighbors.iter().all(|v| v == &val || v == &(val+1)) {
                    map[(x, y)] = val+1;
                    dug_up_this_round += 1;
                }
            }
        }
        // println!("dug_up_this_round: {}", dug_up_this_round);
        // print_array2d(&map);
    }

    print_array2d(&map);
    map.elements_row_major_iter().sum::<usize>()
}

fn solve2(input: String) -> impl Display {
    solve1(input)
}

fn solve3(input: String) -> impl Display {
    let mut map = get_map(input);

    let mut dug_up_this_round = 1;
    while dug_up_this_round > 0 {
        dug_up_this_round = 0;
        for x in 1..map.num_rows()-1 {
            for y in 1..map.num_columns()-1 {
                let val = map[(x, y)];
                if val == 0 { continue; }
                // get all diagonal neighbors (off-grid = 0)
                let ncs = vec![(x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y), (x+1, y), (x-1, y+1), (x, y+1), (x+1, y+1)];
                let mut neighbors:Vec<usize> = Vec::with_capacity(8);
                for nc in ncs {
                    if let Some(neighbor) = map.get(nc.0, nc.1) {
                        neighbors.push(*neighbor);
                    } else {
                        neighbors.push(0);
                    }
                }

                if neighbors.iter().all(|v| v == &val || v == &(val+1)) {
                    map[(x, y)] = val+1;
                    dug_up_this_round += 1;
                }
            }
        }
        // println!("dug_up_this_round: {}", dug_up_this_round);
        // print_array2d(&map);
    }

    print_array2d(&map);
    map.elements_row_major_iter().sum::<usize>()

}

pub fn solve(input: String, part: i32) -> String {
    match part {
        1 => {solve1(input).to_string()},
        2 => {solve2(input).to_string()},
        3 => {solve3(input).to_string()},
        _ => {"invalid part".to_string()}
    }
}