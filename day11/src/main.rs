struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&char> {
        self.grid.get(y).and_then(|row| row.get(x))
    }

    fn _get2(&self, x: usize, y: usize) -> Option<&char> {
        self.get((x, y))
    }

    fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
    }

    fn from_stdin(stdin: std::io::Stdin) -> Self {
        let grid: Vec<_> = stdin
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.chars().collect())
            .collect();

        let grid = grid.into_iter().rev().collect();
        Grid { grid }
    }

    fn expanded_rows(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                if row.iter().all(|&c| c == '.') {
                    Some(y)
                } else {
                    None
                }
            })
            .collect()
    }

    fn expanded_cols(&self) -> Vec<usize> {
        (0..self.width())
            .filter_map(|x| {
                if self.grid.iter().all(|row| row[x] == '.') {
                    Some(x)
                } else {
                    None
                }
            })
            .collect()
    }

    fn _expand(&mut self) {
        let expanded_rows = self.expanded_rows();

        let new_row = vec!['.'; self.width()];

        // Expand rows
        for row_index in expanded_rows.iter().rev() {
            self.grid.insert(*row_index, new_row.clone());
        }

        let expanded_cols = self.expanded_cols();

        // Expand columns
        for col_index in expanded_cols.iter().rev() {
            // Add to each row
            self.grid
                .iter_mut()
                .for_each(|row| row.insert(*col_index, '.'));
        }

        //dbg!(&expanded_rows, &expanded_cols);
    }

    fn go(&self, expansion: isize) -> isize {
        let expanded_rows = self.expanded_rows();
        let expanded_cols = self.expanded_cols();

        let galaxies = self
            .iter_coords()
            .filter_map(|(x, y)| match self.get((x, y)) {
                Some('#') => Some((x, y)),
                _ => None,
            })
            .collect::<Vec<_>>();

        let pair_iterator = galaxies.iter().enumerate().flat_map(|(i, &x)| {
            galaxies
                .iter()
                .enumerate()
                .filter(move |&(j, _)| i < j)
                .map(move |(_, &y)| (x, y))
        });

        let pair_distance = pair_iterator.map(|(from, to)| {
            let (fromx, fromy) = from;
            let (tox, toy) = to;

            let dx = ((fromx as isize) - (tox as isize)).abs();
            let dy = ((fromy as isize) - (toy as isize)).abs();

            // How many empty columns/rows do we cross?
            // Note: The start and the end are not empty because there is a galaxy there.

            let range_help = |a, b| {
                if a < b {
                    a..b
                } else {
                    b..a
                }
            };

            let empty_cols = (range_help(fromx, tox))
                .filter(|x| expanded_cols.contains(x))
                .count() as isize;
            let empty_rows = (range_help(fromy, toy))
                .filter(|y| expanded_rows.contains(y))
                .count() as isize;

            //dbg!(&(from,to), &dx, dy, empty_cols, empty_rows, expansion);

            dx + dy + (empty_cols + empty_rows) * (expansion-1)
        });

        pair_distance.sum::<isize>()
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().rev().for_each(|row| {
            writeln!(f, "{}", row.iter().cloned().collect::<String>())
                .expect("Failed to write to stdout");
        });
        Ok(())
    }
}

fn main() {
    let grid = Grid::from_stdin(std::io::stdin());

    //dbg!(&grid);

    // old part 1
    /*
    grid.expand();

    dbg!(&grid);

    let galaxies = grid
        .iter_coords()
        .filter_map(|(x, y)| match grid.get((x, y)) {
            Some('#') => Some((x, y)),
            _ => None,
        })
        .collect::<Vec<_>>();

    dbg!(&galaxies);
    */

    let p1 = grid.go(2);

    println!("Part 1: {p1}");

    let p2 = grid.go(1000000);

    println!("Part 2: {p2}");

}
