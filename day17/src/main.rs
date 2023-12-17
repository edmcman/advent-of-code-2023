//use std::collections::HashSet;
//use rayon::prelude::*;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Board {
    board: Vec<Vec<char>>,
}

/*#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct BeamState {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}*/

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
}

/*
impl BeamState {
    fn move_next(&self, board: &Board) -> Option<Self> {
        add_delta((self.x, self.y), (self.dx, self.dy))
            .map(|(newx, newy)| Self {
                x: newx,
                y: newy,
                dx: self.dx,
                dy: self.dy,
            })
            .filter(|s| s.x < board.width() && s.y < board.height())
    }

    // Next states
    fn next(&self, board: &Board) -> Vec<Self> {
        match board.get(self.x, self.y) {
            None => vec![],
            // empty space
            Some('.') => vec![self.move_next(board)]
                .into_iter()
                .flatten()
                .collect_vec(),
            // splitters acting as empty space
            Some('-') if self.dx != 0 => vec![self.move_next(board)]
                .into_iter()
                .flatten()
                .collect_vec(),
            Some('|') if self.dy != 0 => vec![self.move_next(board)]
                .into_iter()
                .flatten()
                .collect_vec(),
            // mirror
            Some(c @ ('/' | '\\')) => {
                let (dx, dy) = match c {
                    // If we were going right (+dx), go up (+dy)
                    '/' => (self.dy, self.dx),
                    // If we were going right (+dx), go down (-dy)
                    '\\' => (-self.dy, -self.dx),
                    _ => unreachable!(),
                };
                vec![Self { dx, dy, ..*self }.move_next(board)]
                    .into_iter()
                    .flatten()
                    .collect_vec()
            }
            // splitter splitting
            Some('-') if self.dy != 0 => vec![
                Self {
                    dx: -1,
                    dy: 0,
                    ..*self
                },
                Self {
                    dx: 1,
                    dy: 0,
                    ..*self
                },
            ],
            Some('|') if self.dx != 0 => vec![
                Self {
                    dx: 0,
                    dy: -1,
                    ..*self
                },
                Self {
                    dx: 0,
                    dy: 1,
                    ..*self
                },
            ],
            Some(_) => panic!("oops"),
        }
    }
}
*/

impl Board {
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

        Self { board }
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
struct State {
    x: usize,
    y: usize,
    dir: Direction,
}

impl State {
    fn successors(&self, b: &Board, r: &std::ops::RangeInclusive<isize>) -> impl Iterator<Item = StateAndCost> {
        let (dx, dy) = self.dir.to_deltas();

        let o = r.clone()
            .filter_map(move |dist| {
                let (newx, newy) = b.add_delta((self.x, self.y), (dx * dist, dy * dist))?;

                let costs = (1..=dist)
                    .map(|dist| {
                        let (newx, newy) = b.add_delta((self.x, self.y), (dx * dist, dy * dist))?;
                        b.get(newx, newy)
                    })
                    .collect_vec();

                let cost = if costs.iter().all(|c| c.is_some()) {
                    Some(
                        costs
                            .iter()
                            .map(|c| c.unwrap().to_digit(10).unwrap())
                            .sum::<u32>() as usize,
                    )
                } else {
                    None
                }?;

                Some(
                    [self.dir.turn_left(), self.dir.turn_right()]
                        .into_iter()
                        .map(move |newdir| {
                            (
                                State {
                                    x: newx,
                                    y: newy,
                                    dir: newdir,
                                },
                                cost,
                            )
                        }),
                )
            })
            .flatten()
            .collect_vec();

        //dbg!(&self, &o);

        o.into_iter()
    }

}

type StateAndCost = (State, usize);

fn part1(b: &Board, r: std::ops::RangeInclusive<isize>) -> usize {
    let start1 = State {
        x: 0,
        y: b.height() - 1,
        dir: Direction::East,
    };

    let start2 = State {
        dir: Direction::South,
        ..start1
    };

    [start1, start2]
        .iter()
        .filter_map(|start| {
            dijkstra(
                start,
                |n| n.successors(b, &r),
                |n| n.x == b.width() - 1 && n.y == 0,
            )
        })
        .map(|t| t.1)
        .min()
        .unwrap()
}

fn main() {
    let b = Board::from_stdin(std::io::stdin());
    dbg!(&b);

    let p1 = part1(&b, 1..=3);

    println!("Part 1: {p1}");

    let p2 = part1(&b, 4..=10);

    println!("Part 2: {p2}");
}
