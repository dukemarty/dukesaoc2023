use aoc_utils::{aoc, data, data_utils, io_utils};

mod model;

fn main() {
    aoc::print_day_header(10, "Pipe Maze");

    // let mut pipe_map = io_utils::read_2d_char_map("testdata1.txt");
    // let mut pipe_map = io_utils::read_2d_char_map("testdata2.txt");
    let mut pipe_map = io_utils::read_2d_char_map("puzzle.txt");
    let start_pos = data_utils::find_sym_in_charmap('S', &pipe_map).unwrap();
    println!("Start pos: {:?}", start_pos);

    part1(&pipe_map, &start_pos);

    part2(&mut pipe_map, &start_pos);
}

fn part1(pipes: &Vec<Vec<char>>, start: &data::MapPos) {
    aoc::print_part_header(1, "Longest distance in pipe loop");

    let start_dirs = model::find_connections(start, pipes);
    println!("Start dirs: {:?}", start_dirs);
    let mut pipe_parts = Vec::new();
    let first_step = start_dirs[0].move_pos(start, pipes).unwrap();
    pipe_parts.push(first_step.clone());
    let mut next_step = first_step;
    let mut from_dir = start_dirs[0].clone();
    while next_step.sym(pipes) != 'S' {
        // println!("Loop: ({:?}) from {:?}", next_step, from_dir);
        (next_step, from_dir) = model::step(pipes, next_step, from_dir);
        pipe_parts.push(next_step.clone());
    }

    // println!("Found pipe: {:?}", pipe_parts);
    let dist = pipe_parts.len() / 2;

    println!("Distance: {}", dist);
}

fn part2(pipes: &mut Vec<Vec<char>>, start: &data::MapPos) {
    aoc::print_part_header(2, "Enclosed area size");

    let mut area_map = model::AreaMap::create(pipes, &start);
    // println!("-----------------------------------------------------------------------");
    // println!("{}", area_map);
    // println!("-----------------------------------------------------------------------");
    let start_connections = model::find_connections(start, pipes);
    if start_connections.contains(&model::Dir::Down) {
        if start_connections.contains(&model::Dir::Up) {
            pipes[start.r][start.c] = '|';
        } else if start_connections.contains(&model::Dir::Left) {
            pipes[start.r][start.c] = '7';
        } else {
            pipes[start.r][start.c] = 'F';
        }
    } else if start_connections.contains(&model::Dir::Up) {
        if start_connections.contains(&model::Dir::Left) {
            pipes[start.r][start.c] = 'J';
        } else {
            pipes[start.r][start.c] = 'L';
        }
    } else {
        pipes[start.r][start.c] = '-';
    }
    // area_map.print_relevant_pipes(pipes);

    area_map.mark_inner_area(pipes);
    // println!("-----------------------------------------------------------------------");
    // println!("{}", area_map);

    let area = area_map.inner_count;

    println!("Area: {}", area);
}
