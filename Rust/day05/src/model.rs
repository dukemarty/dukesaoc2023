

#[derive(Debug)]
pub struct Mapping {
    pub from: i64,
    pub length: i64,
    pub shift: i64,
}

impl Mapping {

    pub fn parse_from_line(line: &String) -> Mapping {
        let parts = line.split_whitespace().map(|t| t.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let destination = parts[0];
        let source = parts[1];
        let length = parts[2];

        Mapping {
            from: source,
            length,
            shift: destination - source,
        }
    }

}

#[derive(Debug)]
pub struct Block {
    pub caption: String,
    pub mappings: Vec<Mapping>,
}

impl Block {

    pub fn parse_map_block(raw_block: &Vec<String>) -> Block {
        let res = Block {
            caption: raw_block[0].split_whitespace().collect::<Vec<&str>>()[0].to_string(),
            mappings: raw_block[1..].to_vec().iter().map(|l| Mapping::parse_from_line(l) ).collect::<Vec<Mapping>>(),
        };

        res
    }

    pub fn map_source(&self, source: i64) -> i64 {
        for m in self.mappings.iter() {
            if source >= m.from && source < m.from+m.length {
                return source + m.shift;
            }
        }

        source
    }
}
