use std::collections::{HashSet, VecDeque, HashMap};

use indicatif::ProgressIterator;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    // x, y
    fn to_offset(&self) -> (isize, isize) {
        match self {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        }
    }

    fn rev(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/

impl PipeType {
    fn from_char(c: char) -> Self {
        match c {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            '.' => PipeType::Ground,
            'S' => PipeType::Start,
            _ => panic!("Unknown pipe type: {}", c),
        }
    }

    fn to_dirs(&self) -> Option<[Dir; 2]> {
        match self {
            PipeType::Vertical => Some([Dir::North, Dir::South]),
            PipeType::Horizontal => Some([Dir::East, Dir::West]),
            PipeType::NorthEast => Some([Dir::North, Dir::East]),
            PipeType::NorthWest => Some([Dir::North, Dir::West]),
            PipeType::SouthEast => Some([Dir::South, Dir::East]),
            PipeType::SouthWest => Some([Dir::South, Dir::West]),
            PipeType::Ground => None,
            PipeType::Start => None,
        }
    }

    fn from_dirs(din: &[&Dir; 2]) -> Self {
        //let mut d: [Dir; 2] = (*din).clone();
        let mut d = din.clone();
        d.sort();

        match d {
            [Dir::North, Dir::South] => PipeType::Vertical,
            [Dir::East, Dir::West] => PipeType::Horizontal,
            [Dir::North, Dir::East] => PipeType::NorthEast,
            [Dir::North, Dir::West] => PipeType::NorthWest,
            [Dir::East, Dir::South] => PipeType::SouthEast,
            [Dir::South, Dir::West] => PipeType::SouthWest,
            _ => panic!("Unknown pipe type: {:?}", d),
        }
    }
}

#[derive(Debug)]
struct Board {
    start: (usize, usize),
    board: Vec<Vec<PipeType>>,
}

impl Board {

    fn print(&self) {
        self.board.iter().for_each(|row| {
            row.iter().for_each(|pipe| {
                match pipe {
                    PipeType::Vertical => print!("|"),
                    PipeType::Horizontal => print!("-"),
                    PipeType::NorthEast => print!("L"),
                    PipeType::NorthWest => print!("J"),
                    PipeType::SouthEast => print!("F"),
                    PipeType::SouthWest => print!("7"),
                    PipeType::Ground => print!("."),
                    PipeType::Start => print!("S"),
                }
            });
            println!("");
        });
    }

    fn neighbors(
        &self,
        loc: (usize, usize),
    ) -> impl Iterator<Item = (Dir, (usize, usize), &PipeType)> {
        let (sx, sy) = loc;
        let (sx, sy) = (sx as isize, sy as isize);

        //dbg!(&loc);

        self.board
            .get(sy as usize)
            .unwrap()
            .get(sx as usize)
            .unwrap()
            .to_dirs()
            .unwrap()
            .into_iter()
            .filter_map(move |d| {
                let (dx, dy) = d.to_offset();

                let (x, y) = (sx + dx, sy + dy);
                let (x, y) = (
                    x.try_into() as Result<usize, _>,
                    y.try_into() as Result<usize, _>,
                );

                /*y.map(|y| self.board.get(y).unwrap())
                .and_then(|row| x.map(|x| (d, (x, y.unwrap()), row.get(x).unwrap())))
                .ok()*/

                y.ok()
                    .and_then(|y| self.board.get(y))
                    .and_then(|row| x.ok().and_then(|x| Some((d, (x, y.unwrap()), row.get(x)?))))
            })
    }

    fn from_stdin(stdin: std::io::Stdin) -> Board {
        let board = stdin
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| PipeType::from_char(c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let start = board
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, pipe)| {
                    if let PipeType::Start = pipe {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        // What type of pipe is the start?
        // XXX: Blegh, should use Board::neighbors
        let start_neighbors = [Dir::North, Dir::East, Dir::South, Dir::West]
            .iter()
            .filter_map(|d| {
                let (sx, sy) = start;
                let (sx, sy) = (sx as isize, sy as isize);

                let (dx, dy) = d.to_offset();

                let (x, y) = (sx + dx, sy + dy);
                let (x, y) = (
                    x.try_into() as Result<usize, _>,
                    y.try_into() as Result<usize, _>,
                );

                y.map(|y| board.get(y).unwrap())
                    .and_then(|row| x.map(|x| (d, row.get(x).unwrap())))
                    .ok()
            });

        //let start_neighbors = neighbors(start)

        dbg!(&start_neighbors);

        // Which neighbors have pipes to us?
        let pipe_dirs = start_neighbors.flat_map(|(neighbor_dir, neighbor_pt)| {
            // Does the neighbor in direction dir point back to us?  If so, return its direction.

            dbg!(neighbor_dir, neighbor_pt);

            let a = neighbor_pt.to_dirs().iter().find_map(move |dirs| {
                dbg!(dirs.contains(&neighbor_dir.rev())).then_some(neighbor_dir)
            });
            a
        });

        let pipe_dirs: [&Dir; 2] = dbg!(pipe_dirs.collect::<Vec<_>>())
            .as_slice()
            .try_into()
            .unwrap();

        let start_pipe = PipeType::from_dirs(&pipe_dirs);

        dbg!(&start_pipe);

        // Change the pipetype of th start position.  Bleh.
        let board = board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|pipe| {
                        if let PipeType::Start = pipe {
                            start_pipe.clone()
                        } else {
                            pipe
                        }
                    })
                    .collect()
            })
            .collect();

        Board { start, board }
    }
}

