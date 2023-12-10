use aoc_utils::{aoc, io_utils};

fn main() {
    aoc::print_day_header(9, "Mirage Maintenance");

    // let history = io_utils::read_lines("testdata1.txt");
    let history = io_utils::read_lines("puzzle.txt");

    part1(&history);

    part2(&history);
}

fn part1(history: &Vec<String>) {
    aoc::print_part_header(1, "Sum of next values");

    let mut sum = 0;

    for l in history.iter() {
        let nums = l
            .split_whitespace()
            .map(|t| t.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        sum += determine_follow_value(nums);
    }

    println!("Sum: {}", sum);
}

fn part2(history: &Vec<String>) {
    aoc::print_part_header(2, "Sum of previous values");

    let mut sum = 0;

    for l in history.iter() {
        let nums = l
            .split_whitespace()
            .map(|t| t.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        sum += determine_previous_value(nums);
    }

    println!("Sum: {}", sum);
}


fn determine_follow_value(history: Vec<i32>) -> i32 {
    let mut diffs = Vec::new();

    diffs.push(history);
    let mut last_index = diffs.len() - 1;
    while !diffs[last_index].iter().all(|n| *n == 0) {
        let mut next = Vec::new();
        for i in 0..diffs[last_index].len() - 1 {
            next.push(diffs[last_index][i + 1] - diffs[last_index][i])
        }
        diffs.push(next);
        last_index += 1;
    }
    // println!("diffs: {:?}", diffs);

    let mut pred = 0;
    for i in (0..diffs.len()).rev() {
        pred = *diffs[i].last().unwrap() + pred;
    }
    // println!("Prediction: {}", pred);

    pred
}

fn determine_previous_value(history: Vec<i32>) -> i32 {
    let mut diffs = Vec::new();

    diffs.push(history);
    let mut last_index = diffs.len() - 1;
    while !diffs[last_index].iter().all(|n| *n == 0) {
        let mut next = Vec::new();
        for i in 0..diffs[last_index].len() - 1 {
            next.push(diffs[last_index][i + 1] - diffs[last_index][i])
        }
        diffs.push(next);
        last_index += 1;
    }
    // println!("diffs: {:?}", diffs);

    let mut pred = 0;
    for i in (0..diffs.len()).rev() {
        pred = diffs[i][0] - pred;
    }
    // println!("Prediction: {}", pred);

    pred
}
