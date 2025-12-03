use std::ops::{Add, RangeInclusive};
use std::{fs::read_to_string, io};
use std::time::Instant;

type Input = Vec<RangeInclusive<usize>>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day02")?; // inputs/dayx-x
    
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
    contents.split(',')
        .map(|range| {
            let range: Vec<usize> = range
                .split('-')
                .map(|num| {
                    num.trim().parse().expect("Input should be valid usize")
                })
                .collect();
            range[0]..=range[1]
        })
        .collect()
}

fn digits(num: usize) -> u32 {
    num.ilog10().add(1)
}

// fn next_invalid_id(num: usize) -> (usize, usize) {
//     let shift = 10_usize.pow(digits(num)/2);
//     let lower = num / shift;
//     (lower * shift + lower, lower)
// }

fn part1(input: &Input) -> usize {
    let mut sum = 0;

    let mut ranges = Vec::with_capacity(input.len());
    for range in input.iter() {
        let mut start = *range.start();
        let mut end = *range.end();
        let end_dig = digits(end);
        if !end_dig.is_multiple_of(2) {
            end = 10_usize.pow(end_dig-1);
        }

        // Partition the range into a range of invalid ID candidates
        let dig = digits(start);
        if !dig.is_multiple_of(2) {
            let fixed_start = 10_usize.pow(dig);
            start = 10_usize.pow(dig+1).min(end);
            ranges.push(fixed_start..=(start-1));
        }
        while start < end {
            let dig = digits(start);
            let fixed_start = if !dig.is_multiple_of(2) {
                10_usize.pow(dig)
            } else {
                start
            };
            start = 10_usize.pow(dig+1).min(end);
            ranges.push(fixed_start..=start);
        }
    }

    for range in ranges {
        // println!("range: {range:?}");
        
        let mut start = *range.start();
        let end = *range.end();

        let shift = 10_usize.pow(digits(start)/2);
        let mut lower = start / shift;
        start = lower * shift + lower;
        while start <= end {
            if range.contains(&start) {
                // println!("start: {start}");
                sum += start;
            }

            lower += 1;
            start = lower * shift + lower;
        }
    }

    sum
}

struct Regex {
    str: Vec<char>,
    prefix: usize,
    prefix_index: usize,
    index: usize,
}

impl Regex {
    pub fn new(str: String) -> Self {
        Self { str: str.chars().collect(), prefix: 1, prefix_index: 0, index: 1 }
    }

    pub fn check_invalid_id(mut self) -> bool {
        let len = self.str.len();

        while self.index < len {
            if self.prefix > len/2 { return false; }

            if self.str[self.index] == self.str[self.prefix_index] {
                self.index += 1;
                self.prefix_index = (self.prefix_index + 1) % self.prefix;
            } else {
                self.prefix_index = 0;
                self.prefix += 1;
                while !len.is_multiple_of(self.prefix) {
                    self.prefix += 1;
                }
                self.index = self.prefix;
            }
        }

        if self.prefix > len/2 { return false; }
        true
    }
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;

    for range in input {
        for num in range.clone() {
            let num_str = num.to_string();
            let regex = Regex::new(num_str);
            if regex.check_invalid_id() {
                sum += num;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let parsed = parse(input.to_string());

        assert_eq!(part1(&parsed), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let parsed = parse(input.to_string());

        assert_eq!(part2(&parsed), 4174379265);
    }
}
