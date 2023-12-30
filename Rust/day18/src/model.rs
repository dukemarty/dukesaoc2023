use aoc_utils::data;

#[derive(Debug)]
pub struct Box {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

pub fn parse(token: &str) -> Option<data::Dir> {
    match token {
        "R" => Some(data::Dir::Right),
        "D" => Some(data::Dir::Down),
        "L" => Some(data::Dir::Left),
        "U" => Some(data::Dir::Up),
        _ => None,
    }
}

#[derive(Debug)]
pub struct DigCmd {
    pub dir: data::Dir,
    pub len: u32,
    pub color: String,
}

impl DigCmd {
    pub fn parse(line: &String) -> DigCmd {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let dir = parse(parts[0]).unwrap();
        let len = parts[1].parse::<u32>().unwrap();
        let color = parts[2].trim_start_matches("(").trim_end_matches(")");
        DigCmd {
            dir,
            len,
            color: color.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct DigPlan {
    pub cmds: Vec<DigCmd>,
    pub bounds: Box,
}

impl DigPlan {
    pub fn create(cmds: Vec<DigCmd>) -> DigPlan {
        let bounds = DigPlan::calc_bounds(&cmds);
        DigPlan { cmds: cmds, bounds }
    }

    fn calc_bounds(cmds: &Vec<DigCmd>) -> Box {
        Box {
            x_min: cmds
                .iter()
                .filter_map(|e| {
                    if (*e).dir == data::Dir::Left {
                        Some((*e).len as i32)
                    } else {
                        None
                    }
                })
                .sum::<i32>()
                * (-1),
            x_max: cmds
                .iter()
                .filter_map(|e| {
                    if (*e).dir == data::Dir::Right {
                        Some((*e).len as i32)
                    } else {
                        None
                    }
                })
                .sum::<i32>(),
            y_min: cmds
                .iter()
                .filter_map(|e| {
                    if (*e).dir == data::Dir::Down {
                        Some((*e).len as i32)
                    } else {
                        None
                    }
                })
                .sum::<i32>()
                * (-1),
            y_max: cmds
                .iter()
                .filter_map(|e| {
                    if (*e).dir == data::Dir::Up {
                        Some((*e).len as i32)
                    } else {
                        None
                    }
                })
                .sum::<i32>(),
        }
    }

    pub fn draw(&self, grid: &mut Vec<Vec<char>>, pos: &mut data::MapPos, mark: char) {
        // pos.set('#',  grid);
        pos.set(mark, grid);
        for cmd in self.cmds.iter() {
            for _ in 0..cmd.len {
                pos.mv(&cmd.dir);
                // pos.set('#', &mut grid);
                pos.set(mark, grid);
            }
        }
    }
}

enum FillState {
    None,
    Out,
    OutOnHorizontalFromBelow,
    OutOnHorizontalFromAbove,
    In,
    InOnHorizontalFromBelow,
    InOnHorizontalFromAbove,
}

pub fn fill_trench(grid: &mut Vec<Vec<char>>, boundary_mark: char, fill_in_mark: char) {
    for r in 0..grid.len() {
        let mut state = FillState::Out;
        for c in 0..grid[0].len() {
            match state {
                FillState::None => {}
                FillState::Out if grid[r][c] == boundary_mark => {
                    if grid[r - 1][c] == boundary_mark && grid[r + 1][c] == boundary_mark {
                        state = FillState::In;
                    } else if grid[r - 1][c] == boundary_mark {
                        state = FillState::OutOnHorizontalFromAbove;
                    } else if grid[r + 1][c] == boundary_mark {
                        state = FillState::OutOnHorizontalFromBelow;
                    } else {
                        println!("ERROR impossible situation of boundary field!!!");
                    }
                }
                FillState::In if grid[r][c] == '.' => {
                    grid[r][c] = fill_in_mark;
                }
                FillState::In if grid[r][c] == boundary_mark => {
                    if grid[r - 1][c] == boundary_mark && grid[r + 1][c] == boundary_mark {
                        state = FillState::Out;
                    } else if grid[r - 1][c] == boundary_mark {
                        state = FillState::InOnHorizontalFromAbove;
                    } else if grid[r + 1][c] == boundary_mark {
                        state = FillState::InOnHorizontalFromBelow;
                    } else {
                        println!("ERROR impossible situation of boundary field!!!");
                    }
                }
                FillState::OutOnHorizontalFromBelow if grid[r][c] == '.' => {
                    if grid[r - 1][c - 1] == boundary_mark {
                        state = FillState::In;
                        grid[r][c] = fill_in_mark;
                    } else if grid[r + 1][c - 1] == boundary_mark {
                        state = FillState::Out;
                    } else {
                        println!("ERROR impossible situation of boundary field!!!");
                    }
                }
                FillState::OutOnHorizontalFromAbove if grid[r][c] == '.' => {
                    if grid[r - 1][c - 1] == boundary_mark {
                        state = FillState::Out;
                    } else if grid[r + 1][c - 1] == boundary_mark {
                        state = FillState::In;
                        grid[r][c] = fill_in_mark;
                    } else {
                        println!("ERROR impossible situation of boundary field!!!");
                    }
                }
                FillState::InOnHorizontalFromBelow if grid[r][c] == '.' => {
                    if grid[r - 1][c - 1] == boundary_mark {
                        state = FillState::Out;
                    } else if grid[r + 1][c - 1] == boundary_mark {
                        state = FillState::In;
                        grid[r][c] = fill_in_mark;
                    } else {
                        println!("ERROR impossible situation of boundary field!!!");
                    }
                }
                FillState::InOnHorizontalFromAbove if grid[r][c] == '.' => {
                    if grid[r - 1][c - 1] == boundary_mark {
                        state = FillState::In;
                        grid[r][c] = fill_in_mark;
                    } else if grid[r + 1][c - 1] == boundary_mark {
                        state = FillState::Out;
                    } else {
                        println!("ERROR impossible situation of boundary field!!!");
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn count_trench(grid: &Vec<Vec<char>>, boundary_mark: char, fill_mark: char) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|c| (**c) == boundary_mark || (**c) == fill_mark)
                .count()
        })
        .sum()
}
