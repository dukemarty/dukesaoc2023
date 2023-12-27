pub fn calculate_load_in_map(map: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;

    let max_val = map.len() as i32;
    for (ri, r) in map.iter().enumerate() {
        for c in r {
            if *c == 'O' {
                // println!("Found 'O' in line {}, adding {} to res.", ri, max_val - (ri as i32));
                res += max_val - (ri as i32);
            }
        }
    }

    res
}

pub fn perform_full_tilt_cycle(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {

    // 1st north
    let mut tilted_north: Vec<Vec<char>> = map.iter().map(|l| l.clone()).collect();
    for col in 0..map[0].len() {
        let mut tr = 0;
        for mr in 0..map.len() {
            match map[mr][col] {
                '#' => {
                    tr = mr + 1;
                }
                'O' => {
                    if tr < mr {
                        tilted_north[tr][col] = 'O';
                        tilted_north[mr][col] = '.';
                        tr += 1;
                    } else {
                        tr = mr + 1;
                    }
                }
                '.' => {}
                _ => {}
            };
        }
    }

    // 2nd west
    let mut tilted_west: Vec<Vec<char>> = tilted_north.iter().map(|l| l.clone()).collect();
    for row in 0..map.len() {
        let mut tl = 0;
        for ml in 0..map[0].len() {
            match tilted_north[row][ml] {
                '#' => {
                    tl = ml + 1;
                }
                'O' => {
                    if tl < ml {
                        tilted_west[row][tl] = 'O';
                        tilted_west[row][ml] = '.';
                        tl += 1;
                    } else {
                        tl = ml + 1;
                    }
                }
                '.' => {}
                _ => {}
            };
        }
    }

    // 3rd south
    let mut tilted_south: Vec<Vec<char>> = tilted_west.iter().map(|l| l.clone()).collect();
    for col in 0..map[0].len() {
        let mut tr = map.len() - 1;
        for mr in (0..map.len()).rev() {
            match tilted_west[mr][col] {
                '#' => {
                    if mr > 0 {
                        tr = mr - 1;
                    } else {
                        tr = 0;
                    }
                }
                'O' => {
                    if tr > mr {
                        tilted_south[tr][col] = 'O';
                        tilted_south[mr][col] = '.';
                        tr -= 1;
                    } else {
                        if mr > 0 {
                            tr = mr - 1;
                        } 
                        // else {
                        //     tr = 0;
                        // }
                    }
                }
                '.' => {}
                _ => {}
            };
        }
    }

    // 4th east
    let mut tilted_east: Vec<Vec<char>> = tilted_south.iter().map(|l| l.clone()).collect();
    for row in 0..map.len() {
        let mut tl = map[0].len() - 1;
        for ml in (0..map[0].len()).rev() {
            match tilted_south[row][ml] {
                '#' => {
                    if ml > 0 {
                        tl = ml - 1;
                    } else {
                        tl = 0;
                    }
                }
                'O' => {
                    if tl > ml {
                        tilted_east[row][tl] = 'O';
                        tilted_east[row][ml] = '.';
                        tl -= 1;
                    } else {
                        if ml > 0 {
                            tl = ml - 1;
                        } 
                        // else {
                        //     tl = 0;
                        // }
                    }
                }
                '.' => {}
                _ => {}
            };
        }
    }

    tilted_east
}

pub fn tilt_north(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted: Vec<Vec<char>> = map.iter().map(|l| l.clone()).collect();

    for col in 0..map[0].len() {
        let mut tr = 0;
        for mr in 0..map.len() {
            match map[mr][col] {
                '#' => {
                    tr = mr + 1;
                }
                'O' => {
                    if tr < mr {
                        tilted[tr][col] = 'O';
                        tilted[mr][col] = '.';
                        tr += 1;
                    } else {
                        tr = mr + 1;
                    }
                }
                '.' => {}
                _ => {}
            };
        }
    }

    tilted
}