fn part1(b: &Board) -> (usize, HashSet<(usize, usize)>) {
    let mut queue = VecDeque::new();
    queue.push_back((b.start, 0_usize));

    let mut max = 0;

    let mut seen = HashSet::new();

    while let Some((pt, steps)) = queue.pop_front() {
        dbg!("loop", &pt, &steps);

        if !seen.insert(pt) {
            continue;
        }

        max = *[max, steps].iter().max().unwrap();

        let neighbors = b.neighbors(pt);

        neighbors.for_each(|(dir, loc, pipetype)| {
            if !seen.contains(&loc) {
                queue.push_back((loc, steps + 1));
            }
        });
    }

    (max, seen)
}

impl PipeType {
    fn explode(&self) -> Vec<Vec<bool>> {
        match self {
            Self::Horizontal => [
                [false, false, false],
                [true, true, true],
                [false, false, false],
            ],
            Self::Vertical => [
                [false, true, false],
                [false, true, false],
                [false, true, false],
            ],
            Self::NorthEast => [
                [false, true, false],
                [false, true, true],
                [false, false, false],
            ],
            Self::NorthWest => [
                [false, true, false],
                [true, true, false],
                [false, false, false],
            ],
            Self::SouthEast => [
                [false, false, false],
                [false, true, true],
                [false, true, false],
            ],
            Self::SouthWest => [
                [false, false, false],
                [true, true, false],
                [false, true, false],
            ],
            Self::Ground => [
                [false, false, false],
                [false, false, false],
                [false, false, false],
            ],
            Self::Start => [
                [false, false, false],
                [false, false, false],
                [false, false, false],
            ],
        }
        .map(|row| row.to_vec())
        .to_vec()
    }
}

fn part2_loc(
    g: &Vec<Vec<bool>>,
    loc: (usize, usize),
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    //let (x, y) = loc;

    let mut reach = HashSet::new();
    let mut queue = Vec::new();
    queue.push(loc);

    while let Some((x, y)) = queue.pop() {

        if cache.contains_key(&(x, y)) {
            return cache[&(x,y)];
        }

        if reach.contains(&(x, y)) {
            continue;
        }

        if g[y][x] {
            // In a pipe.
            continue;
        }

        reach.insert((x, y));



        //println!("We reached {:?} from {:?}", (x,y), loc);
        // From loc we can reach (x,y).  Add it to the set.

        let (x, y) = (x as isize, y as isize);

        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .map(|(x, y)| {
                (
                    (*x).try_into() as Result<usize, _>,
                    (*y).try_into() as Result<usize, _>,
                )
            })
            .map(|(x, y)| (x.ok(), y.ok()))
            .filter_map(|(x, y)| Some((x?, y?)))
            .filter(|(x, y)| *y < g.len() && *x < g[0].len())
            .filter(|(x, y)| !g[*y][*x])
            .collect::<Vec<_>>();

        //dbg!(&neighbors);

        neighbors
            .into_iter()
            .filter(|(x, y)| !reach.contains(&(*x, *y)))
            .for_each(|(x, y)| {

                queue.push((x, y));
            });
    }

    // Location is in the loop if it can't access the last row (which we added)

    // If we can reach 0, last_row, then we're outside the loop.
    let is_outside = reach.contains(&(0, g.len() - 1));
    // If we aren't outside the loop, we are in the loop
    let is_in_loop = !is_outside;

    // Update the cache
    reach.iter().for_each(|reachable_loc| {
        cache.insert(*reachable_loc, is_in_loop);
    });

    is_in_loop
}

