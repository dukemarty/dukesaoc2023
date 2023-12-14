use aoc_utils::{aoc, data_utils, io_utils};
use model::ReflectionLine;

mod model;

fn main() {
    aoc::print_day_header(13, "Point of Incidence");

    // let blocks = io_utils::read_emptyline_separated_blocks("testdata1.txt");
    let blocks = io_utils::read_emptyline_separated_blocks("puzzle.txt");
    let blocks = blocks
        .iter()
        .map(|b| data_utils::lines_to_char_map(b))
        .collect::<Vec<Vec<Vec<char>>>>();

    part1(&blocks);

    part2(&blocks);
}

fn part1(blocks: &Vec<Vec<Vec<char>>>) {
    aoc::print_part_header(1, "Summary of pattern notes");

    let mut sum_cols_left = 0;
    let mut sum_rows_above = 0;

    for (_bi, b) in blocks.iter().enumerate() {
        let refl_line = model::find_reflection_line_part1(b);
        // println!("Found for block {}: {:?}", _bi, refl_line);
        match refl_line {
            ReflectionLine::None => {}
            ReflectionLine::Vertical(col) => {
                sum_cols_left += col + 1;
            }
            ReflectionLine::Horizontal(row) => {
                sum_rows_above += row + 1;
            }
        };
    }

    println!("Number of columns left: {}", sum_cols_left);
    println!("Number of rows above: {}", sum_rows_above);

    let summary = 100 * sum_rows_above + sum_cols_left;
    println!("Summary: {}", summary);
}

fn part2(blocks: &Vec<Vec<Vec<char>>>){
    aoc::print_part_header(2, "Summary of smudged pattern notes");

    let mut sum_cols_left = 0;
    let mut sum_rows_above = 0;

    for (_bi, b) in blocks.iter().enumerate() {
        let refl_line = model::find_reflection_line_part2(b);
        // println!("Found for block {}: {:?}", _bi, refl_line);
        match refl_line {
            ReflectionLine::None => {}
            ReflectionLine::Vertical(col) => {
                sum_cols_left += col + 1;
            }
            ReflectionLine::Horizontal(row) => {
                sum_rows_above += row + 1;
            }
        };
    }

    println!("Number of columns left: {}", sum_cols_left);
    println!("Number of rows above: {}", sum_rows_above);

    let summary = 100 * sum_rows_above + sum_cols_left;
    println!("Summary: {}", summary);
}
