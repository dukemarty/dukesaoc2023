use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PulseValue {
    Low,
    High,
}

#[derive(Debug, Clone)]
pub struct Pulse {
    pub sender: String,
    pub val: PulseValue,
    pub rec: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    None,
    Broadcaster,
    FlipFlop(bool),
    Conjunction,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub mod_type: ModuleType,
    pub destinations: Vec<String>,
    pub conjunction_memory: HashMap<String, PulseValue>,
}

impl Module {
    pub fn parse(line: &String) -> Option<Module> {
        let parts = line.split("->").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return None;
        }

        let (mod_type, name) = match parts[0].chars().collect::<Vec<char>>()[0] {
            '%' => (
                ModuleType::FlipFlop(false),
                parts[0].to_string()[1..].trim().to_string(),
            ),
            '&' => (
                ModuleType::Conjunction,
                parts[0].to_string()[1..].trim().to_string(),
            ),
            _ => (ModuleType::Broadcaster, parts[0].trim().to_owned()),
        };

        let destinations = parts[1]
            .split(",")
            .map(|t| t.trim().to_string())
            .collect::<Vec<String>>();

        Some(Module {
            name,
            mod_type,
            destinations,
            conjunction_memory: HashMap::new(),
        })
    }

    pub fn is_in_init_state(&self) -> bool {
        match self.mod_type {
            ModuleType::None => true,
            ModuleType::Broadcaster => true,
            ModuleType::FlipFlop(state) => {
                // if state {
                //     println!("Non init state: {:?}", self);
                // }
                !state
            }
            ModuleType::Conjunction => {
                let res = self
                    .conjunction_memory
                    .values()
                    .all(|cm| *cm == PulseValue::Low);
                // if !res {
                //     println!("Non init state: {:?}", self);
                // }
                res
            }
        }
    }

    pub fn process_pulse(&mut self, sender: &String, val: PulseValue) -> Vec<Pulse> {
        let mut res = Vec::new();

        match self.mod_type {
            ModuleType::None => {}
            ModuleType::Broadcaster => {
                for d in self.destinations.iter() {
                    res.push(Pulse {
                        sender: self.name.clone(),
                        rec: d.clone(),
                        val: val,
                    })
                }
            }
            ModuleType::FlipFlop(mut state) => {
                if val == PulseValue::Low {
                    self.mod_type = ModuleType::FlipFlop(!state);
                    state = !state;
                    for d in self.destinations.iter() {
                        res.push(Pulse {
                            sender: self.name.clone(),
                            rec: d.clone(),
                            val: if state {
                                PulseValue::High
                            } else {
                                PulseValue::Low
                            },
                        });
                    }
                }
            }
            ModuleType::Conjunction => {
                self.conjunction_memory.insert(sender.to_string(), val);
                // println!("Updated conjunction memory: {:?}", self.conjunction_memory);
                let all_inputs_high = self.conjunction_memory.values().all(|cm| {
                    // println!("Checking {:?} for high", *cm);
                    *cm == PulseValue::High
                });
                // println!("All inputs high: {}", all_inputs_high);
                let send_val = if all_inputs_high {
                    PulseValue::Low
                } else {
                    PulseValue::High
                };
                // println!("send_val={:?}", send_val);
                for d in self.destinations.iter() {
                    res.push(Pulse {
                        sender: self.name.clone(),
                        rec: d.clone(),
                        val: send_val,
                    });
                }
            }
        }

        res
    }
}
