use aoc_utils::{aoc, data, io_utils};

mod model;

fn main() {
    aoc::print_day_header(18, "Lavaduct Lagoon");

    // let lines = io_utils::read_lines("testdata1.txt");
    let lines = io_utils::read_lines("puzzle.txt");
    let dig_cmds = model::DigPlan::create(lines.iter().map(|l| model::DigCmd::parse(l)).collect());
    // println!("{:?}", dig_cmds);

    part1(&dig_cmds);
}

fn part1(dig_cmds: &model::DigPlan) {
    aoc::print_part_header(1, "Lagoon size");

    let height = dig_cmds.bounds.y_min.abs() + dig_cmds.bounds.y_max.abs() + 6;
    let width = dig_cmds.bounds.x_min.abs() + dig_cmds.bounds.x_max.abs() + 6;
    let mut grid = Vec::new();
    for _ in 0..height {
        let new_row = ".".repeat(width as usize).chars().collect::<Vec<char>>();
        grid.push(new_row);
    }

    let mut pos = data::MapPos {
        r: dig_cmds.bounds.y_min.abs() as usize + 3,
        c: dig_cmds.bounds.x_min.abs() as usize + 3,
    };
    dig_cmds.draw(&mut grid, &mut pos, '#');
    model::fill_trench(&mut grid, '#', '*');
    // println!("{}", data_utils::charmap_to_string(&grid));

    let res = model::count_trench(&grid, '#', '*');

    println!("Lagoon size: {}", res);
}
