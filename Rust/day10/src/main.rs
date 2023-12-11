use std::process;

use aoc_utils::{aoc, data, data_utils, io_utils};

mod model;

fn main() {
    aoc::print_day_header(10, "Pipe Maze");

    // let pipe_map = io_utils::read_2d_char_map("testdata1.txt");
    // let pipe_map = io_utils::read_2d_char_map("testdata2.txt");
    let pipe_map = io_utils::read_2d_char_map("puzzle.txt");
    let start_pos = data_utils::find_sym_in_charmap('S', &pipe_map).unwrap();
    println!("Start pos: {:?}", start_pos);

    // let pipe_map = io_utils::read_2d_char_map("testdata2.txt");
    // let pipe_map = io_utils::read_2d_char_map("puzzle.txt");

    part1(&pipe_map, &start_pos);
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
        (next_step, from_dir) = step(pipes, next_step, from_dir);
        pipe_parts.push(next_step.clone());
    }

    // println!("Found pipe: {:?}", pipe_parts);
    let dist = pipe_parts.len() / 2;

    println!("Distance: {}", dist);
}

fn step(
    pipes: &Vec<Vec<char>>,
    pos: data::MapPos,
    origin: model::Dir,
) -> (data::MapPos, model::Dir) {
    if origin == model::Dir::None{
        process::exit(1);
    }
    // println!("Moving from pos {:?} from {:?}", pos, origin);
    match (origin, pos.sym(pipes)) {
        (model::Dir::Up, '|') => (
            model::Dir::Up.move_pos(&pos, pipes).unwrap(),
            model::Dir::Up,
        ),
        (model::Dir::Down, '|') => (
            model::Dir::Down.move_pos(&pos, pipes).unwrap(),
            model::Dir::Down,
        ),
        (model::Dir::Left, '-') => (
            model::Dir::Left.move_pos(&pos, pipes).unwrap(),
            model::Dir::Left,
        ),
        (model::Dir::Right, '-') => (
            model::Dir::Right.move_pos(&pos, pipes).unwrap(),
            model::Dir::Right,
        ),
        (model::Dir::Down, 'L') => (
            model::Dir::Right.move_pos(&pos, pipes).unwrap(),
            model::Dir::Right,
        ),
        (model::Dir::Left, 'L') => (
            model::Dir::Up.move_pos(&pos, pipes).unwrap(),
            model::Dir::Up,
        ),
        (model::Dir::Down, 'J') => (
            model::Dir::Left.move_pos(&pos, pipes).unwrap(),
            model::Dir::Left,
        ),
        (model::Dir::Right, 'J') => (
            model::Dir::Up.move_pos(&pos, pipes).unwrap(),
            model::Dir::Up,
        ),
        (model::Dir::Up, '7') => (
            model::Dir::Left.move_pos(&pos, pipes).unwrap(),
            model::Dir::Left,
        ),
        (model::Dir::Right, '7') => (
            model::Dir::Down.move_pos(&pos, pipes).unwrap(),
            model::Dir::Down,
        ),
        (model::Dir::Up, 'F') => (
            model::Dir::Right.move_pos(&pos, pipes).unwrap(),
            model::Dir::Right,
        ),
        (model::Dir::Left, 'F') => (
            model::Dir::Down.move_pos(&pos, pipes).unwrap(),
            model::Dir::Down,
        ),
        _ => (pos, model::Dir::None),
    }

    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    //     model::Dir::Up,
    //     model::Dir::Right,
    //     model::Dir::Down,
    //     model::Dir::Left,
    // ];

    // for o in order.iter() {
    //     if *o == origin {
    //         continue;
    //     }

    //     // let next = o.move_pos(origin);
    // }

    // (pos, model::Dir::None)
}
