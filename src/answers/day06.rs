use std::{fs::read_to_string, io};
use std::time::Instant;

#[derive(PartialEq, Eq, Debug)]
enum Ops {
    Add,
    Mul,
}

impl Ops {
    fn apply(&self, a: &mut i64, b: i64) {
        match self {
            Ops::Add => *a += b,
            Ops::Mul => *a *= b,
        }
    }
}

type Input = (Vec<Vec<char>>, Vec<Ops>);

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day06")?; // inputs/dayxx
    
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
    let lines: Vec<Vec<char>> = contents.lines().map(|str| str.chars().collect()).collect();
    
    let ops = lines.last().unwrap().iter().collect::<String>()
        .split_whitespace()
        .map(|op| {
            match op {
                "+" => Ops::Add,
                "*" => Ops::Mul,
                _ => unreachable!("Input doesnt have other ops"),
            }
        })
        .collect();

    (lines[0..lines.len()-1].to_vec(), ops)
}

fn part1(input: &Input) -> i64 {
    let (lines, ops) = input;

    let mut rows: Vec<Vec<String>> = Vec::with_capacity(4);
    for line in &lines[0..lines.len()] {
        let row = line.split(|c| *c == ' ')
            .filter(|substr| !substr.is_empty())
            .map(|str| str.iter().collect::<String>())
            .collect();
        rows.push(row);
    }

    let len = ops.len();
    let mut idx = 0;
    let mut res = 0;
    while idx < len {
        let mut inner_res = if ops[idx] == Ops::Add { 0 } else { 1 };
        for row in rows.iter() {
            ops[idx].apply(&mut inner_res, row[idx].parse().unwrap());
        }
        res += inner_res;
        idx += 1;
    }

    res
}

fn part2(input: &Input) -> i64 {
    let (lines, ops) = input;
    let rows = lines.len();

    let mut res = 0;
    let mut problem = 0;
    let problems = ops.len();
    let mut inner_res = if ops[problem] == Ops::Add { 0 } else { 1 };

    let mut val_col = 0;
    let columns = lines.first().unwrap().len();
    let mut row = 0;

    // Move vertically on the input.
    // If we find a column full of ' ' then we go to next problem
    while val_col < columns {
        let mut value = String::new();
        while row < rows && lines[row][val_col] == ' ' { row+=1; }
        while row < rows && lines[row][val_col] != ' ' { 
            value.push(lines[row][val_col]);
            row += 1;
        }

        if value.is_empty() {
            // empty row, next problem
            res += inner_res;
            problem += 1;
            if problem < problems { 
                inner_res = if ops[problem] == Ops::Add { 0 } else { 1 };
            }
        } else {
            ops[problem].apply(&mut inner_res, value.trim().parse().unwrap());
        }
        
        row = 0;
        val_col += 1;
    }

    res += inner_res;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 4277556);
    }

    #[test]
    fn test_part2() {
        let contents = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let input = parse(contents.to_string());

        let res = part2(&input);

        assert_eq!(res, 3263827);
    }
}
