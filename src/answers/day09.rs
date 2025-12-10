use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::matrix::{Matrix, Pos};

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
            let y = split.next().unwrap().parse().unwrap();
            let x = split.next().unwrap().parse().unwrap();
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
    let mut map = HashMap::new();
    for i in 0..input.len()-1 {
        for j in i+1..input.len() {
            let area = get_rect_area(input[i], input[j]);
            map.insert((i,j), area);
        }
    }

    let mut kvp = map.into_iter().collect::<Vec<((usize, usize), i64)>>();
    kvp.sort_by_key(|(_, area)| *area);
    kvp.reverse();

    let mut max_area = 0;

    for ((a,b), area) in kvp {
        let region = Region::from_two_points(&input[a], &input[b]);
        let mut found_area = true;

        for point in input {
            if region.encloses(*point) {
                found_area = false;
                break;
            }
        }

        // also check midpoints
        if found_area {
            for c in 0..input.len()-1 {
                if region.encloses_line_midpoint(input[c], input[c+1]) {
                    found_area = false;
                    break;
                }
            }

            if region.encloses_line_midpoint(input[0], *input.last().unwrap()) {
                found_area = false;
            }
        }

        if found_area {
            max_area = area;
            break;
        }
    }

    max_area
}

fn get_rect_area(from: Pos, other: Pos) -> i64 {
    let width = 1 + from.0.abs_diff(other.0) as i64;
    let height = 1 + from.1.abs_diff(other.1) as i64;

    width * height
}

struct Region {
    min: Pos,
    max: Pos,
}

impl Region {
    pub fn from_two_points(p1: &Pos, p2: &Pos) -> Region {
        let minx = p1.0.min(p2.0);
        let miny = p1.1.min(p2.1);
        let maxx = p1.0.max(p2.0);
        let maxy = p1.1.max(p2.1);
        Region { min: Pos(minx, miny), max: Pos(maxx, maxy) }
    }

    pub fn encloses(&self, point: Pos) -> bool {
        point.0 > self.min.0 && point.0 < self.max.0 && point.1 > self.min.1 && point.1 < self.max.1
    }

    pub fn encloses_line_midpoint(&self, start: Pos, end: Pos) -> bool {
        let mid = Pos((start.0 + end.0) / 2, (start.1 + end.1) / 2);

        self.encloses(mid)
    }
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

    #[test]
    fn test_part2_strange_shape() {
        let contents = "\
7,0
11,0
11,7
9,7
9,6
2,6
2,5
9,5
9,3
2,3
2,2
7,2";

        let input = parse(contents.to_string());

        let res = part2(&input);

        assert_eq!(res, 24);
    }

/*
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..#XXXXXX#XX..
.........XXX..
..#XXXXXX#XX..
..#XXXXXX#XX..
.........#X#..
..............
*/
}
