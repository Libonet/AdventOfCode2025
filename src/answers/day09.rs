use std::collections::BinaryHeap;
use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::matrix::{Pos};

type Input = Vec<Pos>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day09")?; // inputs/dayxx
    
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
    contents.lines()
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            Pos(x,y)
        })
        .collect()
}

fn part1(input: &Input) -> i64 {
    let mut edges = BinaryHeap::new();

    for (i, node) in input.iter().enumerate() {
        let mut idx = i+1;
        while idx < input.len() {
            edges.push(Edge { id1: i, id2: idx, dis: area(node, &input[idx]) });
            idx += 1;
        }
    }

    let max = edges.pop().unwrap();

    // println!("max area node pair: {max:#?}");
    // println!("node {}: {}", max.id1, input[max.id1]);
    // println!("node {}: {}", max.id2, input[max.id2]);

    max.dis
}

fn area(p1: &Pos, p2: &Pos) -> i64 {
    let Pos(a,b) = p1;
    let Pos(x,y) = p2;

    (1 + b.abs_diff(*y) as i64) * (1 + a.abs_diff(*x) as i64)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    pub dis: i64,
    pub id1: usize,
    pub id2: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dis.cmp(&other.dis)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &Input) -> i64 {
    let mut edges = BinaryHeap::new();

    let mut i = 0;
    let len = input.len();
    while i < len {
        let mut idx = i+1;
        while idx < len {
            if !empty_area(input, i, idx) { 
                idx += 1;
                continue;
            }
            edges.push(Edge { id1: i, id2: idx, dis: area(&input[i], &input[idx]) });
            idx += 1;
        }

        i += 1;
    }

    let max = edges.pop().unwrap();

    // println!("max area node pair: {max:#?}");
    // println!("node {}: {}", max.id1, input[max.id1]);
    // println!("node {}: {}", max.id2, input[max.id2]);

    max.dis
}

fn empty_area(input: &Input, id1: usize, id2: usize) -> bool {
    let Pos(x1,y1) = input[id1];
    let Pos(x2,y2) = input[id2];

    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    for node in input {
        if (min_x+1..max_x).contains(&node.0)
            && (min_y+1..max_y).contains(&node.1) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 50);
    }

    #[test]
    fn test_part2() {
        let contents = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let input = parse(contents.to_string());

        let res = part2(&input);

        assert_eq!(res, 24);
    }
}
