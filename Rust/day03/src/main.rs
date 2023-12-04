use aoc_utils::{aoc, io_utils};

fn main() {
    aoc::print_day_header(3, "Gear Ratios");

    // let raw_schematics = io_utils::read_lines("testdata1.txt");
    let raw_schematics = io_utils::read_lines("puzzle.txt");
    let extended_schematics = extend_schematic(&raw_schematics);

    part1(&extended_schematics);

    part2(&extended_schematics);
}

fn part1(schem: &Vec<Vec<char>>) {
    aoc::print_part_header(1, "Part number sum");

    let mut sum = 0;

    let mut num = Vec::new();
    for r in 1..schem[0].len() {
        num.clear();
        let mut in_num = false;
        let mut found_adjacent_sym = false;
        for (c, elem) in schem[r].iter().enumerate() {
            if in_num {
                if elem.is_ascii_digit() {
                    num.push(elem);
                    if is_symbol(schem[r - 1][c]) || is_symbol(schem[r + 1][c]) {
                        found_adjacent_sym = true;
                    }
                } else {
                    if is_symbol(schem[r - 1][c]) || is_symbol(schem[r + 1][c]) {
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
                    if is_symbol(schem[r - 1][c]) || is_symbol(schem[r + 1][c]) {
                        found_adjacent_sym = true;
                    }
                    if is_symbol(schem[r - 1][c - 1]) || is_symbol(schem[r + 1][c - 1]) {
                        found_adjacent_sym = true;
                    }
                    if is_symbol(schem[r][c - 1]) {
                        found_adjacent_sym = true;
                    }
                }
            }
        }
    }

    println!("Sum: {}", sum);
}

fn part2(schem: &Vec<Vec<char>>) {
    aoc::print_part_header(1, "Sum of gear ratios");

    let mut sum = 0;

    for r in 1..schem[0].len() {
        // num.clear();
        // let mut in_num = false;
        // let mut found_adjacent_sym = false;
        for (c, elem) in schem[r].iter().enumerate() {
            if *elem != '*' {
                continue;
            }

            let numbers = find_numbers_around(schem, r, c);
            if numbers.len() == 2 {
                let ratio = numbers[0] * numbers[1];
                sum += ratio;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn find_numbers_around(schem: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<u32> {
    let mut res = Vec::new();

    if schem[r - 1][c].is_ascii_digit() {
        res.push(grow_number(schem, r - 1, c));
    } else {
        if schem[r - 1][c - 1].is_ascii_digit() {
            res.push(grow_number(schem, r - 1, c - 1));
        }
        if schem[r - 1][c + 1].is_ascii_digit() {
            res.push(grow_number(schem, r - 1, c + 1));
        }
    }
    if schem[r + 1][c].is_ascii_digit() {
        res.push(grow_number(schem, r + 1, c));
    } else {
        if schem[r + 1][c - 1].is_ascii_digit() {
            res.push(grow_number(schem, r + 1, c - 1));
        }
        if schem[r + 1][c + 1].is_ascii_digit() {
            res.push(grow_number(schem, r + 1, c + 1));
        }
    }

    if schem[r][c - 1].is_ascii_digit() {
        res.push(grow_number(schem, r, c - 1));
    }
    if schem[r][c + 1].is_ascii_digit() {
        res.push(grow_number(schem, r, c + 1));
    }

    res
}

fn grow_number(schem: &Vec<Vec<char>>, r: usize, c: usize) -> u32 {
    let mut start = c;
    while schem[r][start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut res = 0;
    let mut i = start;
    while schem[r][i].is_ascii_digit() {
        res = 10 * res + schem[r][i].to_digit(10).unwrap();
        i += 1;
    }

    res
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
