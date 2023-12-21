use itertools::Itertools;
use pathfinding::directed::bfs::bfs_reach;

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
    //dir: Direction,
}

impl State {
    /*
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
    */
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Debug)]
struct StateAndCost {
    x: usize,
    y: usize,
    cost: usize,
}

impl StateAndCost {
    fn successors(&self, b: &Board, steps: &usize) -> Vec<StateAndCost> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter(|_| self.cost <= *steps)
        .filter_map(|dir| {
            // find deltas for dir
            let (dx, dy) = dir.to_deltas();

            // add the delta
            let (newx, newy) = b.add_delta((self.x, self.y), (dx, dy))?;

            if b.get(newx, newy) == Some(&'#') {
                None
            } else {
                Some(StateAndCost {
                    x: newx,
                    y: newy,
                    cost: self.cost + 1,
                })
            }
        })
        .collect_vec()
    }
}

fn part1(b: &Board, steps: &usize) -> usize {
    let start = StateAndCost {
        x: b.start.0,
        y: b.start.1,
        cost: 0,
    };

    let all_reach = bfs_reach(start, |st| st.successors(b, steps));

    let exact = all_reach.filter(|st| st.cost == *steps);

    exact.count()
}
fn main() {
    let board = Board::from_stdin(std::io::stdin());

    dbg!(&board);

    let p1 = part1(&board, &64);
    println!("p1: {p1}");
}
