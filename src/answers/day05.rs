use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::insert_list::InsertList;
use crate::bintree::RangeTree;

type Input = (RangeTree, Vec<i64>);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day05")?; // inputs/dayxx
    
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
    let split: Vec<&str> = contents.split("\n\n").collect();
    let ranges = split[0].lines()
        .map(|range| {
            let mut vals = range.split("-");
            let start: i64 = vals.next().unwrap().trim().parse().unwrap();
            let end: i64 = vals.next().unwrap().trim().parse().unwrap();
            start..=end
        })
        .collect();
    let values = split[1].lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (RangeTree::new(InsertList::new(ranges)), values)
}

fn part1(input: &Input) -> usize {
    let (tree, values) = input;

    values.iter().filter(|val| tree.check_fresh(**val)).count()
}

fn part2(input: &Input) -> usize {
    let (tree, _) = input;

    // println!("Tree:\n{tree:#?}");

    tree.fresh_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 3);
    }

    #[test]
    fn test_part2() {
        let contents = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let input = parse(contents.to_string());

        let res = part2(&input);

        assert_eq!(res, 14);
    }
}
