use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(14, "Parabolic Reflector Dish");

    // let map = io_utils::read_2d_char_map("testdata1.txt");
    let map = io_utils::read_2d_char_map("puzzle.txt");

    part1(&map);

    part2();
}

fn part1(map: &Vec<Vec<char>>) {
    aoc::print_part_header(1, "Load on north support beam after one N tilt");

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

    // let output: Vec<String> = tilted
    //     .iter()
    //     .map(|l| l.into_iter().collect::<String>())
    //     .collect();
    // for l in output {
    //     println!("{}", l);
    // }

    let res = model::calculate_load_in_map(&tilted);

    println!("Total load: {}", res);
}

fn part2() {
    aoc::print_part_header(2, "");
}
