use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::matrix::{Matrix, Pos};

type Input = (Matrix<char>, Pos);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day07")?; // inputs/dayxx
    
    let input = parse(contents);
    
    println!("Part1:");
    let now = Instant::now();
    let part1_res = part1(&input);
    let elapsed = now.elapsed();
    println!("result = {part1_res}");
    println!("Time taken: {:.2?}", elapsed);

    println!("Part2:");
    let now = Instant::now();
    let part2_res = part2(&input);
    let elapsed = now.elapsed();
    println!("result = {part2_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn parse(contents: String) -> Input {
    let start = contents.find("S").unwrap() as i32;
    (contents.try_into().expect("Input should be a valid matrix"), Pos(0, start))
}

fn part1(input: &Input) -> i64 {
    let (mat, start) = input;
    let mut splits = 0;

    let mut visited = HashSet::new();
    let mut beams = VecDeque::new();

    beams.push_back(*start);

    while let Some(beam) = beams.pop_front() {
        // println!("Curr beam: {beam}");
        // if already visited, continue. Else, mark as visited
        if !visited.insert(beam) { continue; }

        let down_pos = beam + Pos(1, 0);
        match mat.get(down_pos) {
            Some('^') => {
                // println!("Found splitter");
                splits += 1;
                beams.push_back(down_pos + Pos(0, -1));
                beams.push_back(down_pos + Pos(0, 1));
            },
            Some('.') => {
                // println!("Going down!");
                beams.push_back(down_pos);
            },
            // Some(c) => { println!("Unexpected char: {c}"); }
            _ => {} // { println!("Didn't match anything!"); }
        } 
    }

    splits
}

fn part2(input: &Input) -> i64 {
    let (mat, start) = input;
    multiple_worlds(mat, &mut HashMap::new(), *start)
}

fn multiple_worlds(
    mat: &Matrix<char>,
    cache: &mut HashMap<Pos, i64>,
    beam: Pos,
) -> i64 {
    // println!("Curr beam: {beam}");
    // if already visited, continue. Else, return cache
    if let Some(res) = cache.get(&beam) {
        return *res;
    }

    let down_pos = beam + Pos(1, 0);
    let mut timelines;
    match mat.get(down_pos) {
        Some('^') => {
            // println!("Found splitter");
            timelines = multiple_worlds(mat, cache, down_pos + Pos(0,-1));
            timelines += multiple_worlds(mat, cache, down_pos + Pos(0,1));
        },
        Some('.') => {
            // println!("Going down!");
            timelines = multiple_worlds(mat, cache, down_pos);
        },
        // Some(c) => { println!("Unexpected char: {c}"); }
        _ => { timelines = 1 } // { println!("Didn't match anything!"); }
    } 

    cache.insert(beam, timelines);
    timelines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 21);
    }

    #[test]
    fn test_part2() {
        let contents = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let input = parse(contents.to_string());

        let res = part2(&input);

        assert_eq!(res, 40);
    }
}
