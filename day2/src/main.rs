use std::collections::HashMap;
use lazy_static::lazy_static;

type MarbleConfig = HashMap<Color, u32>;

lazy_static! {
    static ref MARBLES: MarbleConfig = {
        let mut m = HashMap::new();
        m.insert(Color::Red, 12);
        m.insert(Color::Green, 13);
        m.insert(Color::Blue, 14);
        m
    };
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn values() -> Vec<Color> {
        vec![Color::Red, Color::Green, Color::Blue]
    }

    fn from_string(s: &str) -> Result<Color, &'static str> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Invalid color"),
        }
    }
}

struct GameInfo {
    id: u32,
    obs: Vec<(u32, Color)>,
}

impl GameInfo {
    // Read a line, and convert to a vector of (u32, string) tuples
    fn from_string(s: &str) -> GameInfo {
        let mut split = s.splitn(3, " ");
        assert!(split.next() == Some("Game"));

        let id = split
            .next()
            .expect("Expected game id")
            .trim_end_matches(':')
            .parse::<u32>()
            .expect("Expected game id to be a number");

        let obs_str = split
            .next()
            .expect("observations")
            .split("; ")
            .map(|str| str.split(", "))
            .flatten();

        let obs = obs_str.map(
            |s| match s.splitn(2, " ").take(2).collect::<Vec<_>>().as_slice() {
                [num, color] => (
                    num.parse::<u32>().expect("Expected number"),
                    Color::from_string(color.trim_end_matches(':')).unwrap(),
                ),
                _ => panic!("Expected number and observation"),
            },
        );

        //obs.for_each(|s| println!("{:?}", s));

        GameInfo {
            id,
            obs: obs.collect(),
        }
    }

    fn is_possible(&self, marbles: &MarbleConfig) -> bool {
        self.obs.iter().all(|(num, color)| marbles.get(color) >= Some(num))
    }

    fn to_power(&self) -> u32 {
        Color::values().iter().map(|color| {
            let max = self.obs.iter().filter_map(|(num, c)| if c == color { Some(num) } else { None }).max().unwrap_or(&0);
            max
        }).product()

    }
}

fn main() {

    let games = std::io::stdin()
        .lines()
        .map(|s| s.unwrap());

    let games = games.map(|s| GameInfo::from_string(&s)).collect::<Vec<_>>();

    let game_powers_sum = games.iter().map(|g| g.to_power()).sum::<u32>();

    let possible_games = games.iter().filter(|g| g.is_possible(&MARBLES));

    let game_ids_sum = possible_games.map(|g| g.id).sum::<u32>();


    println!("Sum of possible game Ids: {game_ids_sum}");

    println!("Sum of game powers: {game_powers_sum}");
}
