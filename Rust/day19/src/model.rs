use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub enum Condition {
    None,
    LessThan(String, u32),
    GreaterThan(String, u32),
}

#[derive(Debug)]
pub enum Target {
    ACCEPT,
    REJECT,
    OtherRule(String),
}

#[derive(Debug)]
pub struct Rule {
    pub cond: Condition,
    pub target: Target,
}

impl Rule {
    pub fn parse(slice: &str) -> Option<Rule> {
        // a<2006:qkq
        // m>2090:A
        // rfg
        static PARSE_RE_LESS: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<prop>\w+)<(?<value>\d+):(?<name>\w+)").unwrap());
        static PARSE_RE_GREATER: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<prop>\w+)>(?<value>\d+):(?<name>\w+)").unwrap());
        static PARSE_RE_UNCOND: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<name>\w+)").unwrap());

        match PARSE_RE_LESS.captures(&slice) {
            Some(caps) => {
                return Some(Rule {
                    cond: Condition::LessThan(
                        caps["prop"].to_string(),
                        caps["value"].parse().unwrap(),
                    ),
                    target: match caps["name"].to_string().as_str() {
                        "A" => Target::ACCEPT,
                        "R" => Target::REJECT,
                        t => Target::OtherRule(t.to_string()),
                    },
                })
            }
            None => {}
        }
        match PARSE_RE_GREATER.captures(&slice) {
            Some(caps) => {
                return Some(Rule {
                    cond: Condition::GreaterThan(
                        caps["prop"].to_string(),
                        caps["value"].parse().unwrap(),
                    ),
                    target: match caps["name"].to_string().as_str() {
                        "A" => Target::ACCEPT,
                        "R" => Target::REJECT,
                        t => Target::OtherRule(t.to_string()),
                    },
                })
            }
            None => {}
        }
        match PARSE_RE_UNCOND.captures(&slice) {
            Some(caps) => {
                return Some(Rule {
                    cond: Condition::None,
                    target: match caps["name"].to_string().as_str() {
                        "A" => Target::ACCEPT,
                        "R" => Target::REJECT,
                        t => Target::OtherRule(t.to_string()),
                    },
                })
            }
            None => {}
        }

        println!("Could not parse rule: <{}>", slice);
        None
    }

    pub fn matches(&self, part: &Part) -> bool {
        match &self.cond {
            Condition::None => true,
            Condition::LessThan(prop, val) => match prop.as_str() {
                "x" => part.x < *val,
                "m" => part.m < *val,
                "a" => part.a < *val,
                "s" => part.s < *val,
                _ => false,
            },
            Condition::GreaterThan(prop, val) => match prop.as_str() {
                "x" => part.x > *val,
                "m" => part.m > *val,
                "a" => part.a > *val,
                "s" => part.s > *val,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Workflow {
    pub fn parse(line: &String) -> Option<Workflow> {
        // px{a<2006:qkq,m>2090:A,rfg}
        static PARSE_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<name>\w+)\{(?<rules_list>[^\}]+)\}").unwrap());

        let Some(caps) = PARSE_RE.captures(&line) else {
            println!("ERROR could not parse workflow line: <{}>", line);
            return None;
        };

        let rules_list: Vec<Rule> = caps["rules_list"]
            .split(',')
            .map(|t| Rule::parse(t).unwrap())
            .collect();

        Some(Workflow {
            name: caps["name"].to_string(),
            rules: rules_list,
        })
    }

    pub fn process(&self, part: &Part) -> &Target {
        for r in self.rules.iter() {
            if r.matches(part) {
                return &r.target;
            }
        }

        &Target::ACCEPT
    }
}

#[derive(Debug)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn parse(line: &String) -> Option<Part> {
        // {x=787,m=2655,a=1222,s=2876}
        static PARSE_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"\{x=(?<x_rating>\d+),m=(?<m_rating>\d+),a=(?<a_rating>\d+),s=(?<s_rating>\d+)\}",
            )
            .unwrap()
        });

        let Some(caps) = PARSE_RE.captures(&line) else {
            println!("ERROR could not parse part line: <{}>", line);
            return None;
        };

        Some(Part {
            x: caps["x_rating"].parse().unwrap(),
            m: caps["m_rating"].parse().unwrap(),
            a: caps["a_rating"].parse().unwrap(),
            s: caps["s_rating"].parse().unwrap(),
        })
    }

    pub fn get_full_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
