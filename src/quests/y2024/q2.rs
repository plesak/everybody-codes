use std::cmp::{max, min};
use std::collections::{HashSet};
use std::fmt::Display;
// use crate::utils;

use array2d::Array2D;
use itertools::Itertools;

fn parse1(input: String) -> (HashSet<String>, String, usize, usize) {
    let mut words = HashSet::new();

    let mut shortest = usize::MAX;
    let mut longest = usize::MIN;

    let mut lns = input.lines();

    lns.next().unwrap().split(':')
        .skip(1).next().unwrap().split(',')
        .for_each(|word| {
            shortest = min(word.len(), shortest);
            longest = max(word.len(), longest);
            words.insert(word.to_string());
        });

    lns.next();
    let sentence = lns.next().unwrap().to_string();

    (words, sentence, shortest, longest)
}

fn solve1(input: String) -> impl Display {
    let (words, sentence, shortest, longest) = parse1(input);
    let mut count = 0;

    for ln in shortest..=longest {
        let chars = sentence.chars().collect::<Vec<char>>();
        chars.windows(ln).for_each(|w| {
            let substring = w.iter().collect::<String>();
            if words.contains(&substring) {
                count += 1;
            }
        });
    }

    count
}

fn parse2(input: String) -> (HashSet<String>, Vec<String>, HashSet<usize>, usize) {
    let mut words = HashSet::new();
    let mut lens = HashSet::new();
    let mut lines = input.lines();
    let mut longest = usize::MIN;

    lines.next().unwrap().split(':')
        .skip(1).next().unwrap().split(',')
        .for_each(|word| {
            lens.insert(word.len());
            words.insert(word.to_string());
            // also insert reverse word
            words.insert(word.chars().rev().collect::<String>());
            longest = max(word.len(), longest);
        });

    lines.next();

    let mut sentences = Vec::new();
        while let Some(line) = lines.next() {
            sentences.push(line.to_string());
        }

    (words, sentences, lens, longest)

}


fn solve2(input: String) -> impl Display {
    let (words, sentences, lens, longest) = parse2(input);
    let mut count = 0;

    for sentence in sentences {
        // println!("processing sentence {}", sentence);
        let mut vals = vec![0; sentence.len()];

        for ln in (1..=longest).rev() {
            // dont have to check for words if no words with such length exist
            if lens.contains(&ln) {
                // iterate over valid starting indices
                for i in 0..=vals.len().saturating_sub(ln) {
                    let vals_window = &mut vals[i..i+ln];
                    // "if there are any 0s left in the considered window"
                    if vals_window.iter().sum::<i32>() < ln as i32 {
                        let substring = sentence[i..i+ln].to_string();
                        if words.contains(&substring) {
                            for v in vals_window.iter_mut() {
                                *v = 1;
                            }
                        }
                    }
                }
                // println!("just checked for size {}", ln);
                // println!("vals vector is {:?}", vals);
            }
        }

        count += vals.iter().sum::<i32>();
    }

    count
}

fn solve3(input: String) -> impl Display {
    let (words, sentences, lens, longest) = parse2(input);
    let nrows = sentences.len();
    let ncols = sentences[0].len();

    let letters = sentences.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let grid_letters = Array2D::from_rows(&letters).unwrap();
    let mut grid_vals = Array2D::filled_with(0, nrows, ncols);

    for ln in (1..=longest).rev() {
        if lens.contains(&ln) {
            // check row-wise
            for y in 0..nrows {
                for x in 0..ncols {
                    let mut xs:Vec<usize> = Vec::with_capacity(ln);
                    for xval in x..x+ln {
                        xs.push(xval % ncols);
                    }
                    let word = xs.iter().map(|xcor| {
                        grid_letters[(y, *xcor)]
                    }).join("");
                    if words.contains(&word) {
                        // println!("found row word {} at {},{}", word, x, y);
                        for xval in xs {
                            grid_vals[(y, xval)] = 1;
                        }
                    }
                }
            }
            // check col-wise
            if ln <= nrows {
                for y in 0..=nrows-ln {
                    for x in 0..ncols {
                        let word = (y..y+ln).into_iter().map(|ycor| {
                            grid_letters[(ycor, x)]
                        }).join("");
                        if words.contains(&word) {
                            // println!("found col word {} at {},{}", word, x, y);
                            for yval in y..y+ln {
                                grid_vals[(yval, x)] = 1;
                            }
                        }
                    }
                }
            }
        }
        // println!("grid looks like this");
        // println!("{:?}", grid_vals);
    }


    grid_vals.elements_row_major_iter().sum::<usize>()
}

pub fn solve(input: String, part: i32) -> String {
    match part {
        1 => {solve1(input).to_string()},
        2 => {solve2(input).to_string()},
        3 => {solve3(input).to_string()},
        _ => {"invalid part".to_string()}
    }
}