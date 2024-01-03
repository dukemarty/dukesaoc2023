use core::fmt;
use std::fmt::{Display, Formatter};

use regex::Regex;

// #[derive(Debug)]
pub struct ConditionRecord {
    pub elems: Vec<char>,
    pub groups: Vec<u32>,
    pub unknowns: Vec<usize>,
    pub check_rgx: Regex,
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "CondRec({}, {:?})",
            self.elems.iter().collect::<String>(),
            self.groups
        )
    }
}

impl ConditionRecord {
    pub fn parse(line: &String) -> ConditionRecord {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let groups = parts[1]
            .split(",")
            .map(|t| t.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let elems: Vec<char> = parts[0].chars().collect();
        let mut unknowns = Vec::new();
        for i in 0..elems.len() {
            if elems[i] == '?' {
                unknowns.push(i);
            }
        }

        let mut regex_string = String::from("");
        regex_string += r"^[\.\?]*";
        let mut first = true;
        for g in groups.iter() {
            if first {
                regex_string += format!("[\\?#]{{{}}}", *g).as_str();
                first = false;
            } else {
                regex_string += format!("[\\.\\?]+[\\?#]{{{}}}", *g).as_str();
            }
        }
        regex_string += r"[\.\?]*$";
        // println!("Created regex: {}", regex_string);
        let check_rgx = Regex::new(&regex_string).unwrap();

        ConditionRecord {
            elems,
            groups,
            unknowns,
            check_rgx,
        }
    }

    pub fn count_possible_valid_arrangements(&self) -> u32 {
        // let mut res = 0;

        if self.unknowns.len() == 0 {
            return 1;
        }

        let res = self.helper(BacktrackState {
            pos: 0,
            elems: self.elems.clone(),
        });

        // println!("Partial result, for {}: {}", self, res);

        res
    }

    fn helper(&self, state: BacktrackState) -> u32 {
        let mut succ_operational = BacktrackState {
            pos: state.pos + 1,
            elems: state.elems.clone(),
        };
        succ_operational.elems[self.unknowns[state.pos]] = '.';
        let mut succ_damaged = BacktrackState {
            pos: state.pos + 1,
            elems: state.elems.clone(),
        };
        succ_damaged.elems[self.unknowns[state.pos]] = '#';

        let mut res = 0;
        if state.pos == self.unknowns.len() - 1 {
            if succ_operational.is_consistent(&self.check_rgx) {
                // println!("  Determined as valid: {}", succ_operational.elems.iter().collect::<String>());
                res += 1;
            }
            if succ_damaged.is_consistent(&self.check_rgx) {
                // println!("  Determined as valid: {}", succ_damaged.elems.iter().collect::<String>());
                res += 1;
            }
        } else {
            if succ_operational.is_consistent(&self.check_rgx){
                res += self.helper(succ_operational);
            }
            if succ_damaged.is_consistent(&self.check_rgx){
                res += self.helper(succ_damaged);
            }
        }

        res
    }

    // fn find_first(elems: &Vec<char>){
    //     for i in 0..elems.len(){
    //         if elems[i] = '?'
    //     }
    // }
}

struct BacktrackState {
    pub pos: usize,
    pub elems: Vec<char>,
}

impl BacktrackState {
    pub fn is_consistent(&self, check: &Regex) -> bool {
        check.is_match(self.elems.iter().collect::<String>().as_str())
    }
}
