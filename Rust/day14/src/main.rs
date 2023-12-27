use aoc_utils::{aoc, io_utils};
use md5;
use std::collections::HashMap;

mod model;

fn main() {
    aoc::print_day_header(14, "Parabolic Reflector Dish");

    // let map = io_utils::read_2d_char_map("testdata1.txt");
    let map = io_utils::read_2d_char_map("puzzle.txt");

    part1(&map);

    part2(&map);
}

fn part1(map: &Vec<Vec<char>>) {
    aoc::print_part_header(1, "Load on north support beam after one N tilt");

    let tilted = model::tilt_north(map);

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

const TEST_CYCLES: u64 = 1000000000;

fn part2(map: &Vec<Vec<char>>) {
    aoc::print_part_header(2, "Load on north after 1000000000 cycles");

    let mut cycle = map;
    let mut next_cycle: Vec<Vec<char>>;
    let mut seen: HashMap<md5::Digest, u64> = HashMap::new();
    let mut round = 0u64;
    let mut cycle_len = 0u64;
    while true {
        round += 1;
        next_cycle = model::perform_full_tilt_cycle(cycle);
        let bytes: Vec<u8> = next_cycle.concat().iter().map(|c| *c as u8).collect();
        let digest = md5::compute(bytes);
        // println!("Test digest: {:?}", digest);

        if seen.contains_key(&digest) {
            cycle = &next_cycle;
            println!(
                "Found my cycle at round {}, same as {}",
                round,
                seen.get(&digest).unwrap()
            );
            cycle_len = round - seen.get(&digest).unwrap();
            break;
        } else {
            seen.insert(digest, round);
        }

        cycle = &next_cycle;

        // let output: Vec<String> = next_cycle
        //     .iter()
        //     .map(|l| l.into_iter().collect::<String>())
        //     .collect();
        // for l in output {
        //     println!("{}", l);
        // }
    }

    let remaining_cycles = (TEST_CYCLES - round) % cycle_len;
    println!("remaining: {}", remaining_cycles);

    for _ in 0..remaining_cycles {
        next_cycle = model::perform_full_tilt_cycle(cycle);
        cycle = &next_cycle;
    }

    let res = model::calculate_load_in_map(cycle);
    println!("Load on north: {}", res);
}
