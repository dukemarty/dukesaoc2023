use aoc_utils::{aoc, io_utils};
use std::collections::HashMap;

mod model;

fn main() {
    aoc::print_day_header(8, "Haunted Wasteland");

    // let blocks = io_utils::read_emptyline_separated_blocks("testdata1.txt");
    // let blocks = io_utils::read_emptyline_separated_blocks("testdata2.txt");
    let blocks = io_utils::read_emptyline_separated_blocks("puzzle.txt");
    let instructions = blocks[0][0].chars().collect::<Vec<char>>();
    let nodes = blocks[1]
        .iter()
        .map(|l| model::Node::parse(l).unwrap())
        .collect::<Vec<model::Node>>();
    let node_map: HashMap<_, _> = nodes.iter().map(|n| (n.name.clone(), n)).collect();
    println!("Instructions: {:?}", instructions);
    println!("Nodes: {:?}", node_map);

    part1(&instructions, &node_map);
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