fn part2(b: &Board, locs: &HashSet<(usize, usize)>) -> usize {
    // Let's map each original tile to a 3x3 grid of tiles.

    let exploded_grid = b
        .board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, pipe)| {
                    if locs.contains(&(x, y)) {
                        pipe.explode()
                    } else {
                        vec![vec![false; 3]; 3]
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Thank you ChatGPT.  Ugh.
    let mut flattened_grid: Vec<Vec<bool>> = exploded_grid
        .into_iter()
        .flat_map(|expanded_row| {
            // Create iterators for each sub-row in the 3x3 grids
            let mut sub_row_iters = expanded_row
                .into_iter()
                .map(|cell| cell.into_iter())
                .collect::<Vec<_>>();

            // For each set of 3 rows, concatenate them side by side
            (0..3).map(move |_| {
                sub_row_iters
                    .iter_mut()
                    .flat_map(|sub_row_iter| sub_row_iter.next().unwrap())
                    .collect::<Vec<bool>>()
            })
        })
        .collect();

    flattened_grid.push(vec![false; flattened_grid[0].len()]);

    flattened_grid.iter().for_each(|row| {
        row.iter().for_each(|b| {
            if *b {
                print!("X");
            } else {
                print!("_");
            }
        });
        println!("");
    });

    let mut cache = HashMap::new();

    //let mut outside_reach_cache = HashSet::new();

    let inside = flattened_grid
        .iter()
        .progress()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .flat_map(|(x, _b)| {
                    
                    /*
                    println!("Printing {:?}", (x,y));

                    flattened_grid.iter().enumerate().for_each(|(y, row)| {
                        print!("{:02} ", y);
                        row.iter().enumerate().for_each(|(x, b)| {
                            if outside_reach_cache.contains(&(x,y)) {
                                if flattened_grid[y][x] {
                                    print!("0");
                                } else {
                                    print!("O");
                                }
                                assert!(outside_reach_cache.len() > 0);
                            }
                            else if *b {
                                print!(".");
                            } else {
                                print!("%");
                            }
                        });
                        println!("");
                    });

                    println!("");
                    println!("");
                    println!("");
                    */

                    //println!("About to run {:?} {}", (x,y), outside_reach_cache.len());
                    

                    if part2_loc(&flattened_grid, (x, y), &mut cache) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    //println!("Inside explode: {:?}", inside);
    //dbg!(&inside);

    // Convert the exploded coords back to normal coordinates
    let inside = inside
        .iter()
        .map(|(x, y)| (x / 3, y / 3))
        .collect::<Vec<_>>();

    let mut inside_counter = HashMap::new();

    for v in &inside {
        let count = inside_counter.entry(v).or_insert(0);
        *count += 1;
    }

    //println!("Inside counter: {:?}", inside_counter);

    let full = inside_counter
        .iter()
        .filter(|(_k, v)| **v == 3*3)
        .map(|(k, _v)| k)
        .collect::<Vec<_>>();


    // We only care about Ground locations
    /*
    let inside = inside
        .iter()
        .filter(|(x, y)| *b.board.get(*y).unwrap().get(*x).unwrap() == PipeType::Ground)
        .collect::<HashSet<_>>();
    */

    println!("Full: {:?}", full);

    full.len()
}

fn main() {
    let board = Board::from_stdin(std::io::stdin());

    println!("{:?}", board);

    let (p1, locs) = part1(&board);

    println!("Part 1: {}", p1);

    println!("Part 2: {}", part2(&board, &locs));
}
