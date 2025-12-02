use std::ops::{Add, Sub};
use std::{fs::read_to_string, io};
use std::time::Instant;

#[derive(Debug)]
enum Rotation {
    L(i32),
    R(i32),
}

impl Rotation {
    fn rotate(&self, curr: i32) -> i32 {
        match self {
            Rotation::L(i) => curr.sub(i).rem_euclid(100),
            Rotation::R(i) => curr.add(i).rem_euclid(100),
        }
    }

    fn rotate_rem(&self, curr: i32) -> (i32, i32) {
        let new = match self {
            Rotation::L(i) => {
                curr.sub(i)
            },
            Rotation::R(i) => {
                curr.add(i)
            },
        };

        let fixed = if new.is_negative() {
            new.abs() + if curr == 0 { 0 } else { 100 }
        } else { new };
        let crossed = fixed.div_euclid(100);
        (new.rem_euclid(100), crossed)
    }
}

type Input = Vec<Rotation>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day1-1")?; // inputs/dayx-x
    
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
            let mut chars = line.chars();
            // println!("line = {}", line);
            let rotation = chars.next().expect("Input should not be empty");
            match rotation {
                'L' => Rotation::L(chars.as_str().parse().expect("Rotation should be valid i32")),
                'R' => Rotation::R(chars.as_str().parse().expect("Rotation should be valid i32")),
                _ => unreachable!("Input should be well formed"),
            }
        })
        .collect()
}

fn part1(input: &Input) -> i32 {
    let mut curr = 50;
    let mut zero_count = 0;
    
    for rotation in input {
        curr = rotation.rotate(curr);
        curr.eq(&0).then(|| zero_count+=1);
    }

    zero_count
}

fn part2(input: &Input) -> i32 {
    let mut curr = 50;
    let mut zero_count = 0;
    
    for rotation in input {
        let crossed;
        (curr, crossed) = rotation.rotate_rem(curr);
        zero_count += crossed;
        if curr == 0 && crossed == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_rem_exact() {
        let input = vec![Rotation::L(50)];

        let zero_count = part2(&input);

        assert_eq!(zero_count, 1);
    }

    #[test]
    fn test_rotate_rem_exact2() {
        let input = vec![Rotation::L(150)];

        let zero_count = part2(&input);

        assert_eq!(zero_count, 2);
    }

    #[test]
    fn test_rotate_rem_aoc2() {
        let input = vec![Rotation::R(1000)];

        let zero_count = part2(&input);

        assert_eq!(zero_count, 10);
    }

    #[test]
    fn test_rotate_rem_aoc1() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let parsed = parse(input.to_string());
        let zero_count = part2(&parsed);

        assert_eq!(zero_count, 6);
    }
}
