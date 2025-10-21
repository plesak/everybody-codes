use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
// use array2d::Array2D;
// use crate::utils;

// fn parse_into_array2d(input: String) -> Array2D<usize> {
//     Array2D::from_rows(
//         &input.split("\n\n").map(|row| {
//             row.split_whitespace().map(|l| {
//                 l.parse::<usize>().unwrap()
//             }).collect::<Vec<usize>>()
//         }).collect::<Vec<Vec<usize>>>()
//     ).unwrap()
// }

fn parse_into_vectors(input: String) -> Vec<VecDeque<usize>> {
    let mut cols:Vec<VecDeque<usize>> = Vec::new();

    input.lines().enumerate().for_each(|(ix, row)| {
        row.split_whitespace().enumerate().for_each(|(idx, l)| {
            let val = l.parse::<usize>().unwrap();
            if ix == 0 {
                cols.push(VecDeque::from(vec![val]));
            } else {
                cols[idx].push_back(val);
            }
        })
    });

    cols
}

fn walk_cols(mut columns: Vec<VecDeque<usize>>, round: usize) -> Vec<VecDeque<usize>> {
    let num_cols = columns.len();
    let clapper_col = round % num_cols;
    let walk_col = (round+1) % num_cols;

    let clapper_val = columns[clapper_col].pop_front().unwrap();
    let col_len = columns[walk_col].len();
    let mut clapper_val_adj = clapper_val % (2*col_len);
    if clapper_val_adj == 0 { clapper_val_adj = 2*col_len; }
    // walk around left
    if clapper_val_adj <= col_len {
        columns[walk_col].insert(clapper_val_adj - 1, clapper_val);
    } else { // walk back around right
        columns[walk_col].insert(2 * col_len - clapper_val_adj + 1, clapper_val);
    }

    columns
}

fn solve1(input: String) -> impl Display {
    let mut columns = parse_into_vectors(input);
    let mut res= "hi".to_string();

    // println!("cols are {:?}", columns);
    for round in 0..10 {
        columns = walk_cols(columns, round);
        // println!("after round {}, cols are {:?}", round+1, columns);

        if round == 9 {
            res = columns.iter().map(|c| c[0].to_string()).collect::<String>();
        }
    }

    res
}

fn solve2(input: String) -> impl Display {
    let mut columns = parse_into_vectors(input);

    let mut shouts:HashMap<String, usize> = HashMap::new();

    // println!("cols are {:?}", columns);
    let mut round = 0;
    loop {
        columns = walk_cols(columns, round);

        // println!("after round {}, cols are {:?}", round+1, columns);
        let shout = columns.iter().map(|c| c[0].to_string()).collect::<String>();
        if shouts.contains_key(&shout) && shouts[&shout] == 2023  {
            return shout.parse::<usize>().unwrap()*(round+1)
        }
        let shout_ref = shouts.entry(shout).or_insert(0);
        *shout_ref += 1;
        round += 1;
    }
}

fn solve3(input: String) -> impl Display {
    let mut columns = parse_into_vectors(input);

    let mut floors:HashSet<Vec<VecDeque<usize>>> = HashSet::new();
    let mut highest_shout = usize::MIN;

    // println!("cols are {:?}", columns);
    let mut round = 0;
    loop {
        columns = walk_cols(columns, round);

        // println!("after round {}, cols are {:?}", round+1, columns);
        let shout = columns.iter()
            .map(|c| c[0].to_string()).collect::<String>()
            .parse::<usize>().unwrap();
        highest_shout = max(shout, highest_shout);

        if !floors.insert(columns.clone()) {
            return highest_shout
        }

        round += 1;
    }
}

pub fn solve(input: String, part: i32) -> String {
    match part {
        1 => {solve1(input).to_string()},
        2 => {solve2(input).to_string()},
        3 => {solve3(input).to_string()},
        _ => {"invalid part".to_string()}
    }
}