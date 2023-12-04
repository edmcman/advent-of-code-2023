use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Card {
    id: u32,
    win: Vec<u32>,
    mine: Vec<u32>
}

impl Card {

    fn matches(&self) -> u32 {
        let win: HashSet<_> = self.win.iter().collect();
        let mine: HashSet<_> = self.mine.iter().collect();
        let num_winning = win.intersection(&mine).count() as u32;
        num_winning
    }

    fn score(&self) -> u32 {
        let num_winning = self.matches();
        if num_winning == 0 { 0 } else { 2_u32.pow(num_winning - 1) }
    }
}


// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn line_to_card (str: &str) -> Card {
    // Hack off Card n
    let split = str.splitn(2, ": ").collect::<Vec<&str>>();
    let id = split.iter().nth(0).unwrap().split_ascii_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
    
    let str = split.iter().nth(1).unwrap();
    
    // :-(  If I call .as_slice(), the split is a temporary and is freed right away.
    let split = str.splitn(2, '|').collect::<Vec<&str>>();

    let (first, second) = match split.as_slice() {
        [a,b] => (*a,*b),
        _ => panic!("Invalid card")
    };
    let win = first.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mine = second.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    Card{id, win, mine}
}

fn main() {
    let cards = std::io::stdin().lines().map(|l| line_to_card(&l.unwrap())).collect::<Vec<Card>>();

    //dbg!(&cards);

    println!("Card score sum: {}", cards.iter().map(|c| c.score()).sum::<u32>());

    // part 2...
    let mut queue = cards.iter().collect::<VecDeque<&Card>>();
    let mut fcards = Vec::from(queue.clone());

    while !queue.is_empty() {
        let card = queue.pop_front().unwrap();
        let matches = card.matches();

        //dbg!(card);

        // OK, we get copies of cards [card.id+1 .. card.id+matches]
        //dbg!("Adding cards to queue", card, matches);

        // I guess a better way to solve this would be to use memoization!  Oh well.

        ((card.id+1)..=(card.id+matches)).for_each(|id| {
            //println!("Adding copy of {id}");
            let card = cards.iter().find(|c| c.id == id).unwrap();
            //dbg!("Adding card", card);
            queue.push_back(card);
            fcards.push(card);
            
        });

    }

    println!("Final: {}", fcards.len());


}
