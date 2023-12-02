pub struct CubeDraw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeDraw {
    pub fn new() -> CubeDraw {
        CubeDraw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn parse(slice: &str) -> CubeDraw {
        let mut res = CubeDraw::new();
        for part in slice.split(",") {
            let tokens = part.split_whitespace().collect::<Vec<&str>>();
            let val = tokens[0].parse::<u32>().unwrap();
            match tokens[1].trim() {
                "blue" => res.blue = val,
                "green" => res.green = val,
                "red" => res.red = val,
                _ => println!("ERROR could not find given color"),
            }
        }

        res
    }

    pub fn could_be_bag(&self, red_count: u32, green_count: u32, blue_count: u32) -> bool {
        self.red <= red_count && self.green <= green_count && self.blue <= blue_count
    }
}

impl Default for CubeDraw {
    fn default() -> CubeDraw {
        CubeDraw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub struct Game {
    pub id: u32,
    pub draws: Vec<CubeDraw>,
}

impl Game {
    pub fn parse(line: &str) -> Game {
        let main_parts = line.split(":").collect::<Vec<&str>>();
        let id = main_parts[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let draws = Game::parse_draws(main_parts[1]);
        Game { id, draws }
    }

    fn parse_draws(slice: &str) -> Vec<CubeDraw> {
        let mut draws = Vec::new();
    
        for draw in slice.split(";") {
            let next_draw = CubeDraw::parse(draw);
            draws.push(next_draw);
        }
    
        draws
    }
    
    pub fn could_be_bag(&self, red_count: u32, green_count: u32, blue_count: u32) -> bool {
        for d in self.draws.iter() {
            if !d.could_be_bag(red_count, green_count, blue_count) {
                return false;
            }
        }

        return true;
    }
}
