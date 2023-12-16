pub fn run_hash_algorithm(line: &str) -> u32 {
    let mut res = 0;

    for c in line.chars() {
        res = ((res + (c as u32)) * 17) % 256;
    }

    res
}

pub enum Step {
    None,
    Remove(String),
    Replace(String, u8),
}

impl Step {
    pub fn parse(step: &str) -> Step {
        if step.chars().last().unwrap() == '-' {
            Step::Remove(step.trim_end_matches('-').to_string())
        } else {
            let parts = step.split('=').collect::<Vec<&str>>();
            Step::Replace(
                parts[0].to_string(),
                u8::from_str_radix(parts[1], 10).unwrap(),
            )
        }
    }
}

#[derive(Debug)]
pub struct Lens {
    pub label: String,
    pub focus: u8,
}

#[derive(Debug)]
pub struct LensBoxes {
    pub boxes: Vec<Vec<Lens>>,
}

impl LensBoxes {
    pub fn create() -> LensBoxes {
        LensBoxes {
            boxes: (0..256).map(|_| Vec::new()).collect::<Vec<Vec<Lens>>>(),
        }
    }

    pub fn perform_step(&mut self, step: &Step) {
        let label = match step {
            Step::None => "",
            Step::Remove(name) => name,
            Step::Replace(name, _) => name,
        };
        let box_index = run_hash_algorithm(label) as usize;

        match step {
            Step::None => {}
            Step::Remove(_) => {
                let lens = self.boxes[box_index]
                    .iter()
                    .position(|l| (*l).label == label);
                match lens {
                    Some(index) => {
                        self.boxes[box_index as usize].remove(index);
                    }
                    None => {}
                };
            }
            Step::Replace(_, focus) => {
                let lens = self.boxes[box_index]
                    .iter()
                    .position(|l| (*l).label == label);
                match lens {
                    Some(index) => {
                        self.boxes[box_index as usize][index].focus = *focus;
                    }
                    None => self.boxes[box_index].push(Lens {
                        label: label.to_string(),
                        focus: *focus,
                    }),
                };
            }
        }
    }

    pub fn calc_focusing_power(&self) -> u32 {
        let mut res: u32 = 0;

        for (bi, b) in self.boxes.iter().enumerate() {
            for (li, l) in b.iter().enumerate() {
                res += ((bi as u32) + 1) * ((li as u32) + 1) * (l.focus as u32);
            }
        }

        res
    }
}
