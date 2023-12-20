
use std::{fs, str};

pub fn read_single_line(filename: &str) -> String {
    fs::read_to_string(filename).expect("Should have been able to read the file")
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");
    let res: Vec<String> = content.lines().map(ToOwned::to_owned).collect();
    res
}

pub fn read_emptyline_separated_blocks(filename: &str) -> Vec<Vec<String>> {
    // let content = fs::read_to_string(filename).expect("Should have been able to read the file");
    // let res: Vec<String> = content.split("\n\n").map(ToOwned::to_owned).collect();
    // res

    let lines = read_lines(filename);
    let mut res = Vec::new();
    let mut b = Vec::new();
    for l in lines {
        if l.is_empty() {
            if b.len() > 0 {
                res.push(b);
                b = Vec::new();
            }
        } else {
            b.push(l);
        }
    }

    if b.len() > 0 {
        res.push(b);
    }

    return res;
}

pub fn read_2d_char_map(filename: &str) -> Vec<Vec<char>> {
    let lines = read_lines(filename);
    lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}