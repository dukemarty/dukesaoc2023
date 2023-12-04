use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(4, "Scratchcards");

    // let card_list = io_utils::read_lines("testdata1.txt");
    let card_list = io_utils::read_lines("puzzle.txt");
    let cards = card_list
        .iter()
        .map(|l| model::Card::parse(l))
        .collect::<Vec<model::Card>>();
    // let card_list = io_utils::read_lines("puzzle.txt");

    part1(&cards);

    part2(&cards);
}

fn part1(cards: &Vec<model::Card>) {
    aoc::print_part_header(1, "Points sum");

    let base: u32 = 2;
    let mut sum = 0;

    for c in cards {
        let hits = c.count_hits();
        if hits > 0 {
            sum += base.pow(hits - 1);
        }
    }

    println!("Sum: {}", sum);
}

fn part2(cards: &Vec<model::Card>) {
    aoc::print_part_header(2, "Card count");

    let mut count = vec![1; cards.len()];
    for i in 0..cards.len() {
        let hits = cards[i].count_hits();
        if hits > 0 {
            for j in i + 1..i + 1 + usize::try_from(hits).unwrap() {
                count[j] += count[i];
            }
        }
    }

    println!(
        "Final count: {}",
        count.iter().fold(0, |acc, num| acc + num)
    );
}
