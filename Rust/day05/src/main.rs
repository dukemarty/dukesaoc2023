use std::collections::HashMap;

use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
        aoc::print_day_header(5, "If You Give A Seed A Fertilizer");

    // let raw_blocks = io_utils::read_emptyline_separated_blocks("TestData1.txt");
    let raw_blocks = io_utils::read_emptyline_separated_blocks("puzzle.txt");

    part1(&raw_blocks);

    part2(&raw_blocks);
}

fn part1(blocks: &Vec<Vec<String>>) {
    aoc::print_part_header(1, "Lowest location");

    let seeds = blocks[0][0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("Seeds: {:?}", seeds);

    let mappings = blocks
        .iter()
        .skip(1)
        .map(|b| model::Block::parse_map_block(b))
        .collect::<Vec<model::Block>>();
    let mut all_maps = HashMap::new();
    for m in mappings.iter() {
        all_maps.insert(m.caption.clone(), m);
        // println!("Mapping: {}", m.caption);
    }
    // println!("All mappings:\n{:?}\n", all_maps);

    let all_locations = seeds
        .iter()
        .map(|s| get_location_for_seed(&all_maps, *s))
        .collect::<Vec<i64>>();
    println!("All locations: {:?}", all_locations);

    let lowest_loc = all_locations.iter().min().unwrap();

    println!("Location: {}", lowest_loc);
}

fn get_location_for_seed(all_maps: &HashMap<String, &model::Block>, seed: i64) -> i64 {
    let mut res = seed;
    res = all_maps.get("seed-to-soil").unwrap().map_source(res);
    res = all_maps.get("soil-to-fertilizer").unwrap().map_source(res);
    res = all_maps.get("fertilizer-to-water").unwrap().map_source(res);
    res = all_maps.get("water-to-light").unwrap().map_source(res);
    res = all_maps
        .get("light-to-temperature")
        .unwrap()
        .map_source(res);
    res = all_maps
        .get("temperature-to-humidity")
        .unwrap()
        .map_source(res);
    res = all_maps
        .get("humidity-to-location")
        .unwrap()
        .map_source(res);

    res
}

fn part2(blocks: &Vec<Vec<String>>) {
    aoc::print_part_header(2, "Lowest location for seed ranges");

    let seeds = model::Range::parse_range_pairs(&blocks[0][0]);
    println!("Seeds: {:?}", seeds);

    let mappings = blocks
        .iter()
        .skip(1)
        .map(|b| model::Block::parse_map_block(b))
        .collect::<Vec<model::Block>>();
    let mut all_maps = HashMap::new();
    for m in mappings.iter() {
        all_maps.insert(m.caption.clone(), m);
        // println!("Mapping: {}", m.caption);
    }
    // println!("All mappings:\n{:?}\n", all_maps);

    let all_locations = get_locations_for_seed_range(&all_maps, &seeds);
    // println!("All locations: {:?}", all_locations);

    let lowest_loc = all_locations.iter().map(|r| r.from).min().unwrap();

    println!("Location: {}", lowest_loc);
}

fn get_locations_for_seed_range<'a>(
    all_maps: &'a HashMap<String, &'a model::Block>,
    seeds: &'a Vec<model::Range>,
) -> Vec<model::Range> {
    let order = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    let mut res = seeds.clone();

    for next_step in order {
        // println!("---------------------------------------------");
        // println!("Ranges before cut: {:?}", res);
        let next_map = *all_maps.get(next_step).unwrap();
        // println!("Relevant mapping: {:?}", next_map);
        let inter_res1 = next_map.recut_ranges(&res);
        // println!("Ranges after {}: {:?}", next_step, inter_res1);
        let inter_res2 = next_map.remap_ranges(inter_res1);
        res = inter_res2.clone();
    }

    res
}
