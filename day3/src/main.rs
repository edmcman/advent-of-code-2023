type Board = Vec<Vec<char>>;

fn get_board() -> Board {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

#[derive(Debug)]
struct Number {
    row: usize,
    firstcol: usize,
    lastcol: usize,
    number: u32,
}

fn is_symbol(c: &char) -> bool {
    if c.is_digit(10) {
        false
    } else {
        *c != '.'
    }
}

impl Number {
    fn has_adjacent_symbol(&self, board: &Board) -> bool {
        // Where are all the adjacent positions? We can just check all points in
        // the rectangle (row-1, firstcol-1) .. (row+1, lastcol+1) because a
        // number is not a symbol.
        let row = self.row as isize;
        let firstcol = self.firstcol as isize;
        let lastcol = self.lastcol as isize;

        println!(
            "Checking for adjacent symbol to {} {} {}:",
            row, firstcol, lastcol
        );

        ((row - 1)..(row + 2)).any(|row| {
            ((firstcol - 1)..(lastcol + 2)).any(|col| {
                //println!("Checking ({}, {})", row, col);
                if row < 0 || row >= board.len() as isize {
                    false
                } else if col < 0 || col >= board[row as usize].len() as isize {
                    false
                } else {
                    let symbol = board[row as usize][col as usize];
                    println!(
                        "Symbol at {} {}: {} {}",
                        row,
                        col,
                        symbol,
                        is_symbol(&symbol)
                    );
                    is_symbol(&symbol)
                }
            })
        })
    }

    fn is_adjacent_to(&self, (row, col): (usize, usize)) -> bool {
        let row_in = row as isize;
        let col_in = col as isize;

        let row = self.row as isize;
        let firstcol = self.firstcol as isize;
        let lastcol = self.lastcol as isize;

        (row - 1..row + 2)
            .any(|row| (firstcol - 1..lastcol + 2).any(|col| row == row_in && col == col_in))
    }
}

fn find_numbers(board: &Board) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (row, line) in board.iter().enumerate() {
        let mut current_number = String::new();

        // Finish the current number
        // XXX: Bleh.  This is ugly.  If I don't pass the arguments explicitly, I have to make finish_number a mutable closure, and then there are borrow conflicts!
        let finish_number =
            |startcol: usize, current_number: &mut String, numbers: &mut Vec<Number>| {
                if current_number.len() > 0 {
                    let number = current_number.parse::<u32>().unwrap();
                    numbers.push(Number {
                        row: row,
                        firstcol: startcol,
                        lastcol: startcol + current_number.len() - 1,
                        number: number,
                    });
                    current_number.clear();
                }
            };

        for (col, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                current_number.push(*c);
            } else {
                finish_number(
                    col - current_number.len(),
                    &mut current_number,
                    &mut numbers,
                );
            }
        }

        finish_number(
            line.len() - current_number.len(),
            &mut current_number,
            &mut numbers,
        );
    }

    numbers
}

fn find_stars(board: &Board) -> Vec<(usize, usize)> {
    let mut stars = Vec::new();

    for (row, line) in board.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '*' {
                stars.push((row, col));
            }
        }
    }

    stars
}

fn get_gear(numbers: &Vec<&Number>, (row, col): (usize, usize)) -> Option<u32> {
    let adjacent_numbers: Vec<_> = numbers
        .iter()
        .filter(|number| number.is_adjacent_to((row, col)))
        .collect();

    match adjacent_numbers.as_slice() {
        [n1, n2] => Some(n1.number * n2.number),
        [n] => None,
        [] => None,
        numbers => panic!("({}, {}) has more than 2 adjacent numbers: {:?}", row, col, numbers)
    }
}

fn main() {
    println!("Hello, world!");

    let board = get_board();

    let numbers = find_numbers(&board);

    let numbers_it = numbers
        .iter()
        .filter(|number| number.has_adjacent_symbol(&board));

        //.collect::<Vec<_>>();

    // We need this more than once...
    let numbers: Vec<_> = numbers_it.collect();

    //println!("Numbers: {:?}", numbers);

    let number_sum: u32 = numbers.iter().map(|n| n.number).sum();

    println!("Number sum: {}", number_sum);

    let stars = find_stars(&board);

    //println!("Stars: {:?}", stars);

    //let starred_gear: Vec<Option<u32>> = stars.iter().map(|star| get_gear(&numbers, *star)).collect();

    //println!("Starred gear: {:?}", starred_gear);

    println!("Starred gear sum: {}", stars.iter().filter_map(|star| get_gear(&numbers, *star)).sum::<u32>());
}
