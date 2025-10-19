use std::io::Write;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;

use crate::quests;

// setup pieces for new quest
pub fn scaffold(year:&str, quest:&str) -> std::io::Result<()> {

    // make directory in year and create setup files
    let path_quest = format!("input/{}/q{}", year, quest);
    fs::create_dir(&path_quest)?;

    let path1 = format!("{}/p1", path_quest);
    fs::create_dir(&path1)?;
    File::create(Path::new(&format!("{}/input.txt", path1)))?;
    File::create(Path::new(&format!("{}/test.txt", path1)))?;

    let path2 = format!("{}/p2", path_quest);
    fs::create_dir(&path2)?;
    File::create(Path::new(&format!("{}/input.txt", path2)))?;
    File::create(Path::new(&format!("{}/test.txt", path2)))?;

    let path3 = format!("{}/p3", path_quest);
    fs::create_dir(&path3)?;
    File::create(Path::new(&format!("{}/input.txt", path3)))?;
    File::create(Path::new(&format!("{}/test.txt", path3)))?;

    // make rust file
    let path_rust = format!("src/quests/y{}/q{}.rs", year, quest);
    File::create(Path::new(&path_rust))?;
    fs::copy("src/quests/template.txt", &path_rust).expect("failed to copy quest template");

    // add mod declaration
    let path_module = format!("src/quests/y{}/mod.rs", year);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path_module)?;
    writeln!(file, "pub mod q{};", quest)?;

    Ok(())
}

// read file as string
pub fn run(year: i32, quest: i32, part: i32, solve: bool) -> String {
    // find correct file
    let input_filename = match solve {
        true => { format!("input/{}/q{}/p{}/input.txt", year, quest, part) }
        false => { format!("input/{}/q{}/p{}/test.txt", year, quest, part) }
    };

    // collect input
    let input = fs::read_to_string(&input_filename).unwrap();

    let result = match (year, quest) {
        (2024, 1) => {
            quests::y2024::q1::solve(input, part)
        },
        (2024, 2) => {
            quests::y2024::q2::solve(input, part)
        },
        (2024, 3) => {
            quests::y2024::q3::solve(input, part)
        },
        _ => {
            format!("y{} q{} is not implemented yet!", year, quest)
        }
    };

    if solve {
        println!("Result of year {}, quest {}, part {}:", year, quest, part);
        println!("{}", result);
    } else {
        println!("Practice run y{} q{} p{}:", year, quest, part);
        println!("{}", result);
    }

    result
}
