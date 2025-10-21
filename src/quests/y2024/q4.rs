use std::fmt::Display;
// use crate::utils;

fn parse1(input: String) -> Vec<usize> {
    let mut nums:Vec<usize> = Vec::new();
    for ln in input.lines() {
        nums.push(ln.parse::<usize>().unwrap());
    }
    nums.sort_unstable();

    nums
}


fn solve1(input: String) -> impl Display {
    let nails = parse1(input);
    let min = nails[0];
    nails.iter().map(|&nail| nail - min).sum::<usize>()
}

fn solve2(input: String) -> impl Display {

    let mut lines = input.lines();
    let mut cumsum = 0;
    let mut processed = 1;

    let mut smallest = lines.next().unwrap().parse::<usize>().unwrap();
    print!("starting with smallest = {}", smallest);
    while let Some(line) = lines.next() {
        let val = line.parse::<usize>().unwrap();
        if val > smallest {
            cumsum += val - smallest;
        } else {
            cumsum += processed * (smallest - val);
            smallest = val;
        }
        processed += 1;
    }

    cumsum
}

fn solve3(input: String) -> impl Display {
    let mut nums = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    nums.sort_unstable();
    let median = nums[nums.len() / 2];
    // let mean = nums.iter().sum::<usize>() / nums.len();
    nums.iter().map(|&num| num.abs_diff(median)).sum::<usize>()
}

pub fn solve(input: String, part: i32) -> String {
    match part {
        1 => {solve1(input).to_string()},
        2 => {solve2(input).to_string()},
        3 => {solve3(input).to_string()},
        _ => {"invalid part".to_string()}
    }
}