use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(12, "Hot Springs");

    // let damaged_records = io_utils::read_lines("testdata1.txt");
    let damaged_records = io_utils::read_lines("puzzle.txt");
    let damaged_records: Vec<model::ConditionRecord> = damaged_records
        .iter()
        .map(|r| model::ConditionRecord::parse(r))
        .collect();
    // println!("Damaged records:\n{}", damaged_records.iter().map(|cr| (*cr).to_string()).collect::<Vec<String>>().join("\n"));

    part1(&damaged_records);
}

fn part1(damaged_records: &Vec<model::ConditionRecord>) {
    aoc::print_part_header(1, "Count of possible arrangements");

    let sum: u32 = damaged_records
        .iter()
        .map(|dr| dr.count_possible_valid_arrangements())
        .sum();

    println!("Sum of counts: {}", sum);
}
