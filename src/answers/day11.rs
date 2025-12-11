use std::collections::HashMap;
use std::{fs::read_to_string, io};
use std::time::Instant;

type Paths = HashMap<String, Vec<String>>;

type Input = Paths;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day11")?; // inputs/dayxx
    
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
    let mut paths = Paths::new();
    for line in contents.lines() {
        let mut pair = line.split(':');
        let start = pair.next().unwrap();
        let ends = pair.next().unwrap().split_whitespace();
        for val in ends {
            let val = val.to_string();
            paths.entry(start.to_string()).and_modify(|vec| vec.push(val.clone())).or_insert(vec![val]);
        }
    }

    paths
}

fn part1(input: &Input) -> i64 {
    let mut memo = HashMap::new();
    memo.insert("out".to_string(), 1);

    solve(input, &mut memo, "you")
}

fn solve(paths: &Input, memo: &mut HashMap<String, i64>, curr: &str) -> i64 {
    if let Some(ways) = memo.get(curr) {
        return *ways;
    }

    // println!("Curr: {curr}");
    let mut ways = 0;
    if let Some(ends) = paths.get(curr) {
        for end in ends {
            ways += solve(paths, memo, end);
        }
    }

    memo.insert(curr.to_string(), ways);
    ways
}

fn part2(input: &Input) -> i64 {
    let mut memo = HashMap::new();
    memo.insert("out".to_string(), 1);

    let dac_to_out = solve(input, &mut memo, "dac");
    let fft_to_out = solve(input, &mut memo, "fft");

    // ------------------

    let mut memo = HashMap::new();
    memo.insert("dac".to_string(), 1);

    let fft_to_dac = solve(input, &mut memo, "fft");
    let svr_to_dac = solve(input, &mut memo, "svr");

    // ------------------

    let mut memo = HashMap::new();
    memo.insert("fft".to_string(), 1);

    let dac_to_fft = solve(input, &mut memo, "dac");
    let svr_to_fft = solve(input, &mut memo, "svr");

    // ------------------

    (fft_to_out * dac_to_fft * svr_to_dac)
    + (dac_to_out * fft_to_dac * svr_to_fft)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let input = parse(contents.to_string());

        let res = part1(&input);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_part2() {
        let contents = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let input = parse(contents.to_string());

        let res = part2(&input);

        assert_eq!(res, 2);
    }
}
