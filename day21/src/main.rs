use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::directed::bfs::bfs_reach;

use indicatif::{self, ProgressIterator};

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Board {
    board: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Board {
    fn add_delta(
        &self,
        (x, y): (usize, usize),
        (dx, dy): (isize, isize),
    ) -> Option<(usize, usize)> {
        let (x, y) = (x as isize + dx, y as isize + dy);

        if x < 0 || y < 0 || x >= self.width() as isize || y >= self.height() as isize {
            return None;
        }

        let (x, y) = (x as usize, y as usize);

        Some((x, y))
    }

    fn add_delta_wrap(
        &self,
        (x, y): (isize, isize),
        (dx, dy): (isize, isize),
    ) -> Option<(isize, isize)> {
        let (x, y) = (x as isize + dx, y as isize + dy);

        Some((x, y))
    }

    fn get_row(&self, row: usize) -> &[char] {
        &self.board[self.board.len() - 1 - row]
    }

    fn get_col(&self, col: usize) -> Vec<char> {
        self.board.iter().map(|r| r[col]).collect_vec()
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        let y = self.board.len().checked_sub(1)?.checked_sub(y)?;
        self.board.get(y)?.get(x)
    }

    fn get_wrap(&self, x: isize, y: isize) -> Option<&char> {
        let (x, y) = (
            x.rem_euclid(self.width() as isize),
            y.rem_euclid(self.height() as isize),
        );

        let (x, y) = (x as usize, y as usize);

        let y = self.board.len().checked_sub(1)?.checked_sub(y)?;
        self.board.get(y)?.get(x)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        let y = self.board.len().checked_sub(1)?.checked_sub(y)?;
        self.board.get_mut(y)?.get_mut(x)
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn from_stdin(stdin: std::io::Stdin) -> Self {
        let board = stdin
            .lines()
            .map(|l| l.unwrap().chars().collect_vec())
            .collect_vec();

        let mut board = Self {
            board,
            start: (0, 0),
        };

        let start = board
            .iter_coords()
            .find(|(x, y)| board.get(*x, *y) == Some(&'S'))
            .unwrap();

        board.start = start;

        *board.get_mut(start.0, start.1).unwrap() = '.';

        board
    }

    fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "");
        self.board.iter().enumerate().for_each(|(i, row)| {
            writeln!(
                f,
                "{:2}  {}",
                row.len() - 1 - i,
                row.iter().cloned().collect::<String>()
            )
            .expect("Failed to write to stdout");
        });
        Ok(())
    }
}

fn print_board(p: &Vec<StateAndCost>) {
    let minx = p.iter().map(|st| st.x).min().unwrap();
    let maxx = p.iter().map(|st| st.x).max().unwrap();
    let miny = p.iter().map(|st| st.y).min().unwrap();
    let maxy = p.iter().map(|st| st.y).max().unwrap();

    println!("({minx},{miny}) ({maxx},{maxy})");

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if p.iter().any(|st| st.x == x && st.y == y) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!("");
    }

}

type Coord = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn to_deltas(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Debug)]
struct StateAndCost {
    x: isize,
    y: isize,
    cost: usize,
}

impl StateAndCost {
    fn successors(&self, b: &Board, steps: usize) -> Vec<StateAndCost> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter(|_| self.cost <= steps)
        .filter_map(|dir| {
            // find deltas for dir
            let (dx, dy) = dir.to_deltas();

            // add the delta
            let (newx, newy) = b.add_delta((self.x as usize, self.y as usize), (dx, dy))?;

            if b.get(newx, newy) == Some(&'#') {
                None
            } else {
                Some(StateAndCost {
                    x: newx as isize,
                    y: newy as isize,
                    cost: self.cost + 1,
                })
            }
        })
        .collect_vec()
    }

    fn successors_wrap(&self, b: &Board, steps: usize) -> Vec<StateAndCost> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter(|_| self.cost <= steps)
        .filter_map(|dir| {
            // find deltas for dir
            let (dx, dy) = dir.to_deltas();

            // add the delta
            let (newx, newy) = b.add_delta_wrap((self.x, self.y), (dx, dy))?;

            match b.get_wrap(newx, newy) {
                None => panic!("nah"),
                Some('#') => None,
                Some(_) => Some(StateAndCost {
                    x: newx,
                    y: newy,
                    cost: self.cost + 1,
                }),
            }
        })
        .collect_vec()
    }
}

