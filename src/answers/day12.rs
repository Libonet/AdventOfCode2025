use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::matrix::Pos;


struct Present {
    #[allow(dead_code)]
    pub shape: Vec<Pos>,
    pub area: usize,
}

struct Region {
    pub width: usize,
    pub length: usize,
    pub area: usize,
    pub presents: Vec<usize>,
}

type Input = (Vec<Present>, Vec<Region>);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day12")?; // inputs/dayxx
    
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
    let groups: Vec<&str> = contents.split("\n\n").collect();

    let mut presents = Vec::new();
    for present in groups[0..groups.len()-1].iter() {
        let lines: Vec<&str> = present.lines().collect();

        let mut area = 0;
        let mut shape = Vec::new();
        for (i,line) in lines[1..].iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    area += 1;
                    shape.push(Pos(i as i32, j as i32));
                }
            }
        }

        presents.push(Present { shape, area });
    }

    let mut regions = Vec::new();
    for line in groups.last().unwrap().lines() {
        let mut pair = line.split(":");
        let size = pair.next().unwrap();
        let mut size = size.split("x");
        let width = size.next().unwrap().parse().unwrap();
        let length = size.next().unwrap().parse().unwrap();

        let mut presents = Vec::new();
        for count in pair.next().unwrap().split_whitespace() {
            presents.push(count.parse().unwrap());
        }

        let area = width * length;

        regions.push(Region { width, length, area, presents });
    }

    (presents, regions)
}

fn part1(input: &Input) -> i32 {
    let (presents, regions) = input;

    let mut valid_regions = 0;
    let mut hard_regions = 0;
    for region in regions {
        let presents_area: usize = region.presents.iter()
            .enumerate()
            .map(|(i, count)| presents[i].area * count)
            .sum();

        if presents_area > region.area {
            continue;
        }
        // println!("presents area: {presents_area}");
        // println!("region area: {}\n", region.area);

        let present_count: usize = region.presents.iter()
            .sum();

        let region_trivial_presents = (region.width / 3)
            * (region.length / 3);
        
        if present_count <= region_trivial_presents {
            valid_regions += 1;
        } else {
            hard_regions += 1;
        }
    }

    println!("hard_regions = {hard_regions}");

    valid_regions
}

fn part2(_input: &Input) -> i32 {
    println!("Merry Christmas!");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 0);
    }
}
