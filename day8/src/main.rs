//extern crate regex;
type Dir = char;

type Network = std::collections::HashMap<String, (String, String)>;

struct Info {
    directions: Vec<Dir>,
    network: Network,
}

fn parse(stdio: std::io::Stdin) -> Info {
    let mut lines = stdio.lines().map(|l| l.unwrap());
    let directions = lines.next().unwrap().chars().collect();

    let lines = lines.skip(1);

    let mut m = Network::new();

    let re = regex::Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();
    let captures = lines.map(|l| {
        re.captures(&l).unwrap().extract::<3>().1.map(|s| s.to_owned())
    });
    captures.for_each(|[k, v1, v2]| {
        m.insert(k, (v1, v2));
    });

    Info{directions, network: m}
}

fn part1(info: &Info, init_str: &str, p2: bool) -> usize {
    let directions = info.directions.iter().cycle();

    let init = info.network.get(init_str).unwrap();

    directions.scan(init, |state, dir| {
        let next_name = match dir {
            'L' => &state.0,
            'R' => &state.1,
            _ => panic!("Invalid direction"),
        };
        match next_name.as_str() {
            s if s.ends_with('Z') && p2 => None,
            "ZZZ" => None,
            _ => {
                let next = info.network.get(next_name).unwrap();
                *state = next;
                Some(next)
            }
        }
    }).count() + 1
}

type P2state<'a> = Vec<&'a (String, String)>;

fn part2(info: &Info) -> usize {
    let inits = info.network.iter().filter(|(k, _)| k.ends_with('A')).map(|(_k, v)| v).collect::<P2state>();

    //dbg!(&inits);

    let directions = info.directions.iter().cycle();

    directions.scan(inits, |state, dir| {
        let next_names = state.iter().map(|(v1, v2)| {
            match dir {
                'L' => v1,
                'R' => v2,
                _ => panic!("Invalid direction"),
            }            
        }).collect::<Vec<&String>>();

        /*if next_names.iter().any(|s| s.ends_with('Z')) {
            dbg!(&next_names);
        }*/

        if next_names.iter().all(|s| s.ends_with('Z')) { None }
        else {
            let next_state = next_names.iter().map(|n| info.network.get(*n).unwrap()).collect::<P2state>();
            *state = next_state;
            Some(())
        }
    }).count() + 1
}

/* Cool, my brute force approach didn't work.  Let's use lcm.  I hate mind-reading aspects of AoC.

   See also https://www.reddit.com/r/adventofcode/comments/18dfpub/2023_day_8_part_2_why_is_spoiler_correct/
*/
fn part2_f_aoc(info: &Info) -> usize {
    let inits = info.network.iter().filter(|(k, _)| k.ends_with('A')).map(|(k, _v)| k); //.collect::<P2state>();

    let t = inits.map(|s| part1(&info, s, true));

    t.reduce(|a, b| num::integer::lcm(a,b)).unwrap()

}

fn main() {
    let info = parse(std::io::stdin());

    println!("Part 1: {}", part1(&info, "AAA", false));
    println!("Part 2: {}", part2_f_aoc(&info));
}
