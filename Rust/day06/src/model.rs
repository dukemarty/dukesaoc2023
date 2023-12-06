use aoc_utils::data_utils;

#[derive(Debug)]
pub struct Race {
    pub time: i64,
    pub record: i64,
}

impl Race {
    pub fn parse_many(times: &String, records: &String) -> Vec<Race> {
        let mut res = Vec::new();

        let (_, times) = data_utils::parse_i32_list_ws_with_title(times);
        let (_, records) = data_utils::parse_i32_list_ws_with_title(records);

        for i in 0..times.len() {
            res.push(Race {
                time: i64::from(times[i]),
                record: i64::from(records[i]),
            });
        }

        return res;
    }

    pub fn parse_single_race(time: &String, record: &String) -> Race {
        let parts = time.split(":").collect::<Vec<&str>>();
        let t = parts[1]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .concat()
            .to_string();
        println!("t: {}", t);
        let time = t.parse::<i64>().unwrap();

        let parts = record.split(":").collect::<Vec<&str>>();
        let r = parts[1]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .concat()
            .to_string();
        println!("r: {}", r);
        let record = r.parse::<i64>().unwrap();

        Race { time, record }
    }

    pub fn count_ways_to_win1(&self) -> i64 {
        let mut res = 0;
        for i in 1..self.time {
            let dist = (self.time - i) * i;
            if dist > self.record {
                res += 1;
            }
        }

        // println!("  #solutions for {:?}: {}", &self, res);
        res
    }

    pub fn count_ways_to_win2(&self) -> i64 {
        let time = self.time as f64;
        let root = f64::sqrt(time * time - 4.0 * (self.record as f64));
        let ex_sol_upper = (time + root) / 2.0;
        let ex_sol_lower = (time - root) / 2.0;

        let mut lower = ex_sol_lower as i64 - 1;
        while ((self.time - lower) * lower) <= self. record {
            lower += 1
        }

        let mut upper = ex_sol_upper as i64 + 2;
        while ((self.time - upper) * upper) <= self.record {
            upper -= 1;
        }

        // println!("  solutions: {} / {}", upper, lower);
        let res = upper - lower + 1;

        // println!("  #solutions for {:?}: {}", &self, res);
        res
    }
}
