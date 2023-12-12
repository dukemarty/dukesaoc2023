#[derive(Debug, Clone)]
pub struct Range {
    pub from: i64,
    pub to: i64,
}

impl Range {
    pub fn parse_range_pairs(line: &String) -> Vec<Range> {
        let nums = line.split(":").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut res = Vec::new();

        for i in 0..nums.len() / 2 {
            let r = Range {
                from: nums[2 * i],
                to: nums[2 * i] + nums[2 * i + 1] - 1,
            };
            res.push(r);
        }

        return res;
    }
}

#[derive(Debug)]
pub struct Mapping {
    pub from: i64,
    pub length: i64,
    pub shift: i64,
}

impl Mapping {
    pub fn parse_from_line(line: &String) -> Mapping {
        let parts = line
            .split_whitespace()
            .map(|t| t.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

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
            mappings: raw_block[1..]
                .to_vec()
                .iter()
                .map(|l| Mapping::parse_from_line(l))
                .collect::<Vec<Mapping>>(),
        };

        res
    }

    pub fn map_source(&self, source: i64) -> i64 {
        for m in self.mappings.iter() {
            if source >= m.from && source < m.from + m.length {
                return source + m.shift;
            }
        }

        source
    }

    pub fn recut_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut res = ranges
            .iter()
            .map(|r| Range {
                from: r.from,
                to: r.to,
            })
            .collect::<Vec<Range>>();

        for m in self.mappings.iter() {
            let mut next = Vec::new();
            for r in res {
                let mut from = r.from;
                if r.from < m.from && m.from < r.to {
                    next.push(Range {
                        from: r.from,
                        to: m.from - 1,
                    });
                    from = m.from;
                }
                if r.from < (m.from + m.length - 1) && (m.from + m.length - 1) < r.to {
                    next.push(Range {
                        from,
                        to: (m.from + m.length - 1),
                    });
                    from = m.from + m.length;
                }
                next.push(Range { from, to: r.to });
            }
            res = next;
        }

        return res;
    }

    pub fn remap_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut res = Vec::new();

        for r in ranges {
            res.push(Range {
                from: self.map_source(r.from),
                to: self.map_source(r.to),
            });
        }

        return res;
    }
}
