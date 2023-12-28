use std::{fmt, process};

use aoc_utils::data::{self, MapPos};

#[derive(Debug, PartialEq, Clone)]
pub enum Dir {
    None,
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn opposition(&self) -> Dir {
        match self {
            Dir::None => Dir::None,
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }

    pub fn move_pos(&self, pos: &data::MapPos, charmap: &Vec<Vec<char>>) -> Option<data::MapPos> {
        match *self {
            Dir::None => Some(pos.clone()),
            Dir::Up => {
                if pos.r > 0 {
                    Some(MapPos {
                        r: pos.r - 1,
                        c: pos.c,
                    })
                } else {
                    None
                }
            }
            Dir::Right => {
                if pos.c + 1 < charmap[0].len() {
                    Some(MapPos {
                        r: pos.r,
                        c: pos.c + 1,
                    })
                } else {
                    None
                }
            }
            Dir::Down => {
                if pos.r + 1 < charmap.len() {
                    Some(MapPos {
                        r: pos.r + 1,
                        c: pos.c,
                    })
                } else {
                    None
                }
            }
            Dir::Left => {
                if pos.c > 0 {
                    Some(MapPos {
                        r: pos.r,
                        c: pos.c - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn find_connections(pos: &data::MapPos, pipes: &Vec<Vec<char>>) -> Vec<Dir> {
    let mut res = Vec::new();

    match Dir::Up.move_pos(pos, pipes) {
        Some(next) => {
            let next_sym = pipes[next.r][next.c];
            if next_sym == '|' || next_sym == '7' || next_sym == 'F' {
                res.push(Dir::Up);
            }
        }
        None => {}
    };
    match Dir::Right.move_pos(pos, pipes) {
        Some(next) => {
            let next_sym = pipes[next.r][next.c];
            if next_sym == '-' || next_sym == '7' || next_sym == 'J' {
                res.push(Dir::Right);
            }
        }
        None => {}
    };
    match Dir::Down.move_pos(pos, pipes) {
        Some(next) => {
            let next_sym = pipes[next.r][next.c];
            if next_sym == '|' || next_sym == 'L' || next_sym == 'J' {
                res.push(Dir::Down);
            }
        }
        None => {}
    };
    match Dir::Left.move_pos(pos, pipes) {
        Some(next) => {
            let next_sym = pipes[next.r][next.c];
            if next_sym == '-' || next_sym == 'L' || next_sym == 'F' {
                res.push(Dir::Left);
            }
        }
        None => {}
    };

    res
}

pub fn step(pipes: &Vec<Vec<char>>, pos: data::MapPos, origin: Dir) -> (data::MapPos, Dir) {
    if origin == Dir::None {
        process::exit(1);
    }
    // println!("Moving from pos {:?} from {:?}", pos, origin);
    match (origin, pos.sym(pipes)) {
        (Dir::Up, '|') => (Dir::Up.move_pos(&pos, pipes).unwrap(), Dir::Up),
        (Dir::Down, '|') => (Dir::Down.move_pos(&pos, pipes).unwrap(), Dir::Down),
        (Dir::Left, '-') => (Dir::Left.move_pos(&pos, pipes).unwrap(), Dir::Left),
        (Dir::Right, '-') => (Dir::Right.move_pos(&pos, pipes).unwrap(), Dir::Right),
        (Dir::Down, 'L') => (Dir::Right.move_pos(&pos, pipes).unwrap(), Dir::Right),
        (Dir::Left, 'L') => (Dir::Up.move_pos(&pos, pipes).unwrap(), Dir::Up),
        (Dir::Down, 'J') => (Dir::Left.move_pos(&pos, pipes).unwrap(), Dir::Left),
        (Dir::Right, 'J') => (Dir::Up.move_pos(&pos, pipes).unwrap(), Dir::Up),
        (Dir::Up, '7') => (Dir::Left.move_pos(&pos, pipes).unwrap(), Dir::Left),
        (Dir::Right, '7') => (Dir::Down.move_pos(&pos, pipes).unwrap(), Dir::Down),
        (Dir::Up, 'F') => (Dir::Right.move_pos(&pos, pipes).unwrap(), Dir::Right),
        (Dir::Left, 'F') => (Dir::Down.move_pos(&pos, pipes).unwrap(), Dir::Down),
        _ => (pos, Dir::None),
    }

    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    //     Dir::Up,
    //     Dir::Right,
    //     Dir::Down,
    //     Dir::Left,
    // ];

    // for o in order.iter() {
    //     if *o == origin {
    //         continue;
    //     }

    //     // let next = o.move_pos(origin);
    // }

    // (pos, Dir::None)
}

pub struct AreaMap {
    pub grid: Vec<Vec<char>>,
    pub inner_count: u32,
}

impl fmt::Display for AreaMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .grid
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl AreaMap {
    pub fn create(base_map: &Vec<Vec<char>>, start: &MapPos) -> AreaMap {
        let mut area_map = Vec::new();
        for _ in 0..base_map.len() {
            let mut next_row = Vec::new();
            for _ in 0..base_map[0].len() {
                next_row.push('.');
            }
            area_map.push(next_row);
        }

        area_map[start.r][start.c] = '#';
        let start_dirs = find_connections(start, base_map);
        let first_step = start_dirs[0].move_pos(start, base_map).unwrap();
        area_map[first_step.r][first_step.c] = '#';
        let mut next_step = first_step;
        let mut from_dir = start_dirs[0].clone();
        while next_step.sym(base_map) != 'S' {
            // println!("Loop: ({:?}) from {:?}", next_step, from_dir);
            (next_step, from_dir) = step(base_map, next_step, from_dir);
            area_map[next_step.r][next_step.c] = '#';
        }

        AreaMap {
            grid: area_map,
            inner_count: 0,
        }
    }

    pub fn mark_inner_area(&mut self, pipes: &Vec<Vec<char>>) {
        for r in 0..self.grid.len() {
            let mut in_ = false;
            let mut last_in_sym = ' ';
            for c in 0..self.grid[0].len() {
                match self.grid[r][c] {
                    '.' if in_ => {
                        self.grid[r][c] = '*';
                        self.inner_count += 1;
                        // println!("Set inner area to '*'");
                    }
                    '#' if pipes[r][c] == '|' => {
                        in_ = !in_;
                        // println!("Toggled in_ to {}", in_);
                    }
                    '#' => match pipes[r][c] {
                        '7' if last_in_sym == 'L' => in_ = !in_,
                        'J' if last_in_sym == 'F' => in_ = !in_,
                        'F' => last_in_sym = 'F',
                        'L' => last_in_sym = 'L',
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn print_relevant_pipes(&self, pipes: &Vec<Vec<char>>) {
        for r in 0..self.grid.len() {
            for c in 0..self.grid[0].len() {
                if self.grid[r][c] == '#' {
                    print!("{}", pipes[r][c]);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}
