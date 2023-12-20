use std::collections::HashMap;

use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(19, "Parabolic Reflector Dish");

    // let blocks = io_utils::read_emptyline_separated_blocks("testdata1.txt");
    let blocks = io_utils::read_emptyline_separated_blocks("puzzle.txt");

    let parts: Vec<model::Part> = blocks[1]
        .iter()
        .map(|l| model::Part::parse(l).unwrap())
        .collect();
    println!("Parts: {:?}", parts);
    let workflows: Vec<model::Workflow> = blocks[0]
        .iter()
        .map(|l| model::Workflow::parse(l).unwrap())
        .collect();
    let workflows: HashMap<String, &model::Workflow> =
        workflows.iter().map(|wf| (wf.name.clone(), wf)).collect();
    println!("Workflows: {:?}", workflows);

    part1(&parts, &workflows);
}

fn part1(parts: &Vec<model::Part>, workflows: &HashMap<String, &model::Workflow>) {
    aoc::print_part_header(1, "Rating sum of accepted parts");

    let mut sum = 0;

    for p in parts.iter() {
        let mut target = model::Target::OtherRule(String::from("in"));
        while true {
            let mut flow =             match target {
                model::Target::ACCEPT => panic!("ERROR: CAN only be OtherRule here!!!"),
                model::Target::REJECT => panic!("ERROR: CAN only be OtherRule here!!!"),
                model::Target::OtherRule(ref f) => f.clone(),
            };
            let next = workflows[&flow].process(p);
            match next {
                model::Target::ACCEPT => {
                    println!("Accepted {:?}", p);
                    sum += p.get_full_rating();
                    break;
                }
                model::Target::REJECT => {
                    println!("Rejected {:?}", p);
                    break;
                }
                model::Target::OtherRule(ref f) => target = model::Target::OtherRule(f.clone()),
            };
        }
    }

    println!("Ratings sum: {}", sum);
}
