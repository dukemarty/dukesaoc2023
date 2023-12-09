use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(7, "Camel Cards");

    // let content = io_utils::read_lines("testdata1.txt");
    let content = io_utils::read_lines("puzzle.txt");
    let hands: Vec<model::Hand> = content.iter().map(|l| model::Hand::parse(l)).collect();
    // println!("Hands: {:?}", hands);

    part1(&hands);
}

fn part1(hands: &Vec<model::Hand>) {
    aoc::print_part_header(1, "Sum of all winnings");

    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| b.cmp_part1(a));
    // println!("Sorted hands: {:?}", sorted_hands);

    let mut sum = 0;
    for i in 0..sorted_hands.len() {
        sum += (i as i32 + 1) * sorted_hands[i].bid;
    }

    println!("Winnings sum: {}", sum);
}
