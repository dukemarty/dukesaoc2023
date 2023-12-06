use aoc_utils::data_utils;

pub struct Race {
    pub time: i32,
    pub record: i32,
}

impl Race {
    pub fn parse_many(times: &String, records: &String) -> Vec<Race> {
        let mut res = Vec::new();

        let (_, times) = data_utils::parse_i32_list_ws_with_title(times);
        let (_, records) = data_utils::parse_i32_list_ws_with_title(records);

        for i in 0..times.len() {
            res.push(Race {
                time: times[i],
                record: records[i],
            });
        }

        return res;
    }

    pub fn count_ways_to_win(&self) -> i32 {
        let mut res = 0;
        for i in 1..self.time {
            let dist = (self.time - i) * i;
            if dist > self.record{
                res += 1;
            }
        }

        res
    }
}
