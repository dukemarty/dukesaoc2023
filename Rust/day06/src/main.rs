use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(6, "Wait For It");

    // let content = io_utils::read_lines("testdata1.txt");
    let content = io_utils::read_lines("puzzle.txt");

    part1(&content);

    part2(&content);
}

fn part1(content: &Vec<String>) {
    aoc::print_part_header(1, "Product of ways to win");

    let races = model::Race::parse_many(&content[0], &content[1]);
    let product1 = races
        .iter()
        .map(|r| r.count_ways_to_win1())
        .fold(1, |acc, num| acc * num);
    // println!("--------------------");
    let product2 = races
        .iter()
        .map(|r| r.count_ways_to_win2())
        .fold(1, |acc, num| acc * num);

    println!("Product 1: {}", product1);
    println!("Product 2: {}", product2);
}

fn part2(content: &Vec<String>){
    aoc::print_part_header(2, "Way to win in long race");

    let race=model::Race::parse_single_race(&content[0], &content[1]);
    println!("Found single race: {:?}", race);

    let res = race.count_ways_to_win2();

    println!("Number of winning moves: {}", res);
}