fn part1(b: &Board, start: StateAndCost, steps: usize) -> usize {
    let mut current = vec![start];

    for i in 0..steps {
        //println!("{i}");

        current = current
            .iter()
            .flat_map(|st| st.successors(b, steps))
            .unique()
            .collect_vec();

        println!("{}", current.len());
    }

    current.len()
}

fn blah(b: &Board, start: StateAndCost, steps: usize) -> usize {
    let mut current = vec![start];

    for i in 0..steps {
        //println!("{i}");

        current = current
            .iter()
            .flat_map(|st| st.successors_wrap(b, steps))
            .unique()
            .collect_vec();

        //println!("round {i}");

        //print_board(&current);
    }

    let outside = current
        .iter()
        .filter(|st| {
            st.x < 0 || st.x > b.width() as isize || st.y < 0 || st.y > b.height() as isize
        })
        .collect_vec();
    /*if outside.len() > 0 {
        println!("outside: {}", outside.len());
        //dbg!(&outside);
    }*/

    current.len()
}

fn part2(b: &Board, steps: usize) -> usize {
    let start = StateAndCost {
        x: b.start.0 as isize,
        y: b.start.1 as isize,
        cost: 0,
    };

    let all_reach = bfs_reach(start, |st| st.successors_wrap(b, steps));

    let exact = all_reach.filter(|st| st.cost == steps);

    exact.count()

    //part2_memo(b, &start, *steps, &mut HashMap::new()).len()
}

fn part2_memo(
    b: &Board,
    st: &StateAndCost,
    total_steps: usize,
    memo: &mut HashMap<StateAndCost, Vec<StateAndCost>>,
) -> Vec<StateAndCost> {
    if let Some(r) = memo.get(st) {
        //dbg!("found memo");
        return r.to_vec();
    };

    let steps_remain = total_steps - st.cost;

    if steps_remain == 0 {
        return vec![st.clone()];
    };

    // Make one move and then recurse.
    let one_move = st.successors_wrap(b, total_steps);

    //dbg!(&one_move);

    let next_results = one_move
        .iter()
        .flat_map(|st| part2_memo(b, &st, total_steps, memo))
        .collect_vec();

    // remove duplicates
    let next_results = next_results.into_iter().unique().collect_vec();

    //dbg!(&next_results);

    //dbg!(&st);
    //dbg!(&next_results);
    memo.insert(st.clone(), next_results.clone());

    next_results
}

fn part2_cycle(b: &Board, start: StateAndCost) -> Vec<usize> {
    let mut seen = HashSet::<usize>::new();
    let mut halt = false;

    (1..)
        .map(|num_steps| part1(b, start.clone(), num_steps))
        .take_while(|num_states| {
            if halt {
                return false;
            };

            if seen.contains(num_states) {
                halt = true;
                return true;
            } else {
                seen.insert(*num_states);
            };

            true
        })
        .collect_vec()
}

fn main() {
    let board = Board::from_stdin(std::io::stdin());

    dbg!(&board);

    let start = StateAndCost {
        x: board.start.0 as isize,
        y: board.start.1 as isize,
        cost: 0,
    };

    let p1 = part1(&board, start.clone(), 64);
    println!("p1: {p1}");

    //blah(&board, start.clone(), 100);

    //blah(&board, start.clone(), 200);

    
    let hm = (0..10)
        .map(|n| 131*n + 65)
        //.map(|n| 2_usize.pow(n))
        .map(|n| (n, blah(&board, start.clone(), n)))
        .for_each(|(steps, n)| println!("{}: {}", steps, n));


    /*let test = part2_cycle(&board, start.clone());
    dbg!(&test);*/

    // ok this is too slow
    /*
        let board_test = board
            .iter_coords()
            .collect_vec()
            .into_iter()
            .progress()
            .map(|(x, y)| {
                let start = StateAndCost {
                    x: x as isize,
                    y: y as isize,
                    cost: 0,
                };
                part2_cycle(&board, start).last().unwrap().clone()
            })
            .collect_vec();

        dbg!(&board_test);
    */

    /*
    let child = std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        let p2 = (0..200).map(|i| (i,part2(&board, &i))).collect_vec();
        p2
    }).unwrap();
    let p2 = child.join().unwrap();
    //let p2 = part2(&board, &5000);

    let bigstr = p2.iter().map(|(i, n)| format!("{{{i},{n}}}")).join(",");
    println!("{{{bigstr}}}");

    p2.iter().for_each(|(i,n)| println!("{i} {n}"));

    //dbg!(&p2);
    //println!("p2: {:?}");
    */
}
