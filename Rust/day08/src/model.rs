use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

impl Node {

    pub fn parse(line: &String) -> Option<Node> {
        // AAA = (BBB, BBB)
        static PARSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap());

        let Some(caps) = PARSE_RE.captures(&line) else {
            println!("ERROR could not parse node line: <{}>", line);
            return None;
        };

        Some(Node {
            name: caps["name"].to_string(),
            left: caps["left"].to_string(),
            right: caps["right"].to_string(),
        })
    }
}
