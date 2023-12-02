pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod aoc {

    pub fn print_day_header(day: i32, title: &str) {
        println!("");
        println!("--- Day {:0>2}: {} ---", day, title);
        println!("{}", "=".repeat(16 + title.len()));
        println!("");
    }

    pub fn print_part_header(id: i32, title: &str){
        println!("Part {}: {}", id, title);
        println!("{}", "-".repeat(8 + title.len()));
    }
}

pub mod io_utils {

    use std::{fs, str};

    pub fn read_lines(filename: &str) -> Vec<String> {
        let content = fs::read_to_string(filename).expect("Should have been able to read the file");
        let res: Vec<String> = content.lines().map(ToOwned::to_owned).collect();
        res
    }

}