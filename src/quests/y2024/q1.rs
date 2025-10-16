use std::collections::HashMap;
use std::fmt::Display;
use lazy_static::lazy_static;
// use crate::utils;

lazy_static! {
    static ref STRENGTH_MAP: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('A', 0);
        map.insert('B', 1);
        map.insert('C', 3);
        map.insert('D', 5);
        map.insert('x', 0);
        map
    };
}
fn solve1(input: String) -> impl Display {
    input.chars().map(|c| STRENGTH_MAP[&c]).sum::<i32>()
}

fn solve2(input: String) -> impl Display {
    input.chars().collect::<Vec<char>>()
        .chunks_exact(2)
        .map(|chunk| {
            let c1 = chunk[0];
            let c2 = chunk[1];
            if &c1 == &'x' || &c2 == &'x' {
                STRENGTH_MAP[&c1] + STRENGTH_MAP[&c2]
            } else {
                STRENGTH_MAP[&c1] + STRENGTH_MAP[&c2] + 2
            }
        })
        .sum::<i32>()
}

fn solve3(input: String) -> impl Display {
    input.chars().collect::<Vec<char>>()
        .chunks_exact(3)
        .map(|chunk| {
            let c1 = chunk[0];
            let c2 = chunk[1];
            let c3 = chunk[2];
            let xs = chunk.iter()
                .filter(|&&c| c == 'x').count();
            let extra_potions = match xs {
                0 => 6,
                1 => 2,
                _ => 0
            };
            STRENGTH_MAP[&c1] + STRENGTH_MAP[&c2] + STRENGTH_MAP[&c3] + extra_potions
        })
        .sum::<i32>()
}

pub fn solve(input: String, part: i32) -> String {
    match part {
        1 => {solve1(input).to_string()},
        2 => {solve2(input).to_string()},
        3 => {solve3(input).to_string()},
        _ => {"invalid part".to_string()}
    }
}