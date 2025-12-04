use std::collections::VecDeque;
use std::{fs::read_to_string, io};
use std::time::Instant;

type Input = Vec<Vec<i32>>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day03")?; // inputs/dayxx
    
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
        .map(|line| line.chars().map(|c| c.to_digit(10).expect("Input should be valid") as i32).collect())
        .collect()
}

fn part1(input: &Input) -> usize {
    solve(input, 2)
}

fn part2(input: &Input) -> usize {
    solve(input, 12)
}

fn find_min_val<T: Ord + Copy>(vec: &VecDeque<T>) -> Option<usize> {
    let len = vec.len();
    let mut i = 0;
    while i < len-1 {
        if vec[i] < vec[i+1] {
            break;
        }

        i+=1;
    }

    Some(i)
}

fn solve(input: &Input, size: usize) -> usize {
    assert!(size > 1);
    let mut result = 0;

    for bank in input.iter() {
        let mut max = VecDeque::with_capacity(size);
        let mut iter = bank.iter().rev();

        // save the first 'size' values
        for &val in iter.by_ref().take(size) {
            max.push_front(val);
        }

        for &val in iter {
            let leftmost = *max.front().expect("VecDeque should not be empty");

            if val >= leftmost {
                // remove worst value, prepend new value
                if let Some(idx) = find_min_val(&max) {
                    max.remove(idx);
                    max.push_front(val);
                } else {
                    // all values are 9
                    break;
                }
            }
        }

        let mut res = 0;
        for val in max {
            res = res*10 + val as usize;
        }
        result += res;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
987654321111111
811111111111119
234234234234278
818181911112111";

        let input = parse(contents.to_string());
        let res = part1(&input);

        assert_eq!(res, 357);
    }

    #[test]
    fn test_part2() {
        let contents = "\
987654321111111
811111111111119
234234234234278
818181911112111";

        let input = parse(contents.to_string());
        let res = part2(&input);

        assert_eq!(res, 3121910778619);
    }
}
