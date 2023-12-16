use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(15, "Lens Library");

    // let line = io_utils::read_single_line("testdata1.txt");
    let line = io_utils::read_single_line("puzzle.txt");
    let steps: Vec<&str> = line.split_terminator(',').collect();

    part1(&steps);

    part2(&steps);
}

fn part1(steps: &Vec<&str>) {
    aoc::print_part_header(1, "Hash of line");

    let hash: u32 = steps.iter().map(|s| model::run_hash_algorithm(*s)).sum();

    println!("Hash: {}", hash);
}

fn part2(steps: &Vec<&str>) {
    aoc::print_part_header(2, "Resulting focusing power");

    let mut boxes = model::LensBoxes::create();
    for s in steps {
        let instr = model::Step::parse(*s);
        boxes.perform_step(&instr);
    }
    // println!("{:?}", boxes);

    let res = boxes.calc_focusing_power();

    println!("Focusing power: {}", res);
}
