use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(7, "Camel Cards");

    // let content = io_utils::read_lines("testdata1.txt");
    // let content = io_utils::read_lines("testdata2.txt");
    let content = io_utils::read_lines("puzzle.txt");
    // println!("Hands: {:?}", hands);

    part1(&content);

    part2(&content);
}

fn part1(content: &Vec<String>) {
    aoc::print_part_header(1, "Sum of all winnings");

    let hands: Vec<model::Hand> = content.iter().map(|l| model::Hand::parse_part1(l)).collect();
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| b.cmp_part1(a));
    // println!("Sorted hands: {:?}", sorted_hands);

    let mut sum = 0;
    for i in 0..sorted_hands.len() {
        sum += (i as i32 + 1) * sorted_hands[i].bid;
    }

    println!("Winnings sum: {}", sum);
}

fn part2(content: &Vec<String>) {
    aoc::print_part_header(2, "Sum of all winnings with joker");

    let hands: Vec<model::Hand> = content.iter().map(|l| model::Hand::parse_part2(l)).collect();
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| b.cmp_part2(a));
    // println!("Sorted hands: {:?}", sorted_hands);

    let mut sum = 0;
    for i in 0..sorted_hands.len() {
        sum += (i as i32 + 1) * sorted_hands[i].bid;
    }

    println!("Winnings sum: {}", sum);
}
