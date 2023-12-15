use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(15, "Lens Library");

    // let line = io_utils::read_single_line("testdata1.txt");
    let line = io_utils::read_single_line("puzzle.txt");

    part1(&line);
}

fn part1(line: &String){
    aoc::print_part_header(1, "Hash of line");

    let steps: Vec<&str> = line.split_terminator(',').collect();
    let hash: u32 = steps.iter().map(|s| model::run_hash_algorithm(*s)).sum();

    println!("Hash: {}", hash);
}