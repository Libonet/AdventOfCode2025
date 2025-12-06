use std::{error::Error, fs, io::{self, Write}, path::Path, process::exit, time::Instant};
use advent_of_code_2025::answers;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Input the day to get the day's answer. (0 for all)");

    let stdin = io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).expect("Should get a correct string");

    if input.starts_with("make") {
        return add_new_day(&input);
    }

    let num: i32 = match input.trim_end().parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    if num != 0 {
        let now = Instant::now();
        get_day(num);
        let elapsed = now.elapsed();
        println!("Time taken for day {num}: {elapsed:?}");
    } else {
        let now = Instant::now();
        for day in 1..=25 {
            get_day(day);
        }
        let elapsed = now.elapsed();
        println!("Time taken for all days: {elapsed:?}");
    }

    Ok(())
}

fn get_day(num: i32) {
    match num {
        1 => get_answer(1, answers::day01::answer),
        2 => get_answer(2, answers::day02::answer),
        3 => get_answer(3, answers::day03::answer),
        4 => get_answer(4, answers::day04::answer),
        5 => get_answer(5, answers::day05::answer),
        6 => get_answer(6, answers::day06::answer),
        // 7 => get_answer(7, answers::day07::answer),
        // 8 => get_answer(8, answers::day08::answer),
        // 9 => get_answer(9, answers::day09::answer),
        // 10 => get_answer(10, answers::day10::answer),
        // 11 => get_answer(11, answers::day11::answer),
        // 12 => get_answer(12, answers::day12::answer),
        _ => {
            eprintln!("Error: Day should exist");
            exit(2);
        }
    }
}

fn get_answer(day: i32, answer: impl Fn() -> Result<(), io::Error>) {
    if let Err(e) = answer() {
        eprintln!("Error on Day {day}: {e}");
        exit(3);
    }
}

fn add_new_day(input: &str) -> Result<(), Box<dyn Error>> {
    let input = input.strip_prefix("make").unwrap();
    let day: i32 = input.trim().parse().expect("Should be a number after make");
    let source = "./src/answers/base.rs";
    let dest_day = "day".to_string()
        + (if day < 10 { "0" } else { "" } )
        + &day.to_string();
    let destination = "./src/answers/".to_string() + &dest_day + ".rs";

    if Path::new(&destination).exists() { 
        panic!("Trying to overwrite existing day!!!");
    }

    println!("Copying from {source} to {destination}");

    fs::copy(source, destination)?;

    println!("Copy successful");

    let mut ans = fs::OpenOptions::new()
        .append(true)
        .open("./src/answers.rs")?;

    println!("Answers open to add mod");

    ans.write_all(&("pub mod ".to_string() + &dest_day + ";\n").into_bytes())?;

    Ok(())
}
