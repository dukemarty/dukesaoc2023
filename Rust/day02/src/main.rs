use aoc_utils::{self, aoc, io_utils};
use model::Game;

mod model;

const BAG1_RED: u32 = 12;
const BAG1_GREEN: u32 = 13;
const BAG1_BLUE: u32 = 14;

fn main() {
    aoc::print_day_header(2, "Cube Conundrum");

    // let contents = io_utils::read_lines("testdata1.txt");
    // let contents = io_utils::read_lines("testdata2.txt");
    let contents = io_utils::read_lines("puzzle.txt");
    let games = parse_games(contents);

    part1(&games);

    part2(&games);
}

fn parse_games(contents: Vec<String>) -> Vec<model::Game> {
    let mut res = Vec::new();
    for line in contents {
        let next = model::Game::parse(&line);
        res.push(next);
    }

    res
}

fn part1(games: &Vec<Game>) {
    aoc::print_part_header(1, "Possible draws sum");

    let mut sum = 0;

    for g in games {
        // eprintln!("Game {:3}: {} draws", g.id, g.draws.len());
        if g.could_be_bag(BAG1_RED, BAG1_GREEN, BAG1_BLUE) {
            sum += g.id;
        }
    }

    println!("Sum: {}", sum);
}

fn part2(games: &Vec<Game>) {
    aoc::print_part_header(2, "Sum of cubed min-bags");

    let mut sum = 0;

    for g in games {
        let bag = g.calc_min_bag();
        let cubed = bag.red * bag.green * bag.blue;
        sum += cubed;
    }

    println!("Sum: {}", sum);
}
