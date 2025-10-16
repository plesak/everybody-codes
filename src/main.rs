mod scaffold;
mod utils;
mod quests;

// use std::env;
use clap::{Parser};
use scaffold::{scaffold, run};
// use utils::{};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// year
    #[arg(required = true)]
    year: i32,

    /// quest
    #[arg(required = true)]
    quest: i32,

    /// part
    #[arg(required = true)]
    part: i32,

    /// command: solve, test, scaffold
    #[arg(required = true)]
    command: String,
}

fn main() {
    let args = Args::parse();

    let command = args.command;
    let quest = args.quest;
    let part = args.part;
    // let default_year = match env::var("EVERYONECODES_YEAR") {
    //     Ok(year) => year.parse().unwrap(),
    //     Err(_) => 2025
    // };
    let year = args.year;

    match command.as_str() {
        "scaffold" => {
            scaffold(&year.to_string(), &quest.to_string()).unwrap();
        },
        "test" => {
            run(year, quest, part,false);
        }
        "solve" => {
            run(year, quest, part,true);
        },
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
