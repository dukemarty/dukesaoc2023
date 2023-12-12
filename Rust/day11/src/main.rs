use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(11, "Cosmic Expansion");

    // let star_map = io_utils::read_2d_char_map("testdata1.txt");
    let star_map = io_utils::read_2d_char_map("puzzle.txt");

    part1(&star_map);
}

fn part1(star_map: &Vec<Vec<char>>) {
    aoc::print_part_header(1, "Sum dists in expanded map");

    let stars = model::find_all_stars(star_map);
    let (empty_rows, empty_cols) =
        model::determine_empty_rows_and_columns(star_map.len(), star_map[0].len(), &stars);
    println!(
        "Empty rows: {:?},   Empty cols: {:?}",
        empty_rows, empty_cols
    );
    let mut dist_sum = 0;
    for i in 0..stars.len() - 1 {
        for j in i..stars.len() {
            dist_sum += model::calc_stars_dist(&stars[i], &stars[j], &empty_rows, &empty_cols);
        }
    }

    println!("Sum of distances: {}", dist_sum);
}
