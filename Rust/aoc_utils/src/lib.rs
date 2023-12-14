pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod aoc {

    pub fn print_day_header(day: i32, title: &str) {
        println!("");
        println!("--- Day {:0>2}: {} ---", day, title);
        println!("{}", "=".repeat(16 + title.len()));
        println!("");
    }

    pub fn print_part_header(id: i32, title: &str) {
        println!("\nPart {}: {}", id, title);
        println!("{}", "-".repeat(8 + title.len()));
    }
}

pub mod data {

    #[derive(Debug, Clone)]
    pub struct MapPos {
        pub r: usize,
        pub c: usize,
    }

    impl MapPos {
        pub fn sym(&self, charmap: &Vec<Vec<char>>) -> char {
            charmap[self.r][self.c]
        }
    }
}

pub mod io_utils {

    use std::{fs, str};

    pub fn read_lines(filename: &str) -> Vec<String> {
        let content = fs::read_to_string(filename).expect("Should have been able to read the file");
        let res: Vec<String> = content.lines().map(ToOwned::to_owned).collect();
        res
    }

    pub fn read_emptyline_separated_blocks(filename: &str) -> Vec<Vec<String>> {
        // let content = fs::read_to_string(filename).expect("Should have been able to read the file");
        // let res: Vec<String> = content.split("\n\n").map(ToOwned::to_owned).collect();
        // res

        let lines = read_lines(filename);
        let mut res = Vec::new();
        let mut b = Vec::new();
        for l in lines {
            if l.is_empty() {
                if b.len() > 0 {
                    res.push(b);
                    b = Vec::new();
                }
            } else {
                b.push(l);
            }
        }

        if b.len() > 0 {
            res.push(b);
        }

        return res;
    }

    pub fn read_2d_char_map(filename: &str) -> Vec<Vec<char>> {
        let lines = read_lines(filename);
        lines
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }
}

pub mod data_utils {
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
}
