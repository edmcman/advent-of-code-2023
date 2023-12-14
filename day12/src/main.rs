use std::collections::VecDeque;

use indicatif::ProgressIterator;

#[derive(Debug, Clone)]
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

    fn test2(&self) -> usize {

        if self.conditions.len() == 0 {
            return if self.broken.is_empty() {1} else {0};
        }

        dbg!(&self.conditions, &self.broken);
        // So the idea is to assume that we're always starting at a new
        // contiguous segment of broken springs.

        // So we'll look and see where the next . is.  Let's say it's at index
        // i.  Every character in [0, i) is either a ? or #.  We'll consider
        // what would happen for each position that is a ?, since this could
        // terminate the contiguous chunk.

        let next_period =
            self.conditions
                .iter()
                .enumerate()
                //.skip(1) // is this right?
                .find_map(|(i, c)| if *c == '.' { Some(i) } else { None });

        let next_broken = self.broken.first();

        let period_index = next_period.unwrap_or(self.conditions.len()-1);

        println!("period_index is {period_index}. this means that we'll consider all the possibilities in 0..{period_index} and then recurse.");

        let mut choices = 1_usize;

        for (i, c) in self.conditions[0..=period_index].iter().enumerate() {

            // In this loop, we are now considering (i-1)*#.
            // We are now considering i-1*#

            dbg!(&i);

            //dbg!(i,c,choices);

            match *c {
                '#' => (), // It's not possible to end here.
                '.' => {
                    // We're ending here.  We better match!
                    // We have i-1 #.  
                    if i > 0 && Some(&(i-1)) == next_broken {
                        // OK... recurse on the remainder                        
                        let config = Config{
                            conditions: self.conditions[i+1..].to_vec(),
                            broken: self.broken[1..].to_vec() // tail
                        };
                        println!("recursing on the remainder {:?}", config);
                        choices *= config.test2();
                    } else {
                        println!("Um this is bad");
                        return 0
                    }
                }
                '?' => {

                    //dbg!(Some(&i), next_broken);

                    // What if it's a period? ###.
                    dbg!(i);
                    if (i > 0 && Some(&(i-1)) == next_broken) || i == 0 {
                        dbg!(i);
                        // i == 0 because we don't have any springs yet.
                        // It is possible for ? to be a .
                        // Recurse on [i..]
                        println!("We could break on a ? at {i}!");
                        println!("{} {period_index}", i+1);
                        if i+1 <= period_index {
                            let recurse_conditions = self.conditions[i+1..period_index].to_vec();
                            let broken = if i == 0 {
                                self.broken.clone()
                            } else {
                                self.broken[1..].to_vec()
                            };
                            let config = Config{
                                conditions: recurse_conditions,
                                broken // this is .tail
                            };
                            println!("add recursing on {:?}", config);
                            choices += dbg!(config.test2());
                        }
                    } else {
                        println!("Can't put . here at {i}");
                    }

                    // What if it's a #? Then we continue
                },
                _ => panic!("unknown char")
            }
        }

        choices
    }

    /*
    fn test(&self) -> usize {
        // Generalization:

        let first_question =
            self.conditions
                .iter()
                .enumerate()
                .find_map(|(i, c)| if *c == '?' { Some(i) } else { None });

        match (self.conditions.as_slice(), self.broken.as_slice()) {
            // We need to match at least one broken spring, and there are no places for them to go.
            ([], [_, ..]) => 0,
            (['.', ..], _) => {
                let new = Config {
                    conditions: self.conditions[1..].to_vec(),
                    broken: self.broken.clone()
                };
                new.test()
            },
            (['#', ..], [n, ..]) => {
                let am_i_new_contiguous =
            }
            _ => panic!("tbd")
        }
    }
    */
}

fn main() {
    let configs = from_stdin(std::io::stdin());
    let configs: Vec<_> = configs.collect();
    //dbg!(&c);

    let counts = configs
        .iter()
        .map(|c| c.expand().filter(|c| c.is_valid()).count())
        .progress()
        .collect::<Vec<_>>();
    println!("Part 1: {}", counts.iter().sum::<usize>());

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

    for c in configs.iter() {
        println!("hmm {}", c.test2());
    }

    /*let p2_counts = p2_configs
        .iter()
        .progress()
        .map(|c| c.expand().filter(|c| c.is_valid()).count())
        .collect::<Vec<_>>();
*/


    //println!("Part 2: {}", p2_counts.iter().sum::<usize>());
}
