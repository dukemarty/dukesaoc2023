use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(6, "Wait For It");

    // let contents = io_utils::read_lines("testdata1.txt");
    let contents = io_utils::read_lines("puzzle.txt");
    let races = model::Race::parse_many(&contents[0], &contents[1]);

    part1(&races);
}

fn part1(races: &Vec<model::Race>) {
    aoc::print_part_header(1, "Product of ways to win");

    let product = races
        .iter()
        .map(|r| r.count_ways_to_win())
        .fold(1, |acc, num| acc * num);

    println!("Product: {}", product);
}
