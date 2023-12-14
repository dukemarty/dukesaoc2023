use std::cmp;

use aoc_utils::data_utils;

#[derive(Debug)]
pub enum ReflectionLine {
    None,
    Vertical(usize),
    Horizontal(usize),
}

pub fn find_reflection_line_part1(block: &Vec<Vec<char>>) -> ReflectionLine {
    let height = block.len();
    let width = block[0].len();
    'columns: for c in 0..width - 1 {
        for i in 0..cmp::min(c + 1, width - c - 1) {
            if !data_utils::are_columns_equal(block, c - i, c + 1 + i) {
                continue 'columns;
            }
        }
        return ReflectionLine::Vertical(c);
    }
    'rows: for r in 0..height - 1 {
        for i in 0..cmp::min(r + 1, height - r - 1) {
            if !data_utils::are_rows_equal(block, r - i, r + 1 + i) {
                continue 'rows;
            }
        }
        return ReflectionLine::Horizontal(r);
    }
    ReflectionLine::None
}

pub fn find_reflection_line_part2(block: &Vec<Vec<char>>) -> ReflectionLine {
    let height = block.len();
    let width = block[0].len();
    'columns: for c in 0..width - 1 {
        let mut smudges = 0;
        for i in 0..cmp::min(c + 1, width - c - 1) {
            smudges += data_utils::count_column_differences(block, c - i, c + 1 + i);
            if smudges > 1 {
                continue 'columns;
            }
        }
        if smudges == 1 {
            return ReflectionLine::Vertical(c);
        }
    }
    'rows: for r in 0..height - 1 {
        let mut smudges = 0;
        for i in 0..cmp::min(r + 1, height - r - 1) {
            smudges += data_utils::count_row_differences(block, r - i, r + 1 + i);
            if smudges > 1 {
                continue 'rows;
            }
        }
        if smudges == 1 {
            return ReflectionLine::Horizontal(r);
        }
    }
    ReflectionLine::None
}
