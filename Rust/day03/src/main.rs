use aoc_utils::{aoc, io_utils};

fn main() {
    aoc::print_day_header(3, "Gear Ratios");

    // let schematics = io_utils::read_lines("testdata1.txt");
    let schematics = io_utils::read_lines("puzzle.txt");

    part1(&schematics);
}

fn part1(schematics: &Vec<String>) {
    aoc::print_part_header(1, "Part number sum");

    let ext = extend_schematic(schematics);
    let mut sum = 0;

    let mut num = Vec::new();
    for r in 1..ext[0].len() {
        num.clear();
        let mut in_num = false;
        let mut found_adjacent_sym = false;
        for (c, elem) in ext[r].iter().enumerate() {
            if in_num {
                if elem.is_ascii_digit() {
                    num.push(elem);
                    if is_symbol(ext[r - 1][c]) || is_symbol(ext[r + 1][c]) {
                        found_adjacent_sym = true;
                    }
                } else {
                    if is_symbol(ext[r - 1][c]) || is_symbol(ext[r + 1][c]) {
                        found_adjacent_sym = true;
                    }
                    if is_symbol(*elem) {
                        found_adjacent_sym = true;
                    }
                    if found_adjacent_sym {
                        let num_str = num.clone().into_iter().collect::<String>();
                        // println!("Found part num: {}", numStr);
                        sum += num_str.parse::<u32>().unwrap();
                    } else {
                        // println!(
                        //     "Apparently, {} is not a part num!",
                        //     num.clone().into_iter().collect::<String>()
                        // );
                    }
                    num.clear();
                    in_num = false;
                    found_adjacent_sym = false;
                }
            } else {
                if elem.is_ascii_digit() {
                    num.push(elem);
                    in_num = true;
                    if is_symbol(ext[r - 1][c]) || is_symbol(ext[r + 1][c]) {
                        found_adjacent_sym = true;
                    }
                    if is_symbol(ext[r - 1][c - 1]) || is_symbol(ext[r + 1][c - 1]) {
                        found_adjacent_sym = true;
                    }
                    if is_symbol(ext[r][c - 1]) {
                        found_adjacent_sym = true;
                    }
                }
            }
        }
    }

    println!("Sum: {}", sum);
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn extend_schematic(s: &Vec<String>) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    res.push(".".repeat(s.len() + 2).chars().collect());
    for l in s {
        res.push(format!(".{}.", l).chars().collect());
    }
    res.push(".".repeat(s.len() + 2).chars().collect());

    res
}
