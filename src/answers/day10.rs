use std::collections::{HashMap, VecDeque};
use std::{fs::read_to_string, io};
use std::time::Instant;

type Input = Vec<Machine>;

struct Machine {
    pub target: i64,
    pub buttons: Vec<i64>,
    pub joltage: Vec<i64>,
}

impl Machine {
    pub fn configure_steps(&self) -> i64 {
        // println!("Configuring steps!!\n\n");
        let mut queue = VecDeque::new();

        queue.extend(self.buttons.iter().map(|button| (0, button, 0)));
        
        while let Some((curr, button, steps)) = queue.pop_front() {
            // println!("goal: {:#032b}", self.target);
            // println!("curr: {curr:#032b}, button: {button:#b}, steps: {steps}");
            if curr == self.target {
                // println!("FOUND TARGET IN {steps} STEPS!!!");
                return steps;
            }

            let new_curr = curr ^ *button;
            // println!("new : {new_curr:#032b}");
            queue.extend(self.buttons.iter().map(|button| (new_curr, button, steps+1)));
        }

        -1
    }

    pub fn joltage_steps(&self) -> i64 {
        if self.joltage.iter().all(|j| *j == 0) { return 0; }

        let buttons: Vec<Vec<usize>> = self.buttons.iter().map(|button| {
            let mut button = *button;
            let mut res = Vec::new();
            let mut i = 0;
            while button > 0 {
                if button & 1 == 1 {
                    res.push(i);
                }
                button >>= 1;
                i += 1;
            }
            res
        }).collect();

        0
    }
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day10")?; // inputs/dayxx
    
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
            let mut groups = line.split_whitespace();
            let mut target = 0;
            let target_group = groups.next().unwrap();
            let mut bits = 0;
            for val in target_group.chars().take(target_group.len()-1).skip(1) {
                bits += 1;
                match val {
                    '#' => { target = (target << 1) + 1 },
                    '.' => { target <<= 1 },
                    _ => {},
                }
            }

            let mut buttons = Vec::new();
            let mut joltage = Vec::new();
            for group in groups {
                if &group[0..1] == "(" {
                    let mut button = 0;
                    for val in group[1..group.len()-1].split(',') {
                        let shift: usize = val.parse().unwrap();
                        button ^= 1 << (bits as usize - shift - 1);
                    }
                    buttons.push(button);
                }

                if &group[0..1] == "{" {
                    let group: Vec<i64> = group[1..group.len()-1]
                        .split(',')
                        .map(|val| val.parse().unwrap())
                        .collect();
                    joltage = group;
                }
            }
            
            Machine { target, buttons, joltage }
        })
        .collect()
}

fn part1(input: &Input) -> i64 {
    let mut res = 0;

    for machine in input {
        res += machine.configure_steps();
    }

    res
}

fn part2(input: &Input) -> i64 {
    let mut res = 0;

    for machine in input {
        res += machine.joltage_steps();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 7);
    }

    #[test]
    fn test_part2() {
        let contents = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 33);
    }
}
