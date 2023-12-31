use aoc_utils::{aoc, io_utils};
use std::collections::{HashMap, HashSet};

mod model;

fn main() {
    aoc::print_day_header(8, "Haunted Wasteland");

    // let blocks = io_utils::read_emptyline_separated_blocks("testdata1.txt");
    // let blocks = io_utils::read_emptyline_separated_blocks("testdata2.txt");
    // let blocks = io_utils::read_emptyline_separated_blocks("testdata3.txt");
    let blocks = io_utils::read_emptyline_separated_blocks("puzzle.txt");
    let instructions = blocks[0][0].chars().collect::<Vec<char>>();
    let nodes = blocks[1]
        .iter()
        .map(|l| model::Node::parse(l).unwrap())
        .collect::<Vec<model::Node>>();
    let node_map: HashMap<_, _> = nodes.iter().map(|n| (n.name.clone(), n)).collect();
    let start_nodes: HashSet<_> = nodes
        .iter()
        .filter(|n| n.is_start_node())
        .map(|n| n.name.clone())
        .collect();
    let end_nodes: HashSet<_> = nodes
        .iter()
        .filter(|n| n.is_end_node())
        .map(|n| n.name.clone())
        .collect();
    // println!("Instructions: {:?}", instructions);
    // println!("Nodes: {:?}", node_map);
    println!("Start nodes: {:?}", start_nodes);
    println!("End nodes  : {:?}", end_nodes);

    part1(&instructions, &node_map);

    part2(&instructions, &node_map, &start_nodes, &end_nodes);
}

pub fn part1(instr: &Vec<char>, nodes: &HashMap<String, &model::Node>) {
    aoc::print_part_header(1, "Steps to ZZZ");

    let mut steps = 0;
    let mut curr = "AAA";
    while curr != "ZZZ" {
        for c in instr.iter() {
            curr = match c {
                'L' => &nodes[curr].left,
                'R' => &nodes[curr].right,
                _ => {
                    println!("ERROR could not step!!!");
                    curr
                }
            };
            steps += 1;
        }
    }

    println!("#Total of steps: {}", steps);
}

pub fn part2(
    instr: &Vec<char>,
    nodes: &HashMap<String, &model::Node>,
    starts: &HashSet<String>,
    ends: &HashSet<String>,
) {
    aoc::print_part_header(2, "Steps to end from all");

    let mut routes = model::MultiRoute::create(instr, nodes, starts, ends);
    println!("Multi routes:\n{:?}", routes);

    let mut congruences = Vec::new();
    for s in starts.iter() {
        let mut steps = Vec::new();
        steps.push(model::Step {
            node: s.clone(),
            count: 0,
        });
        let mut curr = s.clone();
        let mut curr_rounds = 0;
        let mut seen = HashSet::new();
        while !seen.contains(&curr) {
            seen.insert(curr.clone());
            let next = routes.rounds_till_end.get(&curr).unwrap();
            curr = next.0.clone();
            curr_rounds += next.1;
            steps.push(model::Step {
                node: curr.clone(),
                count: curr_rounds,
            });
        }

        let last_step = steps.last().unwrap();
        let n2 = last_step.count;
        let n1 = steps
            .iter()
            .filter(|s| (**s).node == last_step.node)
            .collect::<Vec<&model::Step>>()
            .first()
            .unwrap()
            .count;
        println!("Steps for {}: {:?}", s, steps);
        congruences.push(model::Congruence { a: n1, m: n2 - n1 });
    }
    println!("To solve:");
    for congr in congruences.iter() {
        println!("x = {} mod {}", congr.a, congr.m);
    }

    println!("For puzzle input apparently, simplified solution possible: least common multiple should be enough!");

    let res = lcm(congruences.iter().map(|c| (*c).m).collect()) * (instr.len() as u64);
    // let mut routes = starts.iter().map(|nn| model::Route {pos: nn.clone(), rounds:0}).collect::<Vec<Route>>();

    // let rounds = routes.run_till_common_end() as i64;
    // let steps = rounds * (instr.len() as i64);// (instr.len() as i32);

    println!("#Total of steps: {}", res);
}

fn lcm(numbers: Vec<i32>) -> u64 {
    numbers.iter().fold(1u64, |acc, elem| acc * (*elem as u64))
}
