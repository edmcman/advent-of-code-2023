use lazy_static::lazy_static;

lazy_static! {
    static ref WORDS : Vec<&'static str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
}

fn main() {
    println!("Hello, world!");
    let lines = std::io::stdin().lines();

    let is_numba = |s: &&str| {

        // Is the first character a numeric digit?
        if s.chars().next().expect("empty string").is_digit(10) {
            Some(s.chars().next().expect("nonempty").to_digit(10).expect("number"))
        } else {
            // Is it a string representation of a number?
            WORDS.iter().enumerate().find_map(|(index, word)| if s.starts_with(word) { Some(index) } else { None }).map(|x| x as u32)
        }
    };

    let get_numbas_from_line = |line: &str| -> (u32, u32) {

        let substrs : Vec<&str> = (0..line.len()).map(|i| &line[i..]).collect();

        let first_numba = substrs.iter().find_map(is_numba).unwrap();
        let second_numba = substrs.iter().rev().find_map(is_numba).unwrap();
        println!("{} {}", first_numba, second_numba);
        (first_numba, second_numba)
    };
    let concat_tuple = |(a, b): (u32, u32)| 10*a + b;
    let sum = lines.map(|line| concat_tuple(get_numbas_from_line(&line.unwrap()))).sum::<u32>();
    println!("{}", sum);
}
