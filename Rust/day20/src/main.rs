use std::collections::{HashMap, VecDeque};

use aoc_utils::{aoc, io_utils};

mod model;

fn main() {
    aoc::print_day_header(20, "Pulse Propagation");

    // let lines = io_utils::read_lines("testdata1.txt");
    // let lines = io_utils::read_lines("testdata2.txt");
    let lines = io_utils::read_lines("puzzle.txt");
    let modules: Vec<model::Module> = lines
        .iter()
        .map(|l| model::Module::parse(l))
        .filter(|om| om.is_some())
        .map(|om| om.unwrap())
        .collect();
    // println!("Modules: {:?}", modules);

    part1(&modules);
}

fn part1(modules: &Vec<model::Module>) {
    aoc::print_part_header(1, "Product of pulse counts");

    let mut mod_map: HashMap<String, model::Module> = modules
        .iter()
        .filter(|m| m.mod_type != model::ModuleType::Conjunction)
        .map(|m| (m.name.clone(), m.clone()))
        .collect();
    for mdl in modules
        .iter()
        .filter(|m| m.mod_type == model::ModuleType::Conjunction)
    {
        let mut mod_copy = mdl.clone();
        for m in modules.iter() {
            if m.destinations.contains(&mdl.name) {
                mod_copy
                    .conjunction_memory
                    .insert(m.name.clone(), model::PulseValue::Low);
            }
        }
        mod_map.insert(mod_copy.name.clone(), mod_copy);
    }
    // println!("Module map: {:?}", mod_map);

    let mut pulses = VecDeque::new();

    let mut low_count = 0;
    let mut high_count = 0;

    let mut iout = 0;
    for i in 1..1001 {
        // println!(
        //     "=== {:03} ============================================================",
        //     i
        // );
        let initial_pulse = model::Pulse {
            sender: String::from("button"),
            val: model::PulseValue::Low,
            rec: String::from("broadcaster"),
        };
        pulses.push_back(initial_pulse);
        while pulses.len() > 0 {
            // println!("------------------------------------------");
            let next = pulses.pop_front().unwrap();
            // println!("Next pulse: {:?}", next);
            match next.val {
                model::PulseValue::Low => low_count += 1,
                model::PulseValue::High => high_count += 1,
            }
            if !mod_map.contains_key(&next.rec) {
                continue;
            }
            // let found_mod = mod_map.get_mut(&next.rec);
            // match found_mod {
            //     Some(m) => println!("Found receiving module: {:?}", m),
            //     None => println!("ERROR could not find receiving module..."),
            // }
            let new_pulses = mod_map
                .get_mut(&next.rec)
                .unwrap()
                .process_pulse(&next.sender, next.val);
            // println!("New pulses: {:?}", new_pulses);
            // println!(
            //     "Mod after processing: {:?}",
            //     mod_map.get(&next.rec).unwrap()
            // );

            for np in new_pulses.iter() {
                pulses.push_back((*np).clone());
            }
            // pulses.append(new_pulses);
        }
        if mod_map.values().all(|m| m.is_in_init_state()) {
            println!("Found init state after {}", i);
            iout = i;
            break;
        }
    }
    println!("Low count={} , High count={}", low_count, high_count);
    let factor = if iout == 0 { 1i64 } else { 1000i64 / iout };

    let res = (low_count as i64) * (high_count as i64) * factor * factor;

    println!("Product: {}", res);
}
