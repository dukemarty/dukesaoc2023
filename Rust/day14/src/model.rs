
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