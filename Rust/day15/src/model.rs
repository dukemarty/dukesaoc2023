pub fn run_hash_algorithm(line: &str) -> u32 {
    let mut res = 0;

    for c in line.chars() {
        res = ((res + (c as u32)) * 17) % 256;
    }

    res
}
