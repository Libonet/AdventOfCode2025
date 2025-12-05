use std::{fs::read_to_string, io};
use std::time::Instant;

use crate::matrix::{Matrix, Pos};

type Input = Matrix<char>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day04")?; // inputs/dayxx
    
    let mut input = parse(contents);
    
    println!("Part1:");
    let now = Instant::now();
    let part1_res = part1(&mut input.clone());
    let elapsed = now.elapsed();
    println!("result = {part1_res}");
    println!("Time taken: {:.2?}", elapsed);

    println!("Part2:");
    let now = Instant::now();
    let part2_res = part2(&mut input);
    let elapsed = now.elapsed();
    println!("result = {part2_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn parse(contents: String) -> Input {
    let cols = contents.find('\n').unwrap();
    let values = contents.replace("\n", "").chars().collect();
    
    (values, cols).try_into().unwrap()
}

fn part1(input: &mut Input) -> usize {
    let rows = input.rows();
    let cols = input.cols();
    let mut adjacency = Matrix::<i8>::with_default(rows, cols, -1);

    let possible_adj = [Pos(0,1), Pos(1,-1), Pos(1,0), Pos(1,1)];
    let mut remove_count = 0;

    for pos in Pos::iter(rows, cols) {
        // println!("pos: {pos}, val: {val}");
        if input[pos] != '@' { continue; }

        adjacency[pos] += 1;

        for p_adj in possible_adj {
            let next = pos + p_adj;

            if let Some('@') = input.get(next) {
                adjacency[pos] += 1;
                adjacency[next] += 1;

                // assert!(adjacency[pos] < 10);
                // assert!(adjacency[next] < 10);
            }

        }

        if (0..4).contains(&adjacency[pos]) {
            input[pos] = '.';
            remove_count += 1;
        }
    }

    // println!("adjacency:\n{adjacency}");
    // for (pos, val) in adjacency.iter_pos() {
    //     if (0..4).contains(val) {
    //         input[pos] = '.';
    //         remove_count += 1;
    //     }
    // }

    remove_count
}

fn part2(input: &mut Input) -> usize {
    let mut res = 0;

    loop {
        let remove_count = part1(input);
        if remove_count == 0 { break; }

        res += remove_count;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let mut input = parse(contents.to_string());

        let res = part1(&mut input);

        assert_eq!(res, 13);
    }

    #[test]
    fn test_part2() {
        let contents = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let mut input = parse(contents.to_string());

        let res = part2(&mut input);

        assert_eq!(res, 43);
    }
}
