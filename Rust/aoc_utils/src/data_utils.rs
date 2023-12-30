use crate::data::MapPos;

pub fn parse_i32_list_ws(list: &str) -> Vec<i32> {
    let res = list
        .split_whitespace()
        .map(|t| t.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    res
}

pub fn parse_i32_list_ws_with_title(list: &String) -> (&str, Vec<i32>) {
    let parts = list.split(":").collect::<Vec<&str>>();
    let title = parts[0].trim();
    let numbers = parse_i32_list_ws(parts[1].trim());

    (title, numbers)
}

pub fn find_sym_in_charmap(sym: char, charmap: &Vec<Vec<char>>) -> Option<super::data::MapPos> {
    for (r, line) in charmap.iter().enumerate() {
        for (c, s) in line.iter().enumerate() {
            if *s == sym {
                return Some(MapPos { r, c });
            }
        }
    }

    None
}

pub fn lines_to_char_map(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn are_columns_equal(charmap: &Vec<Vec<char>>, col1: usize, col2: usize) -> bool {
    for i in 0..charmap.len() {
        if charmap[i][col1] != charmap[i][col2] {
            return false;
        }
    }

    true
}

pub fn are_rows_equal(charmap: &Vec<Vec<char>>, row1: usize, row2: usize) -> bool {
    for i in 0..charmap[0].len() {
        if charmap[row1][i] != charmap[row2][i] {
            return false;
        }
    }

    true
}

pub fn count_column_differences(charmap: &Vec<Vec<char>>, col1: usize, col2: usize) -> i32 {
    let mut res = 0;
    for i in 0..charmap.len() {
        if charmap[i][col1] != charmap[i][col2] {
            res += 1;
        }
    }

    res
}

pub fn count_row_differences(charmap: &Vec<Vec<char>>, row1: usize, row2: usize) -> i32 {
    let mut res = 0;
    for i in 0..charmap[0].len() {
        if charmap[row1][i] != charmap[row2][i] {
            res += 1;
        }
    }

    res
}

pub fn charmap_to_string(charmap: &Vec<Vec<char>>) -> String {
    charmap.iter()
    .map(|r| r.iter().collect::<String>())
    .collect::<Vec<String>>()
    .join("\n")
}

