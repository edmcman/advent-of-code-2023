use itertools::Itertools;

#[derive(Clone,Hash,Eq,PartialEq,PartialOrd,Ord)]
struct Board {
    board: Vec<Vec<char>>,
}

fn add_delta((x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
    let (x, y) = (x as isize + dx, y as isize + dy);

    if x < 0 || y < 0 {
        return None;
    }

    let (x, y) = (x as usize, y as usize);

    Some((x, y))
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

    fn tilt(&mut self, dx: isize, dy: isize) {
        loop {
            let mut changed = false;

            let coords = self.iter_coords().collect_vec();

            coords.into_iter().for_each(|(x, y)| {
                match self.get(x, y) {
                    Some('O') => {
                        match add_delta((x, y), (dx, dy)) {
                            Some((newx, newy)) => {
                                match self.get(newx, newy) {
                                    Some('O' | '#') => (),
                                    Some('.') => {
                                        // Empty space!
                                        *self.get_mut(x, y).unwrap() = '.';
                                        *self.get_mut(newx, newy).unwrap() = 'O';
                                        changed = true;
                                    }
                                    Some(_) => panic!("uhh"),
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    }
                    Some(_) => (),
                    None => panic!("uh oh"),
                }
            });

            if !changed {
                break;
            }
        }
    }

    fn load(&self) -> usize {
        (0..self.height())
            .map(|y| self.get_row(y).iter().filter(|&&c| c == 'O').count() * (y + 1))
            .sum()
    }

    fn cycle(&mut self) {
        [(0, 1), (-1, 0), (0, -1), (1, 0)]
            .iter()
            .for_each(|(dx, dy)| self.tilt(*dx, *dy));
    }

    fn cycles(&mut self, n: usize) {

        let mut m = std::collections::HashMap::<Self, usize>::new();

        for i in 1..n+1 {

            let orig = self.clone();

            match m.get(self) {
                Some(j) => {
                    let cycle_len = i - j;
                    let remaining = n - i;
                    let remaining_cycles = remaining / cycle_len;

                    let remaining_iterations = remaining - (remaining_cycles * cycle_len) + j + 1;
                    dbg!(&i, &j, &cycle_len, &remaining, &remaining_cycles, &remaining_iterations);

                    let the_board = m.into_iter().find(|(_, v)| *v == remaining_iterations).unwrap().0;

                    self.board = the_board.board;

                    return
                },
                None => ()
            }

            self.cycle();

            m.insert(orig, i);
        }
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

fn main() {
    let orig_b = Board::from_stdin(std::io::stdin());

    //dbg!(&b);

    let mut b = orig_b.clone();

    b.tilt(0, 1);

    dbg!(&b);

    let p1 = b.load();

    println!("p1: {p1}");

    let mut p2b = orig_b.clone();

    //p2b.cycles(11);
    p2b.cycles(1000000000);

    dbg!(&p2b);

    let p2 = p2b.load();

    println!("p2: {p2}");
}
