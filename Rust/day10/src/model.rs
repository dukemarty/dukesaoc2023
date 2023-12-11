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
