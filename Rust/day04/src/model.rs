use crate::main;

pub struct Card {
    pub id: u32,
    pub winners: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn parse(line: &str) -> Card {
        let main_parts = line.split(":").collect::<Vec<&str>>();
        let id = main_parts[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let num_parts = main_parts[1].split("|").collect::<Vec<&str>>();
        let winners = num_parts[0]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap());
        let numbers = num_parts[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap());
        Card {
            id: id,
            winners: winners.collect::<Vec<u32>>(),
            numbers: numbers.collect::<Vec<u32>>(),
        }
    }

    pub fn count_hits(&self) -> u32 {
        let mut res = 0;

        for n in self.numbers.iter() {
            if self.winners.contains(&n) {
                res += 1;
            }
        }

        res
    }
}
