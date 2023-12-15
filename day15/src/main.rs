use std::{collections::VecDeque, hint};

fn parse(stdin: std::io::Stdin) -> Vec<String> {
    stdin
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|l| l.split(",").map(|s| String::from(s)).collect::<Vec<_>>())
        .collect()
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |current_value, new_value| {
            ((current_value + new_value) * 17) % 256
        })
}

// Lens is a label and focal length.
type Lens = (String, usize);

struct State {
    map: std::collections::HashMap<usize, Vec<Lens>>,
}

fn part2(strings: &Vec<String>) -> usize {
    let final_state = strings.iter().fold(
        State {
            map: std::collections::HashMap::new(),
        },
        |mut state, s| match s.split_once('=') {
            Some((new_label, new_fl)) => {
                let boxnum = hash(new_label);
                let new_fl = new_fl.parse::<usize>().unwrap();
                let m = state.map.entry(boxnum).or_insert(Vec::new());

                let existing_lens = m.iter_mut().find_map(|(old_label, old_fl)| {
                    if new_label == old_label {
                        Some(old_fl)
                    } else {
                        None
                    }
                });

                // Update the map
                match existing_lens {
                    Some(existing_lens) => {
                        *existing_lens = new_fl;
                    }
                    None => {
                        m.push((new_label.to_owned(), new_fl));
                    }
                };

                state
            }
            None => {
                //dbg!(&s);
                assert!(s.ends_with("-"));
                let old_label = s.trim_end_matches('-');
                let boxnum = hash(old_label);
                let m = state.map.entry(boxnum).or_insert(Vec::new());
                m.retain(|(l, _)| l != &old_label);
                state
            }
        },
    );

    final_state
        .map
        .iter()
        .map(|(boxnum, v)| {
            v.iter()
                .enumerate()
                .map(|(slotnum, (_label, fl))| (boxnum + 1) * (slotnum + 1) * fl)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let strings = parse(std::io::stdin());

    //dbg!(&strings);

    //dbg!(&strings.iter().map(|s| hash(s)).collect::<Vec<_>>());

    let p1 = strings.iter().map(|s| hash(s)).sum::<usize>();

    println!("P1: {p1}");

    let p2 = part2(&strings);

    println!("P2: {p2}");
}
