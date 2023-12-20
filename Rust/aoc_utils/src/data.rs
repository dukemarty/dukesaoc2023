
/// Representation of a 2d position in a row-column format.
#[derive(Debug, Clone, Hash)]
pub struct MapPos {
    pub r: usize,
    pub c: usize,
}



impl MapPos {
    /// Get referenced element from a 2d char map.
    pub fn sym(&self, charmap: &Vec<Vec<char>>) -> char {
        charmap[self.r][self.c]
    }
}


#[derive(Debug)]
pub enum Dir {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Dir {

    pub fn opp(&self) -> Dir {
        match self {
            Dir::None => Dir::None,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn turn_right(&self) -> Dir{
        match self {
            Dir::None => Dir::None,
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    pub fn turn_left(&self) -> Dir{
        match self {
            Dir::None => Dir::None,
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
}
