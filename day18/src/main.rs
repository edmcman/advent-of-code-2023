use geo::{point, Area, Contains, LineString, Polygon};
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Board {
    board: Vec<Vec<char>>,
    min_row: isize,
    min_col: isize,
}

impl Board {
    fn add_delta(
        &self,
        (x, y): (isize, isize),
        (dx, dy): (isize, isize),
    ) -> Option<(isize, isize)> {
        let (x, y) = (x as isize + dx, y as isize + dy);

        if x < self.min_col() || y < self.min_row() || x > self.max_col() || y > self.max_row() {
            return None;
        }

        Some((x, y))
    }
}

impl Board {
    fn new() -> Self {
        Board {
            board: vec![vec![' ']],
            min_row: 0,
            min_col: 0,
        }
    }

    fn neighbors(&self, coord: (isize, isize)) -> Vec<(isize, isize)> {
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

    fn ensure_row(&mut self, row: isize) {
        if row < self.min_row() {
            let new_row = vec!['.'; self.width()];
            let num_rows = self.min_row - row;
            for _ in 0..num_rows {
                self.board.insert(0, new_row.clone());
            }
            self.min_row = row;
        } else if row > self.max_row() {
            let new_row = vec!['.'; self.width()];
            let num_rows = row - self.max_row();
            for _ in 0..num_rows {
                self.board.push(new_row.clone());
            }
        }
        assert!(row >= self.min_row());
        assert!(row <= self.max_row());
    }

    fn ensure_col(&mut self, col: isize) {
        if col < self.min_col() {
            let num_cols = self.min_col - col;
            for row in self.board.iter_mut() {
                for _ in 0..num_cols {
                    row.insert(0, '.');
                }
            }
            self.min_col = col;
        } else if col > self.max_col() {
            let num_cols = col - self.max_col();
            for row in self.board.iter_mut() {
                for _ in 0..num_cols {
                    row.push('.');
                }
            }
        }
        assert!(col >= self.min_col());
        assert!(col <= self.max_col());
    }

    fn get_row_mut(&mut self, row: isize) -> &mut [char] {
        self.ensure_row(row);
        let adjusted_row = row - self.min_row();
        self.board.get_mut(adjusted_row as usize).unwrap()
    }

    fn get(&mut self, x: isize, y: isize) -> &mut char {
        self.ensure_row(y);
        self.ensure_col(x);

        let offset = x - self.min_col();

        let row = self.get_row_mut(y);

        row.get_mut(offset as usize).unwrap()
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn min_row(&self) -> isize {
        self.min_row
    }

    fn max_row(&self) -> isize {
        self.min_row + self.height() as isize - 1
    }

    fn min_col(&self) -> isize {
        self.min_col
    }

    fn max_col(&self) -> isize {
        self.min_col + self.width() as isize - 1
    }

    fn iter_coords(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        (self.min_row()..=self.max_row())
            .flat_map(move |y| (self.min_col()..=self.max_col()).map(move |x| (x, y)))
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "");
        self.board.iter().enumerate().for_each(|(i, row)| {
            writeln!(
                f,
                "{:2}  {}",
                self.max_row() - i as isize,
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

#[derive(Debug, Clone)]
struct PlanEntry {
    dir: Direction,
    number: usize,
    color: String,
}

type Plans = Vec<PlanEntry>;

impl PlanEntry {
    fn from_string(s: &str) -> Self {
        let re = Regex::new(r"([A-Z]) (\d+) \(#(\w{6})\)").unwrap();
        re.captures(s)
            .map(|caps| {
                let letter = match caps.get(1).unwrap().as_str().chars().next().unwrap() {
                    'L' => Direction::West,
                    'U' => Direction::North,
                    'R' => Direction::East,
                    'D' => Direction::South,
                    _ => panic!("invalid dir char"),
                };
                let number = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let color_code = caps.get(3).unwrap().as_str().to_string();
                PlanEntry {
                    dir: letter,
                    number: number.try_into().unwrap(),
                    color: color_code,
                }
            })
            .unwrap()
    }
}

fn parse_stdin(stdin: std::io::Stdin) -> Plans {
    stdin
        .lines()
        .map(|l| PlanEntry::from_string(&l.unwrap()))
        .collect_vec()
}

fn part(plans: &Plans) -> usize {
    let mut board = Board::new();

    let start = board.get(0, 0);
    *start = '#';

    let start_coord = (0, 0);

    let coords = plans.iter().fold(vec![start_coord], |mut coords, plan| {
        //dbg!(&board);
        //dbg!(&coord);

        let (dx, dy) = plan.dir.to_deltas();
        let (x, y) = coords.last().unwrap();

        coords.push((x + plan.number as isize * dx, y + plan.number as isize * dy));

        coords
    });

    let poly = Polygon::new(
        LineString::from(
            coords
                .into_iter()
                .map(|(x, y)| (x as f64, y as f64))
                .collect_vec(),
        ),
        vec![],
    );

    //dbg!(&poly);

    //let point = point! {x:1.0, y:-1.0};

    //assert!(poly.contains(&point));

    /*let inner_points = poly.points.iter().cycle().tuple_windows().take(poly.points.len() + 2).map(|(a, b, c)| {

    }).collect_vec();*/

    let outer_points = poly
        .exterior()
        .coords()
        .map(|p| {
            // Each point should be a turn.  So exactly one corner should be inside the polygon.

            let corner_points = [0.5, -0.5]
                .iter()
                .flat_map(|dx| {
                    [0.5, -0.5].iter().map(move |dy| {
                        let x = p.x + dx;
                        let y = p.y + dy;

                        let point = point! {x:x,y:y};

                        //dbg!(&point);

                        point
                    })
                })
                .collect_vec();

            let inside_points = corner_points
                .iter()
                .filter(|p| poly.contains(*p))
                .collect_vec();
            let outside_points = corner_points
                .iter()
                .filter(|p| !poly.contains(*p))
                .collect_vec();

            //.filter(|p| poly.contains(p))
            //.collect_tuple::<(Point,)>()
            //.expect("Expected exactly one corner inside").0;
            //.collect_vec();

            let the_corner = match (outside_points.as_slice(), inside_points.as_slice()) {
                ([inside], _) => **inside,
                (_, [outside]) => {
                    let dx = outside.x() - p.x;
                    let dy = outside.y() - p.y;
                    point! {x: p.x - dx, y: p.y - dy}
                }
                _ => panic!(
                    "Expected exactly one corner {:?} {:?}",
                    inside_points, outside_points
                ),
            };

            //dbg!(&the_corner);

            the_corner
        })
        .collect_vec();

    let outer_poly = Polygon::new(LineString::from(outer_points), vec![]);

    outer_poly.unsigned_area() as usize
}

fn main() {
    let plans = parse_stdin(std::io::stdin());
    dbg!(&plans);

    let p1 = part(&plans);
    println!("Part 1: {p1}");

    let p2_plans = plans
        .iter()
        .map(|plan| {
            let number = usize::from_str_radix(&plan.color[0..5], 16).unwrap();
            let dir = match plan.color.chars().last() {
                Some('0') => Direction::East,
                Some('1') => Direction::South,
                Some('2') => Direction::West,
                Some('3') => Direction::North,
                _ => panic!("uh oh"),
            };
            PlanEntry {
                number,
                dir,
                color: "".to_string(),
            }
        })
        .collect_vec();

    let p2 = part(&p2_plans);
    println!("Part 2: {p2}");
}
