use itertools::Itertools;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<char>>,
}

#[derive(Debug)]
enum Symmetry {
    Vertical(usize),
    Horizontal(usize),
}

impl Symmetry {
    fn score(&self) -> usize {
        match self {
            Symmetry::Vertical(c) => *c,
            Symmetry::Horizontal(r) => 100 * r,
        }
    }
}

impl Board {
    fn get_row(&self, row: usize) -> &[char] {
        &self.board[self.board.len() - row]
    }

    fn get_col(&self, col: usize) -> Vec<char> {
        self.board.iter().map(|r| r[col]).collect_vec()
    }

    fn find_symmetry(&self, num_diff: usize) -> Option<Symmetry> {
        // First look for horizontal symmetry over rows.
        let hs = (0..self.board.len() - 1).find_map(|row| {
            let above = row + 1..self.board.len();
            let below = 0..row + 1;

            let above_len = above.end - above.start;
            let below_len = below.end - below.start;

            let min_len = above_len.min(below_len);

            let above_rows = &self.board[above.start..above.start + min_len];
            let below_rows = &self.board[above.start - min_len..above.start];

            //dbg!(row, min_len, above_rows, below_rows);

            let numdiff = above_rows
                .iter()
                .rev()
                .flatten()
                .zip(below_rows.iter().flatten())
                .filter(|(a, b)| a != b)
                .count();

            if numdiff == num_diff {
                println!("Below");
                below_rows
                    .iter()
                    .for_each(|r| println!("{}", (*r).iter().collect::<String>()));

                println!("Above");
                above_rows
                    .iter()
                    .for_each(|r| println!("{}", (*r).iter().collect::<String>()));

                Some(Symmetry::Horizontal(row + 1))
            } else {
                None
            }
        });

        let vs = (0..(self.board[0].len() - 1)).find_map(|col| {
            let right = col + 1..self.board[0].len();
            let left = 0..col + 1;

            let left_len = left.end - left.start;
            let right_len = right.end - right.start;

            let min_len = left_len.min(right_len);

            let left = col + 1 - min_len..col + 1;
            let right = col + 1..col + 1 + min_len;

            let left_cols = left.clone().map(|col| self.get_col(col)).collect_vec();
            let right_cols = right.clone().map(|col| self.get_col(col)).collect_vec();

            dbg!(col, min_len, &left, &right, &left_cols, &right_cols);

            /*
            println!("Left");
            left_cols
                //.clone()
                .iter()
                .rev()
                .for_each(|r| println!("{}", (*r).iter().collect::<String>()));

            println!("Right");
            right_cols
                //.clone()
                .iter()
                .for_each(|r| println!("{}", (*r).iter().collect::<String>()));
            */

            let numdiff = left_cols
                .iter()
                .rev()
                .flatten()
                .zip(right_cols.iter().flatten())
                .filter(|(a, b)| a != b)
                .count();

            if min_len > 0 && numdiff == num_diff {
                Some(Symmetry::Vertical(col + 1))
            } else {
                None
            }
        });

        hs.or(vs)
    }
}

fn parse(stdin: std::io::Stdin) -> Vec<Board> {
    let groups = stdin.lines().map(|l| l.unwrap()).group_by(|l| l.is_empty());

    let groups = groups
        .into_iter()
        .flat_map(|(k, g)| if !k { Some(g) } else { None });

    groups
        .map(|g| Board {
            board: g.map(|l| l.chars().collect_vec()).collect_vec(),
        })
        .collect_vec()
}

fn main() {
    println!("Hello, world!");

    let boards = parse(std::io::stdin());

    //dbg!(&boards);

    let p1 = boards
        .iter()
        .map(|b| b.find_symmetry(0).unwrap().score())
        .sum::<usize>();

    println!("p1: {p1}");

    let p2 = boards
        .iter()
        .map(|b| b.find_symmetry(1).unwrap().score())
        .sum::<usize>();

    println!("p2: {p2}");
}
