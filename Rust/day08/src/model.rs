use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

impl Node {
    pub fn parse(line: &String) -> Option<Node> {
        // AAA = (BBB, BBB)
        static PARSE_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap());

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

    pub fn is_start_node(&self) -> bool {
        self.name.chars().last().unwrap() == 'A'
    }

    pub fn is_end_node(&self) -> bool {
        self.name.chars().last().unwrap() == 'Z'
    }
}

#[derive(Debug)]
pub struct Route {
    pub pos: String,
    pub rounds: i32,
}

#[derive(Debug)]
pub struct MultiRoute {
    pub routes: Vec<Route>,
    pub rounds_till_end: HashMap<String, (String, i32)>,
}

impl MultiRoute {
    pub fn create(
        instr: &Vec<char>,
        nodes: &HashMap<String, &Node>,
        starts: &HashSet<String>,
        ends: &HashSet<String>,
    ) -> MultiRoute {
        let rounds_till_end: HashMap<_, _> = starts
            .union(ends)
            .map(|nn| (nn.clone(), Self::rounds_till_some_end(nn, instr, nodes)))
            .collect();
        // println!("Rounds till end: {:?}", rounds_till_end);

        MultiRoute {
            rounds_till_end,
            routes: starts
                .iter()
                .map(|nn| Route {
                    pos: nn.clone(),
                    rounds: 0,
                })
                .collect::<Vec<Route>>(),
        }
    }

    fn rounds_till_some_end(
        name: &String,
        instr: &Vec<char>,
        nodes: &HashMap<String, &Node>,
    ) -> (String, i32) {
        let mut rounds = 0;
        let mut curr = name;
        let mut is_first = true;
        while !nodes[curr].is_end_node() || is_first {
            for c in instr.iter() {
                curr = match c {
                    'L' => &nodes[curr].left,
                    'R' => &nodes[curr].right,
                    _ => {
                        println!("ERROR could not step!!!");
                        curr
                    }
                };
            }
            rounds += 1;
            is_first = false;
        }

        (curr.clone(), rounds)
    }
}

#[derive(Debug)]
pub struct Step {
    pub node: String,
    pub count: i32,
}

#[derive(Debug)]
pub struct Congruence {
    pub a: i32,
    pub m: i32,
}
