use core::panic;
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Module {
    name: String,
    modtype: ModuleType,
    dest: Vec<String>,
}

impl Module {
    fn from_string(s: &str) -> Self {
        match s.splitn(3, ' ').collect::<Vec<&str>>().as_slice() {
            [typeandname, _arrow, dest] if *typeandname == "broadcaster" => Module {
                name: "broadcaster".to_string(),
                modtype: ModuleType::Broadcast,
                dest: dest.split(", ").map(|s| s.to_string()).collect(),
            },
            [typeandname, _arrow, dest] => Module {
                name: typeandname[1..].to_string(),
                modtype: match typeandname.chars().next().unwrap() {
                    '%' => ModuleType::FlipFlop(OnOff::Off),
                    '&' => ModuleType::Conjunction(HashMap::new()),
                    _ => panic!("Invalid input: {}", typeandname),
                },
                dest: dest.split(", ").map(|s| s.to_string()).collect(),
            },
            _ => panic!("Invalid input: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum OnOff {
    On,
    Off,
}

impl OnOff {
    fn to_pulsetype(&self) -> PulseType {
        match self {
            OnOff::On => PulseType::High,
            OnOff::Off => PulseType::Low,
        }
    }

    fn flip(&self) -> Self {
        match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PulseType {
    Low,
    High,
}

impl PulseType {
    fn flip(&self) -> Self {
        match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

#[derive(Debug, Clone)]
struct Pulse {
    src: String,
    dest: String,
    pulse_type: PulseType,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(OnOff),
    Conjunction(HashMap<String, PulseType>),
}

fn parse(stdin: std::io::Stdin) -> Vec<Module> {
    let initial = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Module::from_string(&l))
        .collect_vec();

    // Fix the ModuleType for each Conjunction
    initial
        .iter()
        .map(|module| match &module.modtype {
            ModuleType::Conjunction(_) => {
                // Identify all inputs
                let input_modules = initial.iter().filter_map(|input_mod| {
                    if input_mod.dest.iter().any(|dest| *dest == module.name) {
                        Some((input_mod.name.to_owned(), PulseType::Low))
                    } else {
                        None
                    }
                });

                let modtype = ModuleType::Conjunction(input_modules.collect());

                Module {
                    modtype,
                    ..module.clone()
                }
            }
            _ => module.clone(),
        })
        .collect_vec()
}

fn process(modules: &mut Vec<Module>, all_pulses: &mut Vec<Pulse>) {
    let button_press = Pulse {
        src: "button".to_string(),
        dest: "broadcaster".to_string(),
        pulse_type: PulseType::Low,
    };

    let mut pulse_queue = VecDeque::<Pulse>::from([button_press]);

    while let Some(pulse) = pulse_queue.pop_front() {
        //dbg!(&pulse);

        all_pulses.push(pulse.clone());

        let current_mod = modules.iter_mut().find(|m| m.name == pulse.dest);
        if let None = current_mod {
            //println!("No module found with name {}", pulse.dest);
            continue;
        }

        let current_mod = current_mod.unwrap();

        match (&mut current_mod.modtype, &pulse.pulse_type) {
            (ModuleType::FlipFlop(_), PulseType::High) => (), // Ignored!

            (ModuleType::FlipFlop(flipflop_enabled), PulseType::Low) => {
                /* However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse. */

                // Flip it!
                *flipflop_enabled = flipflop_enabled.flip();

                current_mod.dest.iter().for_each(|dest| {
                    pulse_queue.push_back(Pulse {
                        src: current_mod.name.to_string(),
                        dest: dest.to_string(),
                        pulse_type: flipflop_enabled.to_pulsetype(),
                    })
                });
            }
            (ModuleType::Conjunction(m), pulsetype) => {
                // When a pulse is received, the conjunction module first updates its memory for that input.
                *m.get_mut(&pulse.src).unwrap() = pulsetype.clone();

                let remembers_high_pulses =
                    m.values().all(|pulsetype| *pulsetype == PulseType::High);

                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                let new_pulse = if remembers_high_pulses {
                    PulseType::Low
                } else {
                    PulseType::High
                };

                current_mod.dest.iter().for_each(|dest| {
                    pulse_queue.push_back(Pulse {
                        src: current_mod.name.to_string(),
                        dest: dest.to_string(),
                        pulse_type: new_pulse.clone(),
                    })
                });
            }
            (ModuleType::Broadcast, pulsetype) => {
                // When a pulse is received, the broadcaster module sends a pulse of the same type to all of its destinations.
                current_mod.dest.iter().for_each(|dest| {
                    pulse_queue.push_back(Pulse {
                        src: current_mod.name.to_string(),
                        dest: dest.to_string(),
                        pulse_type: pulsetype.clone(),
                    })
                });
            }
        }
    }
}

fn part1(modules: &Vec<Module>, n: usize) -> usize {
    let mut modules = modules.clone();

    let mut all_pulses = Vec::<Pulse>::new();

    for _ in 0..n {
        process(&mut modules, &mut all_pulses);
    }

    let low_pulses = all_pulses
        .iter()
        .filter(|p| p.pulse_type == PulseType::Low)
        .count();
    let high_pulses = all_pulses
        .iter()
        .filter(|p| p.pulse_type == PulseType::High)
        .count();

    println!("low_pulses: {}", low_pulses);
    println!("high_pulses: {}", high_pulses);

    low_pulses * high_pulses
}

fn part2(modules: &Vec<Module>) -> i32 {
    let mut modules = modules.clone();

    let mut all_pulses = Vec::<Pulse>::new();

    (0..)
        .find_map(|n| {
            process(&mut modules, &mut all_pulses);

            let num_rx_low = all_pulses.iter().filter(|p| p.dest == "rx" && p.pulse_type == PulseType::Low).count();
            
            //dbg!(&num_rx_low);

            if num_rx_low > 0 { Some(n) } else { None }
        })
        .unwrap()
}

fn main() {
    let modules = parse(std::io::stdin());
    dbg!(&modules);

    let p1 = part1(&modules, 1000);
    println!("p1: {p1}");

    let p2 = part2(&modules);
    println!("p2: {p2}");
}
