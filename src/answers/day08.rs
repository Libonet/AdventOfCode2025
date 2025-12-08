use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::{fs::read_to_string, io};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pub id: usize,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub component: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    pub dis: i64,
    pub id1: usize,
    pub id2: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dis.cmp(&other.dis)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn euclidean_distance(&self, other: &Node) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

type Input = Vec<Node>;

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("inputs/day08")?; // inputs/dayxx
    
    let input = parse(contents);
    
    println!("Part1:");
    let now = Instant::now();
    let part1_res = part1(input.clone());
    let elapsed = now.elapsed();
    println!("result = {part1_res}");
    println!("Time taken: {:.2?}", elapsed);

    println!("Part2:");
    let now = Instant::now();
    let part2_res = part2(input);
    let elapsed = now.elapsed();
    println!("result = {part2_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn parse(contents: String) -> Input {
    contents.lines().enumerate()
        .map(|(id, line)| {
            let mut tup = line.split(',');
            let x = tup.next().unwrap().parse().unwrap();
            let y = tup.next().unwrap().parse().unwrap();
            let z = tup.next().unwrap().parse().unwrap();

            Node { id, x, y, z, component: None }
        })
        .collect()
}

fn part1(input: Input) -> usize {
    solve_part1(input, 1000, 3)
}

fn solve_part1(mut input: Input, edge_target: usize, circuits: usize) -> usize {
    let mut edges = create_edges(&input);

    // println!("edges:\n{:#?}", edges.clone().into_sorted_vec().iter().rev());

    let mut components = Vec::new();
    let mut edge_count = 0;
    while let Some(edge) = edges.pop() {
        let Edge { dis: _, id1, id2 } = edge.0;
        // println!("curr edge: {edge:#?}");

        merge_comp(&mut input, id1, id2, &mut components);

        edge_count += 1;
        if edge_count == edge_target {
            break;
        }
    }

    components.sort_by_key(|a| Reverse(a.len()));

    components.iter().take(circuits).map(|a| a.len()).reduce(|acc, len| acc * len).unwrap_or(1)
}

fn part2(mut input: Input) -> i64 {
    let mut edges = create_edges(&input);

    // println!("edges:\n{:#?}", edges.clone().into_sorted_vec().iter().rev());

    let mut components = Vec::new();
    while let Some(edge) = edges.pop() {
        let Edge { dis: _, id1, id2 } = edge.0;
        // println!("curr edge: {edge:#?}");

        merge_comp(&mut input, id1, id2, &mut components);

        if components[0].len() == input.len() {
            return input[id1].x * input[id2].x;
        }
    }

    -1
}

fn create_edges(input: &Input) -> BinaryHeap<Reverse<Edge>> {
    let mut edges = BinaryHeap::new();

    for node in input.iter() {
        let mut other_id = node.id + 1;
        while other_id < input.len() {
            let other = &input[other_id];
            edges.push(Reverse(Edge { id1: node.id, id2: other_id, dis: node.euclidean_distance(other) }));

            other_id += 1;
        }
    }

    edges
}

fn merge_comp(
    input: &mut Input,
    id1: usize,
    id2: usize,
    components: &mut Vec<HashSet<usize>>,
) {
    match (input[id1].component, input[id2].component) {
        (None, None) => {
            // println!("No components. Create a new one! (please)");
            let component_count = components.len();
            input[id1].component = Some(component_count);
            input[id2].component = Some(component_count);

            // println!("New component. Id = {component_count}");
            let mut set = HashSet::new();
            set.insert(id1);
            set.insert(id2);
            components.push(set);
        },
        (Some(c1), None) => {
            // println!("1 component (id: {c1}). Adding node {id2}");
            input[id2].component = Some(c1);
            components[c1].insert(id2);
        },
        (None, Some(c2)) => {
            // println!("1 component (id: {c2}). Adding node {id1}");
            input[id1].component = Some(c2);
            components[c2].insert(id1);
        },
        (Some(c1), Some(c2)) => {
            if c1 != c2 {
                // println!("2 components! merging {c1} and {c2}");
                // fix c2 components
                for &item_id in components[c2].iter() {
                    input[item_id].component = Some(c1);
                }
                components[c1] = components[c1].union(&components[c2]).copied().collect();
                components.remove(c2);

                // fix the next components
                for (i,component) in components.iter().enumerate().skip(c2) {
                    for &item_id in component.iter() {
                        input[item_id].component = Some(i);
                    }
                }
            } else {
                // println!("Redundant edge! Both in component {c1}");
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let input = parse(contents.to_string());

        let res = solve_part1(input, 10, 3);

        assert_eq!(res, 40);
    }

    #[test]
    fn test_part2() {
        let contents = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let input = parse(contents.to_string());

        let res = part2(input);

        assert_eq!(res, 25272);
    }
}
