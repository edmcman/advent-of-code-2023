use std::collections::VecDeque;

use indicatif::ProgressIterator;

use memoize::memoize;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Config {
    conditions: Vec<char>,
    broken: Vec<usize>,
}

fn from_stdin(stdin: std::io::Stdin) -> impl Iterator<Item = Config> {
    stdin.lines().map(|l| l.unwrap()).map(|l| {
        match l.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
            [c, b] => Config {
                conditions: c.chars().collect(),
                broken: b.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
            },
            _ => panic!("Invalid input"),
        }
    })
}

impl Config {
    // ???.### 1,1,3 - 1 arrangement

    fn to_part2(&self) -> Self {
        let mut new_conditions = self.conditions.clone();
        new_conditions.push('?');
        let mut new_conditions: Vec<char> = std::iter::repeat(new_conditions)
            .take(4)
            .flatten()
            .collect();
        new_conditions.append(self.conditions.clone().as_mut());

        let new_broken: Vec<usize> = std::iter::repeat(self.broken.clone())
            .take(5)
            .flatten()
            .collect();

        Config {
            conditions: new_conditions,
            broken: new_broken,
        }
    }

    fn is_valid(&self) -> bool {
        // There has to be at least as many ? and # as broken things listed
        let need_broken = self.broken.iter().sum::<usize>();

        let have_broken = self
            .conditions
            .iter()
            .filter(|c| **c == '#' || **c == '?')
            .count();

        if need_broken > have_broken {
            return false;
        }

        let mut broken: VecDeque<usize> = self.broken.clone().into_iter().collect();
        //let mut i = 0;

        let mut last_broken = None;

        // Add a '.'?

        // Add a working spring at the end to force the last broken to be processed
        let mut conditions = self.conditions.clone();
        conditions.push('.');

        for c in conditions {
            match c {
                '?' => return true,
                '#' => {
                    last_broken = last_broken.map_or(Some(1), |l| Some(l + 1));
                }
                '.' => match (last_broken, broken.get(0)) {
                    (Some(l), Some(h)) if l == *h => {
                        broken.pop_front().unwrap();
                        last_broken = None;
                    }
                    (None, _) => (),
                    (Some(_), Some(_)) => return false,
                    (Some(_), None) => return false,
                },
                _ => panic!("Invalid input"),
            }
        }

        broken.is_empty()
    }

    fn expand(&self) -> Box<dyn Iterator<Item = Config> + '_> {
        match self
            .conditions
            .iter()
            .enumerate()
            .find_map(|(i, c)| if *c == '?' { Some(i) } else { None })
        {
            None => Box::new(vec![self.clone()].into_iter()),
            Some(i) => {
                let o = ['#', '.'].iter().map(move |c| {
                    let mut new = self.clone();
                    new.conditions[i] = *c;
                    new
                });

                Box::new(
                    // Early reject!
                    o.filter(|c| c.is_valid())
                        .map(|c| c.expand().collect::<Vec<_>>())
                        .flatten()
                        .filter(|c| c.is_valid()),
                )
            }
        }
    }

}

#[memoize]
fn rec_attempt(c: Config, num_broken_pre: usize) -> usize {

    //dbg!(&self.conditions);

    match c.conditions.first() {
        None => {
            match (c.broken.as_slice(), num_broken_pre) {
                ([x], y) if *x == y => 1,
                ([0], 0) => 1,
                ([], 0) => 1,
                _ => 0
            }
        },
        Some('#') => {
            let new_conditions = c.conditions[1..].to_vec();
            rec_attempt(Config{conditions: new_conditions, broken: c.broken.clone()}, num_broken_pre + 1)
        },
        Some('.') => match (c.broken.as_slice(), num_broken_pre) {
            ([x, tail @ ..], y) if *x == y => {

                // Cool, let's consume it and recurse.
                let new_broken = c.broken[1..].to_vec();
                let new_conditions = c.conditions[1..].to_vec();
                rec_attempt(Config{
                    conditions: new_conditions,
                    broken: new_broken
                }, 0)
            },
            (_, 0) => {
                // Don't consume but recurse
                let new_conditions = c.conditions[1..].to_vec();
                rec_attempt(Config {
                    conditions: new_conditions,
                    broken: c.broken.clone()
                }, 0)
            },
            _ => 0 // nope
        },
        Some('?') => {
            // Well, try each option.

            [".", "#"].iter().map(|s| {
                let new_conditions = s.chars().chain(c.conditions[1..].to_vec().into_iter()).collect::<Vec<_>>();
                rec_attempt(Config{conditions: new_conditions, broken: c.broken.clone()}, num_broken_pre)
            }).sum()
        },
        Some(_) => panic!("what")
    }
}


fn main() {
    let configs = from_stdin(std::io::stdin());
    let configs: Vec<_> = configs.collect();
    //dbg!(&c);

/*      let counts = configs
        .iter()
        .map(|c| c.expand().filter(|c| c.is_valid()).count())
        .progress()
        .collect::<Vec<_>>();
    println!("Part 1: {}", counts.iter().sum::<usize>());*/

    let counts_new = configs
        .iter()
        .map(|c| rec_attempt(c.clone(), 0))
        .progress()
        .collect::<Vec<_>>();

    dbg!(&counts_new);

    /*
    for c in configs {
        for e in c.expand() {
            if e.is_valid() {
                println!("{:?}", e);
            }
        }
    }
    */

    let p2_configs = configs.iter().map(|c| c.to_part2()).collect::<Vec<_>>();

    let p2_counts = p2_configs
        .iter()
        .map(|c| rec_attempt(c.clone(), 0))
        .progress()
        .collect::<Vec<_>>();

    /*let p2_counts = p2_configs
            .iter()
            .progress()
            .map(|c| c.expand().filter(|c| c.is_valid()).count())
            .collect::<Vec<_>>();
    */

    println!("Part 2: {}", p2_counts.iter().sum::<usize>());
}
