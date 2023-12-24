use itertools::Itertools;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn to_deltas(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn from_char(c: &char) -> Self {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("bad dir"),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
struct Board {
    grid: Vec<Vec<char>>,
}

impl Board {
    fn iter_grid(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..=self.max_row()).flat_map(move |y| (0..=self.max_col()).map(move |x| (x, y)))
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            '.' | '>' | '<' | 'v' | '^' => true,
            '#' => false,
            _ => panic!("bad char"),
        }
    }

    fn from_stdin(stdin: std::io::Stdin) -> Self {
        let v = stdin
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.chars().collect())
            .collect();

        Board { grid: v }
    }

    fn max_col(&self) -> usize {
        self.grid[0].len() - 1
    }

    fn max_row(&self) -> usize {
        self.grid.len() - 1
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn add_delta(
        &self,
        (x, y): (usize, usize),
        (dx, dy): (isize, isize),
    ) -> Option<(usize, usize)> {
        let (x, y) = (x as isize + dx, y as isize + dy);

        let x: usize = x.try_into().ok()?;
        let y = y.try_into().ok()?;

        if x > self.max_col() || y > self.max_row() {
            return None;
        }

        Some((x, y))
    }

    fn neighbors(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter_map(|dir| self.add_delta(coord, dir.to_deltas()))
        .collect_vec()
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().for_each(|row| {
            writeln!(f, "{}", row.iter().cloned().collect::<String>())
                .expect("Failed to write to stdout");
        });
        Ok(())
    }
}

fn vis(board: &Board, path: &Vec<(usize, usize)>) {
    let mut board = board.clone();
    for (x, y) in path {
        board.grid[*y][*x] = 'O';
    }
    println!("{:?}", &board);
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Debug)]
struct State {
    x: usize,
    y: usize,
    path: Vec<(usize, usize)>,
}

impl State {
    fn succ_cost(&self, board: &Board) -> Vec<(State, usize)> {
        self.succ(board).into_iter().map(|s| (s, 1)).collect_vec()
    }

    fn succ(&self, board: &Board) -> Vec<State> {
        match board.get(self.x, self.y) {
            '.' => {
                // can go in any direction
                let neighbors = board.neighbors((self.x, self.y));
                neighbors
                    .iter()
                    // first make sure we don't repeat a step
                    .filter(|(x, y)| !self.path.contains(&(*x, *y)))
                    // don't go into trees
                    .filter_map(|(x, y)| {
                        if board.is_valid(*x, *y) {
                            let mut new_path = self.path.clone();
                            new_path.push((*x, *y));
                            Some(State {
                                x: *x,
                                y: *y,
                                path: new_path,
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            c @ ('<' | '>' | '^' | 'v') => {
                let dir = Direction::from_char(&c);
                let delta = dir.to_deltas();
                let new_coord = board.add_delta((self.x, self.y), delta);
                match new_coord {
                    Some((x, y)) => {
                        if board.is_valid(x, y) && !self.path.contains(&(x, y)) {
                            let mut new_path = self.path.clone();
                            new_path.push((x, y));
                            vec![State {
                                x,
                                y,
                                path: new_path,
                            }]
                        } else {
                            vec![]
                        }
                    }
                    None => vec![],
                }
            }
            _ => todo!("todo"),
        }
    }
}

// don't worry about cycles
fn simple_succ(board: &Board, t: (usize, usize)) -> Vec<(usize, usize)> {
    let st = State {
        x: t.0,
        y: t.1,
        path: vec![],
    };

    let o = st.succ(board).into_iter().map(|s| (s.x, s.y)).collect_vec();

    //println!("succ({:?}) = {:?}", &t, &o);

    o
}

fn search_for_longest(
    board: &Board,
    path: Vec<(usize, usize)>,
    start: &(usize, usize),
    goal: &(usize, usize),
) -> Option<usize> {
    if start == goal {
        return Some(0);
    }

    //println!("{:?}", &path);

    let mut new_path = path.clone();
    new_path.push(*start);

    let succ = simple_succ(board, *start);
    let best = succ
        .iter()
        .filter(|p| !path.contains(p))
        .filter_map(|p| Some((p, search_for_longest(board, new_path.clone(), &p, goal)?)))
        .max_by_key(|(_, len)| *len)?;

    Some(best.1 + 1)
}

fn p1(board: &Board) -> usize {
    //let start = board.iter_grid().find(|(x, y)| board.get(*x, *y) == 'S').unwrap();
    let start = State {
        x: 1,
        y: 0,
        path: vec![],
    };

    let bfs_reach = pathfinding::directed::bfs::bfs_reach(start, |s| s.succ(board));

    //bfs_reach.for_each(|s| println!("reachable: {:?}", &s));
    let longest = bfs_reach.max_by_key(|s| s.path.len()).unwrap();

    println!("longest: {:?}", &longest);

    longest.path.len()
}

fn p2(board: &Board) -> usize {
    let no_slopes = board
        .grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    '<' | '>' | 'v' | '^' => '.',
                    c => *c,
                    _ => panic!("bad char"),
                })
                .collect_vec()
        })
        .collect_vec();

    let board = Board { grid: no_slopes };

    dbg!(&board);

    // Rust doesn't let you change the main thread's stack size, grrr.

    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024); // 32 MB stack size

    let handler = builder.spawn(move || {
        search_for_longest(&board, vec![], &(1, 0), &(board.max_col() - 1, board.max_row() - 1)).unwrap() + 1
        // Your recursive function or code with high stack demands
    }).unwrap();

    handler.join().unwrap()

    //search_for_longest(&board, vec![], &(1, 0), &(board.max_col() - 1, board.max_row() - 1)).unwrap() + 1
   
}

fn main() {
    let board = Board::from_stdin(std::io::stdin());
    dbg!(&board);

/*    let p1 = p1(&board);
    println! {"p1: {p1}"};
*/
    let p2 = p2(&board);
    println!("p2: {p2}");
}
