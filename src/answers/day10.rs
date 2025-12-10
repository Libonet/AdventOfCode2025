use std::collections::VecDeque;
use std::{fs::read_to_string, io};
use std::time::Instant;

use microlp::{LinearExpr, Problem};

type Input = Vec<Machine>;

struct Machine {
    pub target: i64,
    pub buttons: Vec<Vec<bool>>,
    pub joltage: Vec<i64>,
}

impl Machine {
    pub fn configure_steps(&self) -> i64 {
        // println!("Configuring steps!!\n");
        let mut queue = VecDeque::new();

        // println!("butons: {:#?}", self.buttons);

        let mut buttons = Vec::new();
        for vals in &self.buttons {
            let mut button = 0;
            for (i,val) in vals.iter().rev().enumerate() {
                if *val {
                    button |= 1 << i;
                }
            }
            buttons.push(button);
        }

        queue.extend(buttons.iter().map(|button| (0, button, 0)));
        
        while let Some((curr, button, steps)) = queue.pop_front() {
            // println!("goal: {:#032b}", self.target);
            // println!("curr: {curr:#032b}, button: {button:#b}, steps: {steps}");
            if curr == self.target {
                // println!("FOUND TARGET IN {steps} STEPS!!!");
                return steps;
            }

            let new_curr = curr ^ *button;
            // println!("new : {new_curr:#032b}");
            queue.extend(buttons.iter().map(|button| (new_curr, button, steps+1)));
        }

        -1
    }

    pub fn joltage_steps(&self) -> i64 {
        if self.joltage.iter().all(|j| *j == 0) { return 0; }

        let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);
        let mut vars = Vec::new(); 

        for _ in 0..self.buttons.len() {
            vars.push(problem.add_integer_var(1., (0, i32::MAX)));
        }

        for constraint in 0..self.joltage.len() {
            let mut equation = LinearExpr::empty();
            for (i, var) in self.buttons.iter().enumerate() {
                if var[constraint] {
                    equation.add(vars[i], 1.);
                }
            }
            problem.add_constraint(
                equation,
                microlp::ComparisonOp::Eq,
                self.joltage[constraint] as f64
            );
        }

        problem.solve().unwrap().objective().round() as i64
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

            for val in target_group.chars().take(target_group.len()-1).skip(1) {
                match val {
                    '#' => { target = (target << 1) + 1 },
                    '.' => { target <<= 1 },
                    _ => {},
                }
            }

            let mut id_buttons = Vec::new();
            let mut joltage = Vec::new();
            for group in groups {
                if &group[0..1] == "(" {
                    let mut vals = Vec::new();
                    for val in group[1..group.len()-1].split(",") {
                        let id: usize = val.parse().unwrap();
                        vals.push(id);
                    }
                    id_buttons.push(vals);
                }

                if &group[0..1] == "{" {
                    let group: Vec<i64> = group[1..group.len()-1]
                        .split(',')
                        .map(|val| val.parse().unwrap())
                        .collect();
                    joltage = group;
                }
            }

            let mut buttons = Vec::new();
            let max = id_buttons
                .iter()
                .map(|vals| vals
                    .iter()
                    .max()
                    .unwrap())
                .max()
                .unwrap();

            for vals in &id_buttons {
                let mut button = vec![false; max+1];
                for val in vals {
                    button[*val] = true;
                }
                buttons.push(button);
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

//     #[test]
//     fn test_part2() {
//         let contents = "\
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
//
//         let input = parse(contents.to_string());
//
//         let res = part2(&input);
//
//         assert_eq!(res, 33);
//     }
}
