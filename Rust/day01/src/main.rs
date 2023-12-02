use aoc_utils::{self, aoc, io_utils};

fn main() {
    aoc::print_day_header(1, "Trebuchet?!");

    let contents = io_utils::read_lines("puzzle.txt");

    part1(&contents);

    println!("");

    part2(&contents);
}

fn part1(contents: &Vec<String>) {
    aoc::print_part_header(1, "Calibration value from digits");

    let mut sum = 0;
    for line in contents {
        let next = extract_calibration_value_part1(&line);
        sum += next;
    }

    println!("Sum: {}", sum);
}

fn part2(contents: &Vec<String>) {
    aoc::print_part_header(2, "Calibration value with words");

    let mut sum = 0;
    for line in contents {
        let next = extract_calibration_value_part2(&line);
        sum += next;
    }

    println!("Sum: {}", sum);
}

fn extract_calibration_value_part1(line: &str) -> i32 {
    let mut first_found = false;
    let mut last_found = false;
    let mut first_digit = ' ';
    let mut last_digit = ' ';
    let chars: Vec<_> = line.chars().collect();
    for i in 0..chars.len() {
        if !first_found {
            if chars[i].is_numeric() {
                first_digit = chars[i];
                first_found = true;
            }
        }
        if !last_found {
            if chars[chars.len() - i - 1].is_numeric() {
                last_digit = chars[chars.len() - i - 1];
                last_found = true;
            }
        }
    }

    let mut s = String::from("");
    s.push(first_digit);
    s.push(last_digit);

    s.parse::<i32>().unwrap()
}

fn extract_calibration_value_part2(line: &str) -> i32 {
    let mut first_found = false;
    let mut last_found = false;
    let mut first_digit = ' ';
    let mut last_digit = ' ';
    let chars: Vec<_> = line.chars().collect();
    for i in 0..chars.len() {
        if !first_found {
            if chars[i].is_numeric() {
                first_digit = chars[i];
                first_found = true;
                let word_number = find_earlier_word(line, i);
                if word_number.is_some() {
                    // eprintln!("Found word number: {}", word_number.unwrap());
                    first_digit = word_number.unwrap();
                }
            }
        }
        if !last_found {
            let check_pos_from_back = chars.len() - i - 1;
            if chars[check_pos_from_back].is_numeric() {
                last_digit = chars[check_pos_from_back];
                last_found = true;
                let word_number = find_later_word(line, check_pos_from_back);
                if word_number.is_some() {
                    // eprintln!("Found word number: {}", word_number.unwrap());
                    last_digit = word_number.unwrap();
                }
            }
        }
    }

    if !first_found {
        let word_number = find_earlier_word(line, line.len());
        if word_number.is_some() {
            // eprintln!("Found word number: {}", word_number.unwrap());
            first_digit = word_number.unwrap();
        }
    }
    if !last_found {
        let word_number = find_later_word(line, 0);
        if word_number.is_some() {
            // eprintln!("Found word number: {}", word_number.unwrap());
            last_digit = word_number.unwrap();
        }
    }

    let mut s = String::from("");
    s.push(first_digit);
    s.push(last_digit);

    // eprintln!("Combined digits: {}", s);

    s.parse::<i32>().unwrap()
}

fn find_earlier_word(line: &str, pos: usize) -> Option<char> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let values = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // eprintln!("Check '{}' for words before pos {}...", line, pos);

    let mut found_pos = line.len() + 1;
    let mut found_char = '0';
    for i in 0..words.len() {
        let found = line.find(words[i]);
        if found.is_some() {
            // eprintln!("Found word: {} @ pos {}", values[i], found.unwrap());
            if found.unwrap() < found_pos {
                found_pos = found.unwrap();
                found_char = values[i];
            }
        }
    }

    if found_pos < pos {
        return Some(found_char);
    } else {
        return None;
    }
}

fn find_later_word(line: &str, pos: usize) -> Option<char> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let values = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // eprintln!("Check '{}' for words after pos {}...", line, pos);

    let mut found_pos = 0;
    let mut found_char = '0';
    for i in 0..words.len() {
        let found = line.rfind(words[i]);
        if found.is_some() {
            // eprintln!("Found word: {} @ pos {}", values[i], found.unwrap());
            if found.unwrap() > found_pos {
                found_pos = found.unwrap();
                found_char = values[i];
            }
        }
    }

    if found_pos > pos {
        return Some(found_char);
    } else {
        return None;
    }
}

// match chars[i].to_digit(10){
//     Some(d) => d,
//     None => 0
// };
