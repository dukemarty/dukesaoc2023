use std::cmp;

use aoc_utils::data::MapPos;

pub fn find_all_stars(star_map: &Vec<Vec<char>>) -> Vec<MapPos> {
    let mut res = Vec::new();

    for (ri, r) in star_map.iter().enumerate() {
        for (ci, c) in r.iter().enumerate() {
            if *c == '#' {
                res.push(MapPos { r: ri, c: ci });
            }
        }
    }

    res
}

pub fn determine_empty_rows_and_columns(
    row_count: usize,
    col_count: usize,
    stars: &Vec<MapPos>,
) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    let mut rows = vec![true; row_count];
    let mut cols = vec![true; col_count];

    for s in stars {
        rows[s.r] = false;
        cols[s.c] = false;
    }

    for (ri, r) in rows.iter().enumerate() {
        if *r {
            empty_rows.push(ri);
        }
    }
    for (ci, c) in cols.iter().enumerate() {
        if *c {
            empty_cols.push(ci);
        }
    }

    (empty_rows, empty_cols)
}

struct PosRange {
    pub from: usize,
    pub to: usize,
}

impl PosRange {
    fn create(v1: usize, v2: usize) -> PosRange {
        PosRange {
            from: cmp::min(v1, v2),
            to: cmp::max(v1, v2),
        }
    }

    fn dist(&self) -> usize {
        self.to - self.from
    }

    fn is_in_range(&self, v: usize) -> bool {
        self.from <= v && v <= self.to
    }
}

pub fn calc_stars_dist(
    star1: &MapPos,
    star2: &MapPos,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
) -> usize {
    let col_range = PosRange::create(star1.c, star2.c);
    let row_range = PosRange::create(star1.r, star2.r);
    // let col_from = cmp::min(star1.c, star2.c);
    // let col_to = cmp::max(star1.c, star2.c);
    // let row_from = cmp::min(star1.r, star2.r);
    // let row_to = cmp::max(star1.r, star2.r);

    let col_dist = col_range.dist()
        + 999999*empty_cols
            .iter()
            .filter(|c| col_range.is_in_range(**c))
            .count();
    let row_dist = row_range.dist()
        + 999999*empty_rows
            .iter()
            .filter(|r| row_range.is_in_range(**r))
            .count();

    col_dist + row_dist
}
