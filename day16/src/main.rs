use std::collections::HashSet;
use rayon::prelude::*;
use itertools::Itertools;

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Board {
    board: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct BeamState {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

fn add_delta((x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
    let (x, y) = (x as isize + dx, y as isize + dy);

    if x < 0 || y < 0 {
        return None;
    }

    let (x, y) = (x as usize, y as usize);

    Some((x, y))
}

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

fn num_energized(board: &Board, states: &mut Vec<BeamState>) -> usize {
    let mut seen = std::collections::HashSet::new();

    while let Some(state) = states.pop() {
        if !seen.insert(state.clone()) {
            continue;
        }

        let next_states = state.next(board);

        //println!("state: {:?} new_states {:?}", state, next_states);

        states.extend(next_states);
    }

    //dbg!(&seen);

    seen.into_iter()
        .map(|s| (s.x, s.y))
        .collect::<HashSet<_>>()
        .len()
}

fn part1(board: &Board) -> usize {
    num_energized(
        board,
        &mut vec![BeamState {
            x: 0,
            y: board.height() - 1,
            dx: 1,
            dy: 0,
        }],
    )
}

// I could memoize this, but I don't feel like it.  I'll just use rayon instead, haha.
fn part2(board: &Board) -> usize {
    let top_row = board.height() - 1;
    let bottom_row = 0;
    let left_row = 0;
    let right_row = board.width() - 1;

    let states = (0..board.width()).flat_map(|x| {
        (0..board.height()).flat_map(move |y| {

            vec![
                // top edge
                BeamState {
                    x,
                    y: top_row,
                    dx: 0,
                    dy: -1,
                },
                // bottom edge
                BeamState {
                    x,
                    y: bottom_row,
                    dx: 0,
                    dy: 1,
                },
                // left edge
                BeamState {
                    x: left_row,
                    y,
                    dx: 1,
                    dy: 0,
                },
                // right edge
                BeamState {
                    x: right_row,
                    y,
                    dx: -1,
                    dy: 0,
                }]
        })
    }).collect_vec();

    states.into_par_iter().map(|s| num_energized(board, &mut vec![s])).max().unwrap()

}

fn main() {
    let b = Board::from_stdin(std::io::stdin());
    dbg!(&b);

    let p1 = part1(&b);

    println!("Part 1: {}", p1);

    let p2 = part2(&b);

    println!("Part 2: {}", p2);
}
